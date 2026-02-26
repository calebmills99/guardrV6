use anyhow::Result;
use reqwest::Client;
use tracing::{info, warn};

use super::{FaceMatch, FaceSearchResult};

const FACECHECK_API_URL: &str = "https://facecheck.id/api/upload_pic";
const FACECHECK_RESULT_URL: &str = "https://facecheck.id/api/search";

/// Upload an image and initiate a face search
pub async fn search_face(image_url: &str, api_key: &str) -> Result<FaceSearchResult> {
    let client = Client::new();

    let response = client
        .post(FACECHECK_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "images": [image_url],
            "id_search": api_key
        }))
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            let body: serde_json::Value = response.json().await?;
            info!("FaceCheck: Search initiated for image");

            let id_search = body
                .get("id_search")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if id_search.is_empty() {
                return Ok(FaceSearchResult {
                    matches_found: 0,
                    matches: vec![],
                });
            }

            poll_results(&id_search, api_key).await
        }
        status => {
            warn!("FaceCheck: Upload returned status {}", status);
            Err(anyhow::anyhow!("FaceCheck returned status {}", status))
        }
    }
}

async fn poll_results(id_search: &str, api_key: &str) -> Result<FaceSearchResult> {
    let client = Client::new();

    for attempt in 0..10 {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        let response = client
            .post(FACECHECK_RESULT_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&serde_json::json!({
                "id_search": id_search
            }))
            .send()
            .await?;

        if response.status().as_u16() == 200 {
            let body: serde_json::Value = response.json().await?;

            if let Some(output) = body.get("output") {
                if let Some(items) = output.get("items").and_then(|i| i.as_array()) {
                    let matches: Vec<FaceMatch> = items
                        .iter()
                        .filter_map(|item| {
                            let url = item.get("url")?.as_str()?.to_string();
                            let score = item
                                .get("score")
                                .and_then(|s| s.as_f64())
                                .unwrap_or(0.0) as f32;
                            let source = item.get("base_url").and_then(|v| v.as_str()).map(String::from);
                            Some(FaceMatch { url, score, source })
                        })
                        .collect();

                    info!("FaceCheck: Found {} matches", matches.len());
                    return Ok(FaceSearchResult {
                        matches_found: matches.len() as u32,
                        matches,
                    });
                }
            }

            let status = body.get("status").and_then(|s| s.as_str()).unwrap_or("");
            if status == "error" {
                return Err(anyhow::anyhow!("FaceCheck search failed"));
            }
        }

        info!("FaceCheck: Polling attempt {} for search {}", attempt + 1, id_search);
    }

    Err(anyhow::anyhow!("FaceCheck search timed out"))
}
