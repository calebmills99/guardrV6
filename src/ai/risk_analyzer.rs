use anyhow::Result;
use reqwest::Client;
use tracing::{info, warn};

use super::{RiskAssessment, RiskFactor, RiskInputData};
use crate::osint;

/// Multi-source risk analysis aggregator
/// Combines OSINT data from all available sources into a unified risk score
pub fn calculate_comprehensive_risk(input: RiskInputData) -> RiskAssessment {
    let mut factors = Vec::new();
    let mut total_score: f32 = 0.0;
    let mut factor_count: f32 = 0.0;

    // Breach data factor (0-100)
    // In dating safety context: breaches = proof of real, long-lived digital identity
    // Zero breaches is MORE suspicious (potentially fake/new identity)
    let breach_score = match input.breach_count {
        0 => 65.0,   // No breaches = suspicious, could be a fabricated identity
        1..=2 => 35.0,  // Minimal presence
        3..=5 => 15.0,  // Normal person, been online a while
        6..=10 => 10.0, // Very established digital history
        _ => 8.0,       // Extremely long online presence — definitely a real person
    };
    factors.push(RiskFactor {
        category: "digital_history".to_string(),
        score: breach_score,
        description: if input.breach_count == 0 {
            "No breach history found — could indicate a new or fabricated identity".to_string()
        } else {
            format!(
                "Found in {} data breaches — confirms a real, established online presence",
                input.breach_count
            )
        },
        source: "HIBP + BreachDirectory".to_string(),
    });
    total_score += breach_score;
    factor_count += 1.0;

    // Digital footprint factor (0-100) — inverted: more platforms = lower risk
    if input.username_platforms_total > 0 {
        let presence_ratio = input.username_platforms_found as f32 / input.username_platforms_total as f32;
        let footprint_score = match input.username_platforms_found {
            0 => 85.0, // No presence = very suspicious
            1..=2 => 60.0,
            3..=5 => 30.0,
            _ => 15.0, // Extensive presence = likely real
        };
        factors.push(RiskFactor {
            category: "digital_footprint".to_string(),
            score: footprint_score,
            description: format!(
                "Found on {}/{} platforms ({:.0}% presence)",
                input.username_platforms_found,
                input.username_platforms_total,
                presence_ratio * 100.0
            ),
            source: "Username Search".to_string(),
        });
        total_score += footprint_score;
        factor_count += 1.0;
    }

    // Content moderation factor
    if input.moderation_flagged || input.moderation_score > 0.1 {
        let mod_score = (input.moderation_score * 100.0).min(100.0);
        factors.push(RiskFactor {
            category: "content_safety".to_string(),
            score: mod_score,
            description: if input.moderation_flagged {
                "Conversation flagged for harmful content".to_string()
            } else {
                format!("Content moderation score: {:.1}%", mod_score)
            },
            source: "OpenAI Moderation".to_string(),
        });
        total_score += mod_score;
        factor_count += 1.0;
    }

    // Deepfake detection factor
    if let Some(prob) = input.deepfake_probability {
        let df_score = (prob * 100.0).min(100.0);
        factors.push(RiskFactor {
            category: "photo_authenticity".to_string(),
            score: df_score,
            description: format!(
                "AI-generated/manipulated probability: {:.1}%",
                df_score
            ),
            source: "Reality Defender".to_string(),
        });
        total_score += df_score;
        factor_count += 1.0;
    }

    // Face search factor
    if let Some(matches) = input.face_matches {
        let face_score = if matches == 0 {
            70.0 // No matches could mean stolen/unique photo — moderate risk
        } else if matches <= 3 {
            20.0 // Some matches = likely real person
        } else {
            40.0 // Many matches could indicate stock photo
        };
        factors.push(RiskFactor {
            category: "reverse_image".to_string(),
            score: face_score,
            description: format!("Face found in {} locations online", matches),
            source: "FaceCheck.id".to_string(),
        });
        total_score += face_score;
        factor_count += 1.0;
    }

    // Network exposure factor
    if let Some(vulns) = input.shodan_vulns {
        if vulns > 0 {
            let vuln_score = ((vulns as f32) * 15.0).min(80.0);
            factors.push(RiskFactor {
                category: "network_exposure".to_string(),
                score: vuln_score,
                description: format!(
                    "{} known vulnerabilities, {} open ports",
                    vulns,
                    input.shodan_open_ports.unwrap_or(0)
                ),
                source: "Shodan".to_string(),
            });
            total_score += vuln_score;
            factor_count += 1.0;
        }
    }

    // Calculate overall
    let overall = if factor_count > 0.0 {
        total_score / factor_count
    } else {
        50.0 // Unknown
    };

    let risk_level = match overall as u32 {
        0..=25 => "LOW",
        26..=50 => "MEDIUM",
        51..=75 => "HIGH",
        _ => "CRITICAL",
    }
    .to_string();

    let confidence = (factor_count / 6.0).min(1.0); // Max 6 sources

    let recommendations = generate_recommendations(&factors, overall);

    let summary = format!(
        "Risk score {:.0}/100 ({}) based on {} data sources. {}",
        overall,
        risk_level,
        factors.len(),
        if overall > 50.0 {
            "Exercise caution with this profile."
        } else {
            "Profile appears relatively safe."
        }
    );

    info!(
        "Risk assessment: score={:.1}, level={}, confidence={:.2}, factors={}",
        overall, risk_level, confidence, factors.len()
    );

    RiskAssessment {
        overall_risk_score: overall,
        risk_level,
        confidence,
        factors,
        summary,
        recommendations,
    }
}

