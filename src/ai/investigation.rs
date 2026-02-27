use anyhow::Result;
use tracing::{info, warn};

use super::{InvestigationFinding, InvestigationReport};
use crate::config::OsintConfig;
use crate::osint;
use crate::ai::risk_analyzer;

/// PERA Investigation Cycle — ported from Kallisto-OSINTer's GuardrAgent
/// Plan → Execute → Review → Adjust
pub async fn investigate(
    email: Option<&str>,
    username: Option<&str>,
    name: Option<&str>,
    location: Option<&str>,
    image_url: Option<&str>,
    messages: Option<&[String]>,
    config: &OsintConfig,
) -> Result<InvestigationReport> {
    let objective = format!(
        "Investigate safety of dating profile: email={}, username={}, name={}, location={}",
        email.unwrap_or("N/A"),
        username.unwrap_or("N/A"),
        name.unwrap_or("N/A"),
        location.unwrap_or("N/A"),
    );

    info!("PERA Investigation started: {}", objective);

    let mut findings: Vec<InvestigationFinding> = Vec::new();
    let mut cycle = 0;
    let max_cycles = 2;

    // === CYCLE 1: Primary data gathering ===
    cycle += 1;
    info!("PERA Cycle {}: PLAN - Primary data sources", cycle);

    // EXECUTE: Breach checks
    if let Some(email) = email {
        // HIBP
        if let Some(ref key) = config.hibp_api_key {
            match osint::hibp::check_breaches(email, key).await {
                Ok(breaches) => {
                    findings.push(InvestigationFinding {
                        tool: "HIBP".to_string(),
                        query: email.to_string(),
                        success: true,
                        confidence: 0.95,
                        data: serde_json::json!({
                            "breach_count": breaches.len(),
                            "breaches": breaches
                        }),
                    });
                }
                Err(e) => {
                    warn!("HIBP check failed: {}", e);
                    findings.push(InvestigationFinding {
                        tool: "HIBP".to_string(),
                        query: email.to_string(),
                        success: false,
                        confidence: 0.0,
                        data: serde_json::json!({"error": e.to_string()}),
                    });
                }
            }
        }

        // BreachDirectory
        if let Some(ref key) = config.breach_directory_api_key {
            match osint::breach_directory::search_breaches(email, "auto", key).await {
                Ok(breaches) => {
                    findings.push(InvestigationFinding {
                        tool: "BreachDirectory".to_string(),
                        query: email.to_string(),
                        success: true,
                        confidence: 0.85,
                        data: serde_json::json!({
                            "breach_count": breaches.len(),
                            "breaches": breaches
                        }),
                    });
                }
                Err(e) => {
                    warn!("BreachDirectory check failed: {}", e);
                    findings.push(InvestigationFinding {
                        tool: "BreachDirectory".to_string(),
                        query: email.to_string(),
                        success: false,
                        confidence: 0.0,
                        data: serde_json::json!({"error": e.to_string()}),
                    });
                }
            }
        }

        // DNS lookup on email domain
        if let Some(domain) = osint::dns_lookup::domain_from_email(email) {
            match osint::dns_lookup::lookup_dns(&domain).await {
                Ok(records) => {
                    findings.push(InvestigationFinding {
                        tool: "DNS".to_string(),
                        query: domain,
                        success: true,
                        confidence: 0.7,
                        data: serde_json::to_value(&records).unwrap_or_default(),
                    });
                }
                Err(e) => warn!("DNS lookup failed: {}", e),
            }
        }
    }

    // EXECUTE: Username search
    if let Some(username) = username {
        let results = osint::username_search::search_username(username).await;
        let found_count = results.iter().filter(|r| r.found).count();

        findings.push(InvestigationFinding {
            tool: "UsernameSearch".to_string(),
            query: username.to_string(),
            success: true,
            confidence: 0.8,
            data: serde_json::json!({
                "platforms_checked": results.len(),
                "platforms_found": found_count,
                "results": results
            }),
        });
    }

    // EXECUTE: Content moderation on messages
    if let Some(msgs) = messages {
        if let Some(ref key) = config.openai_api_key {
            match crate::ai::moderation::moderate_conversation(msgs, key).await {
                Ok(result) => {
                    findings.push(InvestigationFinding {
                        tool: "OpenAI_Moderation".to_string(),
                        query: format!("{} messages", msgs.len()),
                        success: true,
                        confidence: 0.9,
                        data: serde_json::to_value(&result).unwrap_or_default(),
                    });
                }
                Err(e) => warn!("Moderation check failed: {}", e),
            }
        }
    }

    // REVIEW: Check what we have so far
    let successful = findings.iter().filter(|f| f.success).count();
    let confidence = if findings.is_empty() {
        0.0
    } else {
        successful as f32 / findings.len() as f32
    };

    info!(
        "PERA Cycle {} REVIEW: {}/{} successful, confidence={:.2}",
        cycle,
        successful,
        findings.len(),
        confidence
    );

    // === CYCLE 2: Deep dive (image analysis, network intel) ===
    if confidence < 0.9 || image_url.is_some() {
        cycle += 1;
        info!("PERA Cycle {}: PLAN - Deep dive analysis", cycle);

        // EXECUTE: Deepfake detection
        if let Some(img) = image_url {
            if let Some(ref key) = config.reality_defender_api_key {
                match osint::reality_defender::analyze_image(img, key).await {
                    Ok(result) => {
                        findings.push(InvestigationFinding {
                            tool: "RealityDefender".to_string(),
                            query: img.to_string(),
                            success: true,
                            confidence: 0.85,
                            data: serde_json::to_value(&result).unwrap_or_default(),
                        });
                    }
                    Err(e) => warn!("Reality Defender failed: {}", e),
                }
            }

            // Reverse image face search
            if let Some(ref key) = config.facecheck_api_key {
                match osint::facecheck::search_face(img, key).await {
                    Ok(result) => {
                        findings.push(InvestigationFinding {
                            tool: "FaceCheck".to_string(),
                            query: img.to_string(),
                            success: true,
                            confidence: 0.8,
                            data: serde_json::to_value(&result).unwrap_or_default(),
                        });
                    }
                    Err(e) => warn!("FaceCheck failed: {}", e),
                }
            }
        }

        // EXECUTE: Shodan (if we have an IP from DNS)
        if let Some(ref key) = config.shodan_api_key {
            let a_records: Vec<String> = findings
                .iter()
                .filter(|f| f.tool == "DNS" && f.success)
                .flat_map(|f| {
                    f.data
                        .get("a_records")
                        .and_then(|a| a.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                })
                .collect();

            for ip in a_records.iter().take(2) {
                match osint::shodan::lookup_host(ip, key).await {
                    Ok(intel) => {
                        findings.push(InvestigationFinding {
                            tool: "Shodan".to_string(),
                            query: ip.to_string(),
                            success: true,
                            confidence: 0.75,
                            data: serde_json::to_value(&intel).unwrap_or_default(),
                        });
                    }
                    Err(e) => warn!("Shodan lookup failed for {}: {}", ip, e),
                }
            }
        }
    }

    // FINAL REVIEW
    let total = findings.len() as u32;
    let successful = findings.iter().filter(|f| f.success).count() as u32;
    let final_confidence = if total > 0 {
        let avg: f32 = findings.iter().filter(|f| f.success).map(|f| f.confidence).sum::<f32>()
            / successful.max(1) as f32;
        avg
    } else {
        0.0
    };

    // Build risk assessment from findings
    let risk_assessment = build_risk_from_findings(&findings, config).await;

    info!(
        "PERA Investigation complete: {} cycles, {}/{} findings, confidence={:.2}",
        cycle, successful, total, final_confidence
    );

    Ok(InvestigationReport {
        objective,
        total_cycles: cycle,
        total_findings: total,
        successful_findings: successful,
        final_confidence,
        findings,
        risk_assessment: Some(risk_assessment),
    })
}

async fn build_risk_from_findings(
    findings: &[InvestigationFinding],
    config: &OsintConfig,
) -> super::RiskAssessment {
    // Extract metrics from findings
    let breach_count: u32 = findings
        .iter()
        .filter(|f| (f.tool == "HIBP" || f.tool == "BreachDirectory") && f.success)
        .map(|f| f.data.get("breach_count").and_then(|v| v.as_u64()).unwrap_or(0) as u32)
        .sum();

    let (username_found, username_total) = findings
        .iter()
        .find(|f| f.tool == "UsernameSearch" && f.success)
        .map(|f| {
            let found = f.data.get("platforms_found").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let total = f.data.get("platforms_checked").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            (found, total)
        })
        .unwrap_or((0, 0));

    let (mod_flagged, mod_score) = findings
        .iter()
        .find(|f| f.tool == "OpenAI_Moderation" && f.success)
        .map(|f| {
            let flagged = f.data.get("flagged").and_then(|v| v.as_bool()).unwrap_or(false);
            let score = f.data.get("overall_score").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
            (flagged, score)
        })
        .unwrap_or((false, 0.0));

    let deepfake_prob = findings
        .iter()
        .find(|f| f.tool == "RealityDefender" && f.success)
        .and_then(|f| {
            f.data
                .get("manipulation_probability")
                .and_then(|v| v.as_f64())
                .map(|v| v as f32)
        });

    let face_matches = findings
        .iter()
        .find(|f| f.tool == "FaceCheck" && f.success)
        .and_then(|f| f.data.get("matches_found").and_then(|v| v.as_u64()).map(|v| v as u32));

    let shodan_vulns = findings
        .iter()
        .find(|f| f.tool == "Shodan" && f.success)
        .and_then(|f| {
            f.data
                .get("vulns")
                .and_then(|v| v.as_array())
                .map(|a| a.len() as u32)
        });

    let shodan_ports = findings
        .iter()
        .find(|f| f.tool == "Shodan" && f.success)
        .and_then(|f| {
            f.data
                .get("open_ports")
                .and_then(|v| v.as_array())
                .map(|a| a.len() as u32)
        });

    risk_analyzer::calculate_comprehensive_risk(risk_analyzer::ComprehensiveRiskInput {
        breach_count,
        username_platforms_found: username_found,
        username_platforms_total: username_total,
        moderation_flagged: mod_flagged,
        moderation_score: mod_score,
        deepfake_probability: deepfake_prob,
        face_matches,
        shodan_vulns,
        shodan_open_ports: shodan_ports,
    })
}
