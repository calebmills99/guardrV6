use anyhow::Result;
use reqwest::Client;
use tracing::{info, warn};

use super::{RiskAssessment, RiskCategory, RiskFactor};

fn breach_factor(breach_count: u32) -> RiskFactor {
    let score = match breach_count {
        0 => 65.0,
        1..=2 => 35.0,
        3..=5 => 15.0,
        6..=10 => 10.0,
        _ => 8.0,
    };
    let description = if breach_count == 0 {
        "No breach history found — could indicate a new or fabricated identity".to_string()
    } else {
        format!(
            "Found in {} data breaches — confirms a real, established online presence",
            breach_count
        )
    };
    RiskFactor {
        category: RiskCategory::DigitalHistory,
        score,
        description,
        source: "HIBP + BreachDirectory".to_string(),
    }
}

fn digital_footprint_factor(found: u32, total: u32) -> Option<RiskFactor> {
    if total == 0 {
        return None;
    }
    let presence_ratio = found as f32 / total as f32;
    let score = match found {
        0 => 85.0,
        1..=2 => 60.0,
        3..=5 => 30.0,
        _ => 15.0,
    };
    Some(RiskFactor {
        category: RiskCategory::DigitalFootprint,
        score,
        description: format!(
            "Found on {}/{} platforms ({:.0}% presence)",
            found,
            total,
            presence_ratio * 100.0
        ),
        source: "Username Search".to_string(),
    })
}

fn content_moderation_factor(flagged: bool, score: f32) -> Option<RiskFactor> {
    if !flagged && score <= 0.1 {
        return None;
    }
    let mod_score = (score * 100.0).min(100.0);
    Some(RiskFactor {
        category: RiskCategory::ContentSafety,
        score: mod_score,
        description: if flagged {
            "Conversation flagged for harmful content".to_string()
        } else {
            format!("Content moderation score: {:.1}%", mod_score)
        },
        source: "OpenAI Moderation".to_string(),
    })
}

fn deepfake_factor(prob: f32) -> RiskFactor {
    let score = (prob * 100.0).min(100.0);
    RiskFactor {
        category: RiskCategory::PhotoAuthenticity,
        score,
        description: format!("AI-generated/manipulated probability: {:.1}%", score),
        source: "Reality Defender".to_string(),
    }
}

fn face_search_factor(matches: u32) -> RiskFactor {
    let score = if matches == 0 {
        70.0
    } else if matches <= 3 {
        20.0
    } else {
        40.0
    };
    RiskFactor {
        category: RiskCategory::ReverseImage,
        score,
        description: format!("Face found in {} locations online", matches),
        source: "FaceCheck.id".to_string(),
    }
}

fn network_exposure_factor(vulns: Option<u32>, ports: Option<u32>) -> Option<RiskFactor> {
    let v = vulns.unwrap_or(0);
    if v == 0 {
        return None;
    }
    let score = ((v as f32) * 15.0).min(80.0);
    Some(RiskFactor {
        category: RiskCategory::NetworkExposure,
        score,
        description: format!(
            "{} known vulnerabilities, {} open ports",
            v,
            ports.unwrap_or(0)
        ),
        source: "Shodan".to_string(),
    })
}

fn risk_level_for_score(overall: f32) -> &'static str {
    match overall as u32 {
        0..=25 => "LOW",
        26..=50 => "MEDIUM",
        51..=75 => "HIGH",
        _ => "CRITICAL",
    }
}

pub fn calculate_comprehensive_risk(
    breach_count: u32,
    username_platforms_found: u32,
    username_platforms_total: u32,
    moderation_flagged: bool,
    moderation_score: f32,
    deepfake_probability: Option<f32>,
    face_matches: Option<u32>,
    shodan_vulns: Option<u32>,
    shodan_open_ports: Option<u32>,
    phone_risk: Option<(f32, String)>,
) -> RiskAssessment {
    let mut factors: Vec<RiskFactor> = Vec::new();

    factors.push(breach_factor(breach_count));

    if let Some(f) = digital_footprint_factor(username_platforms_found, username_platforms_total) {
        factors.push(f);
    }

    if let Some(f) = content_moderation_factor(moderation_flagged, moderation_score) {
        factors.push(f);
    }

    if let Some(prob) = deepfake_probability {
        factors.push(deepfake_factor(prob));
    }

    if let Some(matches) = face_matches {
        factors.push(face_search_factor(matches));
    }

    if let Some(f) = network_exposure_factor(shodan_vulns, shodan_open_ports) {
        factors.push(f);
    }

    if let Some((score, description)) = phone_risk {
        factors.push(RiskFactor {
            category: RiskCategory::PhoneRisk,
            score,
            description,
            source: "numverify".to_string(),
        });
    }

    let overall = if factors.is_empty() {
        50.0
    } else {
        factors.iter().map(|f| f.score).sum::<f32>() / factors.len() as f32
    };

    let risk_level = risk_level_for_score(overall).to_string();
    let confidence = (factors.len() as f32 / 6.0).min(1.0);
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
        match (factor.category, factor.score) {
            (RiskCategory::DigitalHistory, score) if score > 50.0 => {
                recs.push("No breach history found — this person may have a very new or fabricated online identity. Ask for additional verification.".to_string());
            }
            (RiskCategory::DigitalFootprint, score) if score > 60.0 => {
                recs.push("Very limited online presence detected. This could indicate a fake or newly created profile.".to_string());
            }
            (RiskCategory::ContentSafety, score) if score > 30.0 => {
                recs.push("Potentially harmful language detected in communications. Proceed with caution.".to_string());
            }
            (RiskCategory::PhotoAuthenticity, score) if score > 40.0 => {
                recs.push("Profile photo may be AI-generated or manipulated. Request a live video call to verify.".to_string());
            }
            (RiskCategory::ReverseImage, score) if score > 50.0 => {
                recs.push("Profile photo could not be verified across known sources. Consider a reverse image search.".to_string());
            }
            (RiskCategory::NetworkExposure, score) if score > 30.0 => {
                recs.push("Network infrastructure associated with this profile has known vulnerabilities.".to_string());
            }
            (RiskCategory::PhoneRisk, score) if score > 50.0 => {
                recs.push("Phone number appears to be VoIP or disposable — commonly used by scammers. Ask for a verified mobile number.".to_string());
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

pub async fn llm_risk_analysis(
    prompt: &str,
    openai_key: Option<&str>,
    anthropic_key: Option<&str>,
) -> Result<String> {
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
    let client = crate::http_client::build_default_client();
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
    let client = crate::http_client::build_default_client();
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
