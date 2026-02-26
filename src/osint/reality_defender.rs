use anyhow::Result;
use tracing::{info, warn};

use super::DeepfakeResult;

const REALITY_DEFENDER_URL: &str = "https://api.realitydefender.com/v2";

/// Analyze an image for deepfake / AI-generated content
pub async fn analyze_image(image_url: &str, api_key: &str) -> Result<DeepfakeResult> {
    let client = super::build_http_client();

    let response = client
        .post(&format!("{}/detect/image", REALITY_DEFENDER_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "url": image_url,
            "models": ["all"]
        }))
        .send()
        .await?;

    match response.status().as_u16() {
        200 | 201 => {
            let body: serde_json::Value = response.json().await?;
            info!("Reality Defender: Image analysis complete");

            let probability = body
                .get("probability")
                .or_else(|| body.get("score"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as f32;

            let is_manipulated = probability > 0.5;

            let indicators: Vec<String> = body
                .get("indicators")
                .and_then(|i| i.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            Ok(DeepfakeResult {
                is_manipulated,
                manipulation_probability: probability,
                indicators,
                media_type: "image".to_string(),
            })
        }
        401 => Err(anyhow::anyhow!("Reality Defender: Invalid API key")),
        429 => Err(anyhow::anyhow!("Reality Defender: Rate limit exceeded")),
        status => {
            warn!("Reality Defender: Unexpected status {}", status);
            let body = response.text().await.unwrap_or_default();
            Err(anyhow::anyhow!(
                "Reality Defender returned status {}: {}",
                status,
                body
            ))
        }
    }
}

/// Analyze a video for deepfake content
pub async fn analyze_video(video_url: &str, api_key: &str) -> Result<DeepfakeResult> {
    let client = super::build_http_client();

    let response = client
        .post(&format!("{}/detect/video", REALITY_DEFENDER_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "url": video_url,
            "models": ["all"]
        }))
        .send()
        .await?;

    match response.status().as_u16() {
        200 | 201 => {
            let body: serde_json::Value = response.json().await?;
            let probability = body
                .get("probability")
                .or_else(|| body.get("score"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as f32;

            Ok(DeepfakeResult {
                is_manipulated: probability > 0.5,
                manipulation_probability: probability,
                indicators: vec![],
                media_type: "video".to_string(),
            })
        }
        status => {
            warn!("Reality Defender video: status {}", status);
            Err(anyhow::anyhow!("Reality Defender returned status {}", status))
        }
    }
}
