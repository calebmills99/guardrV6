use anyhow::Result;
use reqwest::Client;
use tracing::info;

use super::DnsRecords;

const DNS_API_URL: &str = "https://dns.google/resolve";

/// DNS record lookup using Google's public DNS-over-HTTPS API
/// Ported from Kallisto-OSINTer's dns_lookup module
pub async fn lookup_dns(domain: &str) -> Result<DnsRecords> {
    let client = super::build_http_client();
    let mut records = DnsRecords {
        domain: domain.to_string(),
        a_records: vec![],
        mx_records: vec![],
        ns_records: vec![],
        txt_records: vec![],
    };

    // A records (type 1)
    if let Ok(a) = query_dns(&client, domain, 1).await {
        records.a_records = a;
    }

    // MX records (type 15)
    if let Ok(mx) = query_dns(&client, domain, 15).await {
        records.mx_records = mx;
    }

    // NS records (type 2)
    if let Ok(ns) = query_dns(&client, domain, 2).await {
        records.ns_records = ns;
    }

    // TXT records (type 16)
    if let Ok(txt) = query_dns(&client, domain, 16).await {
        records.txt_records = txt;
    }

    info!(
        "DNS lookup for {}: {} A, {} MX, {} NS, {} TXT records",
        domain,
        records.a_records.len(),
        records.mx_records.len(),
        records.ns_records.len(),
        records.txt_records.len()
    );

    Ok(records)
}

async fn query_dns(client: &Client, domain: &str, record_type: u16) -> Result<Vec<String>> {
    let response = client
        .get(DNS_API_URL)
        .query(&[("name", domain), ("type", &record_type.to_string())])
        .header("Accept", "application/dns-json")
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;

    let records: Vec<String> = body
        .get("Answer")
        .and_then(|a| a.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|entry| {
                    entry.get("data").and_then(|d| d.as_str()).map(|s| s.trim_matches('"').to_string())
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(records)
}

/// Extract domain from an email address
pub fn domain_from_email(email: &str) -> Option<String> {
    email.split('@').nth(1).map(|s| s.to_string())
}