fn generate_recommendations(factors: &[RiskFactor], overall: f32) -> Vec<String> {
    let mut recs = Vec::new();

    for factor in factors {
        match factor.category.as_str() {
            "digital_history" if factor.score > 50.0 => {
                recs.push("No breach history found — this person may have a very new or fabricated online identity. Ask for additional verification.".to_string());
            }
            "digital_footprint" if factor.score > 60.0 => {
                recs.push("Very limited online presence detected. This could indicate a fake or newly created profile.".to_string());
            }
            "content_safety" if factor.score > 30.0 => {
                recs.push("Potentially harmful language detected in communications. Proceed with caution.".to_string());
            }
            "photo_authenticity" if factor.score > 40.0 => {
                recs.push("Profile photo may be AI-generated or manipulated. Request a live video call to verify.".to_string());
            }
            "reverse_image" if factor.score > 50.0 => {
                recs.push("Profile photo could not be verified across known sources. Consider a reverse image search.".to_string());
            }
            _ => {}
        }
    }

    if overall > 70.0 {
        recs.push("HIGH RISK: Meet only in public places and tell a trusted friend your plans.".to_string());
        recs.push("Consider reporting this profile to the dating platform.".to_string());
    } else if overall > 40.0 {
        recs.push("MODERATE RISK: Verify identity via video call before meeting in person.".to_string());
    }

    recs.push("Trust your instincts — if something feels off, it probably is.".to_string());

    recs
}

/// Call an LLM for narrative risk analysis
pub async fn llm_risk_analysis(
    prompt: &str,
    openai_key: Option<&str>,
    anthropic_key: Option<&str>,
) -> Result<String> {
    // Try OpenAI first, then Anthropic as fallback (Kallisto pattern)
    if let Some(key) = openai_key {
        match call_openai(prompt, key).await {
            Ok(response) => return Ok(response),
            Err(e) => warn!("OpenAI analysis failed, trying fallback: {}", e),
        }
    }

    if let Some(key) = anthropic_key {
        match call_anthropic(prompt, key).await {
            Ok(response) => return Ok(response),
            Err(e) => warn!("Anthropic analysis failed: {}", e),
        }
    }

    Err(anyhow::anyhow!("No LLM provider available for risk analysis"))
}

async fn call_openai(prompt: &str, api_key: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "system",
                    "content": "You are Guardr, an AI safety analyst for online dating. Analyze the provided OSINT data and produce a concise risk assessment. Be direct, specific, and actionable. Focus on safety implications for the user."
                },
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.3,
            "max_tokens": 1000
        }))
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;
    let content = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Analysis unavailable")
        .to_string();
    Ok(content)
}

async fn call_anthropic(prompt: &str, api_key: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 1000,
            "system": "You are Guardr, an AI safety analyst for online dating. Analyze the provided OSINT data and produce a concise risk assessment. Be direct, specific, and actionable.",
            "messages": [{"role": "user", "content": prompt}]
        }))
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;
    let content = body["content"][0]["text"]
        .as_str()
        .unwrap_or("Analysis unavailable")
        .to_string();
    Ok(content)
}
