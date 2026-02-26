use anyhow::Result;
use reqwest::Client;
use tracing::info;

use super::IpIntelligence;

const RDAP_URL: &str = "https://rdap.org/ip";
const IP_API_URL: &str = "https://ipapi.co";

/// IP geolocation and org lookup via ipapi.co (free, no key needed, 1000/day)
pub async fn lookup_ip(ip: &str) -> Result<IpIntelligence> {
    let client = crate::http_client::build_default_client();
    let url = format!("{}/{}/json/", IP_API_URL, ip);

    let response = client
        .get(&url)
        .header("User-Agent", "Guardr-Safety-Platform")
        .send()
        .await?;

    match response.status().as_u16() {
        200 => {
            let body: serde_json::Value = response.json().await?;

            if body.get("error").and_then(|e| e.as_bool()).unwrap_or(false) {
                return Err(anyhow::anyhow!("IP lookup failed: reserved or invalid IP"));
            }

            info!("IP lookup: Data retrieved for {}", ip);

            Ok(IpIntelligence {
                ip: ip.to_string(),
                open_ports: vec![],
                hostnames: vec![],
                vulns: vec![],
                os: None,
                org: body.get("org").and_then(|v| v.as_str()).map(String::from),
                isp: body.get("org").and_then(|v| v.as_str()).map(String::from),
                country: body.get("country_name").and_then(|v| v.as_str()).map(String::from),
                city: body.get("city").and_then(|v| v.as_str()).map(String::from),
            })
        }
        429 => Err(anyhow::anyhow!("IP lookup: Rate limit exceeded")),
        status => Err(anyhow::anyhow!("IP lookup returned status {}", status)),
    }
}

/// RDAP lookup for IP registration data (from Kallisto-OSINTer)
pub async fn lookup_rdap(ip: &str) -> Result<serde_json::Value> {
    let client = crate::http_client::build_default_client();
    let url = format!("{}/{}", RDAP_URL, ip);

    let response = client
        .get(&url)
        .header("Accept", "application/rdap+json")
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;
    info!("RDAP lookup complete for {}", ip);
    Ok(body)
}
