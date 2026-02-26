use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, warn};

use super::BreachRecord;

const HIBP_BASE_URL: &str = "https://haveibeenpwned.com/api/v3";
const HIBP_PASSWORDS_URL: &str = "https://api.pwnedpasswords.com";

#[derive(Debug, Deserialize)]
struct HibpBreach {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "BreachDate")]
    breach_date: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "DataClasses")]
    data_classes: Vec<String>,
}

pub async fn check_breaches(email: &str, api_key: &str) -> Result<Vec<BreachRecord>> {
    let client = crate::http_client::build_default_client();
    let url = format!("{}/breachedaccount/{}", HIBP_BASE_URL, email);

    let response = client
        .get(&url)
        .header("hibp-api-key", api_key)
        .header("User-Agent", "Guardr-Safety-Platform")
        .query(&[("truncateResponse", "false")])
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            let breaches: Vec<HibpBreach> = response.json().await?;
            info!("HIBP: Found {} breaches for {}", breaches.len(), email);
            Ok(breaches
                .into_iter()
                .map(|b| BreachRecord {
                    source: "HIBP".to_string(),
                    title: b.title,
                    date: Some(b.breach_date),
                    data_types: b.data_classes,
                    description: Some(b.description),
                })
                .collect())
        }
        404 => {
            info!("HIBP: No breaches found for {}", email);
            Ok(vec![])
        }
        429 => {
            warn!("HIBP: Rate limited");
            Err(anyhow::anyhow!("HIBP rate limit exceeded"))
        }
        status => {
            warn!("HIBP: Unexpected status {}", status);
            Err(anyhow::anyhow!("HIBP returned status {}", status))
        }
    }
}

pub async fn check_password(password_sha1_prefix: &str) -> Result<Vec<(String, u64)>> {
    let client = crate::http_client::build_default_client();
    let url = format!("{}/range/{}", HIBP_PASSWORDS_URL, password_sha1_prefix);

    let response = client
        .get(&url)
        .header("User-Agent", "Guardr-Safety-Platform")
        .send()
        .await?;

    let body = response.text().await?;
    let results: Vec<(String, u64)> = body
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].trim().parse().unwrap_or(0)))
            } else {
                None
            }
        })
        .collect();

    Ok(results)
}
