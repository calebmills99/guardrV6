use anyhow::Result;
use tracing::{info, warn};

use super::IpIntelligence;

const SHODAN_API_URL: &str = "https://api.shodan.io";
const INTERNETDB_URL: &str = "https://internetdb.shodan.io";

/// Free InternetDB lookup (no API key required)
pub async fn lookup_internetdb(ip: &str) -> Result<IpIntelligence> {
    let client = super::build_http_client();
    let url = format!("{}/{}", INTERNETDB_URL, ip);

    let response = client.get(&url).send().await?;

    match response.status().as_u16() {
        200 => {
            let body: serde_json::Value = response.json().await?;
            info!("Shodan InternetDB: Found data for {}", ip);

            Ok(IpIntelligence {
                ip: ip.to_string(),
                open_ports: body
                    .get("ports")
                    .and_then(|p| p.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as u16)).collect())
                    .unwrap_or_default(),
                hostnames: body
                    .get("hostnames")
                    .and_then(|h| h.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default(),
                vulns: body
                    .get("vulns")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default(),
                os: None,
                org: None,
                isp: None,
                country: None,
                city: None,
            })
        }
        404 => {
            info!("Shodan InternetDB: No data for {}", ip);
            Ok(IpIntelligence {
                ip: ip.to_string(),
                open_ports: vec![],
                hostnames: vec![],
                vulns: vec![],
                os: None,
                org: None,
                isp: None,
                country: None,
                city: None,
            })
        }
        status => Err(anyhow::anyhow!("Shodan InternetDB returned status {}", status)),
    }
}

/// Full Shodan host lookup (requires API key)
pub async fn lookup_host(ip: &str, api_key: &str) -> Result<IpIntelligence> {
    let client = super::build_http_client();
    let url = format!("{}/shodan/host/{}", SHODAN_API_URL, ip);

    let response = client
        .get(&url)
        .query(&[("key", api_key)])
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            let body: serde_json::Value = response.json().await?;
            info!("Shodan: Full host data for {}", ip);

            Ok(IpIntelligence {
                ip: ip.to_string(),
                open_ports: body
                    .get("ports")
                    .and_then(|p| p.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as u16)).collect())
                    .unwrap_or_default(),
                hostnames: body
                    .get("hostnames")
                    .and_then(|h| h.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default(),
                vulns: body
                    .get("vulns")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default(),
                os: body.get("os").and_then(|v| v.as_str()).map(String::from),
                org: body.get("org").and_then(|v| v.as_str()).map(String::from),
                isp: body.get("isp").and_then(|v| v.as_str()).map(String::from),
                country: body.get("country_name").and_then(|v| v.as_str()).map(String::from),
                city: body.get("city").and_then(|v| v.as_str()).map(String::from),
            })
        }
        401 => Err(anyhow::anyhow!("Shodan: Invalid API key")),
        status => {
            warn!("Shodan: Unexpected status {}", status);
            Err(anyhow::anyhow!("Shodan returned status {}", status))
        }
    }
}

/// Search Shodan for a query string
pub async fn search(query: &str, api_key: &str) -> Result<serde_json::Value> {
    let client = super::build_http_client();
    let url = format!("{}/shodan/host/search", SHODAN_API_URL);

    let response = client
        .get(&url)
        .query(&[("key", api_key), ("query", query)])
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;
    Ok(body)
}
