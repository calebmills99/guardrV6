pub mod hibp;
pub mod breach_directory;
pub mod shodan;
pub mod facecheck;
pub mod reality_defender;
pub mod username_search;
pub mod dns_lookup;
pub mod ip_lookup;
pub mod phone_lookup;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsintResult {
    pub source: String,
    pub success: bool,
    pub confidence: f32,
    pub data: serde_json::Value,
    pub risk_contribution: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachRecord {
    pub source: String,
    pub title: String,
    pub date: Option<String>,
    pub data_types: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameResult {
    pub platform: String,
    pub url: String,
    pub found: bool,
    pub status_code: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpIntelligence {
    pub ip: String,
    pub open_ports: Vec<u16>,
    pub hostnames: Vec<String>,
    pub vulns: Vec<String>,
    pub os: Option<String>,
    pub org: Option<String>,
    pub isp: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecords {
    pub domain: String,
    pub a_records: Vec<String>,
    pub mx_records: Vec<String>,
    pub ns_records: Vec<String>,
    pub txt_records: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceSearchResult {
    pub matches_found: u32,
    pub matches: Vec<FaceMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceMatch {
    pub url: String,
    pub score: f32,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepfakeResult {
    pub is_manipulated: bool,
    pub manipulation_probability: f32,
    pub indicators: Vec<String>,
    pub media_type: String,
}
