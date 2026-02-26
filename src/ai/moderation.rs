use anyhow::Result;
use reqwest::Client;
use tracing::{info, warn};

use super::{ModerationCategory, ModerationResult};

const OPENAI_MODERATION_URL: &str = "https://api.openai.com/v1/moderations";

/// Run text through OpenAI's free Moderation API
/// Detects: harassment, hate, self-harm, sexual, violence, and their subcategories
pub async fn moderate_text(text: &str, api_key: &str) -> Result<ModerationResult> {
    let client = Client::new();

    let response = client
        .post(OPENAI_MODERATION_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "input": text,
            "model": "omni-moderation-latest"
        }))
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            let body: serde_json::Value = response.json().await?;

            let result = &body["results"][0];
            let flagged = result["flagged"].as_bool().unwrap_or(false);

            let categories_obj = &result["categories"];
            let scores_obj = &result["category_scores"];

            let category_names = [
                "harassment",
                "harassment/threatening",
                "hate",
                "hate/threatening",
                "self-harm",
                "self-harm/intent",
                "self-harm/instructions",
                "sexual",
                "sexual/minors",
                "violence",
                "violence/graphic",
            ];

            let mut categories = Vec::new();
            let mut max_score: f32 = 0.0;

            for name in &category_names {
                let cat_flagged = categories_obj[name].as_bool().unwrap_or(false);
                let score = scores_obj[name].as_f64().unwrap_or(0.0) as f32;

                if score > max_score {
                    max_score = score;
                }

                if cat_flagged || score > 0.1 {
                    categories.push(ModerationCategory {
                        name: name.to_string(),
                        flagged: cat_flagged,
                        score,
                    });
                }
            }

            info!(
                "OpenAI Moderation: flagged={}, categories={}, max_score={:.3}",
                flagged,
                categories.len(),
                max_score
            );

            Ok(ModerationResult {
                flagged,
                categories,
                overall_score: max_score,
            })
        }
        status => {
            warn!("OpenAI Moderation: status {}", status);
            Err(anyhow::anyhow!("OpenAI Moderation returned status {}", status))
        }
    }
}

/// Moderate a batch of messages (e.g., a conversation)
pub async fn moderate_conversation(messages: &[String], api_key: &str) -> Result<ModerationResult> {
    let combined = messages.join("\n---\n");
    moderate_text(&combined, api_key).await
}
