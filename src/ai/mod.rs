pub mod moderation;
pub mod risk_analyzer;
pub mod investigation;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResult {
    pub flagged: bool,
    pub categories: Vec<ModerationCategory>,
    pub overall_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationCategory {
    pub name: String,
    pub flagged: bool,
    pub score: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskCategory {
    DigitalHistory,
    DigitalFootprint,
    ContentSafety,
    PhotoAuthenticity,
    ReverseImage,
    NetworkExposure,
}

impl std::fmt::Display for RiskCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RiskCategory::DigitalHistory => "digital_history",
            RiskCategory::DigitalFootprint => "digital_footprint",
            RiskCategory::ContentSafety => "content_safety",
            RiskCategory::PhotoAuthenticity => "photo_authenticity",
            RiskCategory::ReverseImage => "reverse_image",
            RiskCategory::NetworkExposure => "network_exposure",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk_score: f32,
    pub risk_level: String,
    pub confidence: f32,
    pub factors: Vec<RiskFactor>,
    pub summary: String,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub category: RiskCategory,
    pub score: f32,
    pub description: String,
    pub source: String,
}

/// PERA cycle state from Kallisto-OSINTer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigationReport {
    pub objective: String,
    pub total_cycles: u32,
    pub total_findings: u32,
    pub successful_findings: u32,
    pub final_confidence: f32,
    pub findings: Vec<InvestigationFinding>,
    pub risk_assessment: Option<RiskAssessment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestigationFinding {
    pub tool: String,
    pub query: String,
    pub success: bool,
    pub confidence: f32,
    pub data: serde_json::Value,
}
