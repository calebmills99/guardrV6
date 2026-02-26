use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, warn};

use super::BreachRecord;

const BREACH_DIR_URL: &str = "https://breachdirectory-breachdirectory-v1.p.rapidapi.com/";

#[derive(Debug, Deserialize)]
struct BdResponse {
    success: bool,
    found: u32,
    result: Option<Vec<BdResult>>,
}

#[derive(Debug, Deserialize)]
struct BdResult {
    sources: Option<Vec<String>>,
    has_password: Option<bool>,
}

pub async fn search_breaches(query: &str, query_type: &str, rapidapi_key: &str) -> Result<Vec<BreachRecord>> {
    let client = Client::new();

    let response = client
        .get(BREACH_DIR_URL)
        .header("X-RapidAPI-Key", rapidapi_key)
        .header("X-RapidAPI-Host", "breachdirectory-breachdirectory-v1.p.rapidapi.com")
        .query(&[("func", query_type), ("term", query)])
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            let body: serde_json::Value = response.json().await?;
            info!("BreachDirectory: Response for {} ({})", query, query_type);

            let mut records = Vec::new();
            if let Some(result) = body.get("result").and_then(|r| r.as_array()) {
                for entry in result {
                    let sources = entry
                        .get("sources")
                        .and_then(|s| s.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    for source in &sources {
                        records.push(BreachRecord {
                            source: "BreachDirectory".to_string(),
                            title: source.clone(),
                            date: None,
                            data_types: vec!["email".to_string()],
                            description: Some(format!("Found in {} via BreachDirectory", source)),
                        });
                    }
                }
            }

            Ok(records)
        }
        429 => {
            warn!("BreachDirectory: Rate limited");
            Err(anyhow::anyhow!("BreachDirectory rate limit exceeded"))
        }
        status => {
            warn!("BreachDirectory: Unexpected status {}", status);
            Err(anyhow::anyhow!("BreachDirectory returned status {}", status))
        }
    }
}
