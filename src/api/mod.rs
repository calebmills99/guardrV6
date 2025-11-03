// Guardr API Module Structure
// This defines the web API endpoints for the Rust backend

use axum::{
    routing::{get, post, put, delete},
    Router,
    http::StatusCode,
    middleware,
};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
};
use std::time::Duration;

pub mod auth;
pub mod security;
pub mod reports;
pub mod users;
pub mod dating;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

// Main API router
pub fn create_router() -> Router<crate::state::AppState> {
    Router::new()
        // Health check (no auth required)
        .route("/health", get(health_check))
        .route("/", get(root_info))

        // Public demo endpoint (no auth required)
        // Note: Ingress strips /api prefix, so /api/check becomes /check
        .route("/check", post(demo_check))

        // Authentication routes (no auth required)
        .route("/v1/auth/register", post(auth::register))
        .route("/v1/auth/login", post(auth::login))
        .route("/v1/auth/refresh", post(auth::refresh_token))
        .route("/v1/auth/logout", post(auth::logout))

        // User management (auth required)
        .route("/v1/user/profile", get(users::get_profile))
        .route("/v1/user/profile", put(users::update_profile))
        .route("/v1/user/api-keys", get(users::list_api_keys))
        .route("/v1/user/api-keys", post(users::create_api_key))
        .route("/v1/user/api-keys/:key_id", delete(users::revoke_api_key))

        // Security analysis endpoints (auth required)
        .route("/v1/security/check-breach", post(security::check_breach))
        .route("/v1/security/check-password", post(security::check_password_strength))
        .route("/v1/security/risk-score", post(security::calculate_risk_score))
        .route("/v1/security/bulk-check", post(security::bulk_security_check))
        .route("/v1/security/filter-data", post(security::filter_data))

        // Dating safety endpoints (auth required)
        .route("/v1/dating/analyze-conversation", post(dating::analyze_conversation))
        .route("/v1/dating/verify-claims", post(dating::verify_identity_claims))
        .route("/v1/dating/safety-report", post(dating::generate_safety_report))

        // Reports and history (auth required)
        .route("/v1/reports", get(reports::list_reports))
        .route("/v1/reports/:report_id", get(reports::get_report))
        .route("/v1/reports/:report_id", delete(reports::delete_report))
        .route("/v1/reports/export", get(reports::export_reports))

        // Admin endpoints (admin auth required)
        .route("/v1/admin/breach-sources", get(reports::admin::list_breach_sources))
        .route("/v1/admin/breach-sources", post(reports::admin::add_breach_source))
        .route("/v1/admin/update-breach-data", post(reports::admin::update_breach_data))
        
        // Add middleware layers
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

use axum::{extract::State, Json};
use crate::state::AppState;

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Guardr API is healthy".to_string()),
        error: None,
        message: Some("Service operational".to_string()),
        timestamp: chrono::Utc::now(),
    })
}

async fn root_info() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "name": "Guardr API",
            "version": "1.0.0",
            "description": "Personal Safety Platform API",
            "documentation": "https://api.guardr.app/docs",
            "status": "operational"
        })),
        error: None,
        message: Some("Welcome to Guardr API".to_string()),
        timestamp: chrono::Utc::now(),
    })
}

// Public demo check endpoint
#[derive(Debug, Deserialize)]
struct DemoCheckRequest {
    name: String,
    location: Option<String>,
}

#[derive(Debug, Serialize)]
struct SafetyTip {
    category: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct DemoCheckResponse {
    name: String,
    risk_level: String,
    risk_score: u32,
    person_verification: String,
    recommendations: Vec<String>,
    safety_tips: Vec<SafetyTip>,
}

async fn demo_check(
    State(_state): State<AppState>,
    Json(payload): Json<DemoCheckRequest>,
) -> Json<DemoCheckResponse> {
    // This is a demo/mock endpoint for the public website
    // In production, you'd call real OSINT services here

    let name = payload.name.clone();
    let location_str = payload.location.as_ref().map(|l| format!(" in {}", l)).unwrap_or_default();

    // Mock risk assessment based on name length (for demo purposes)
    let risk_score = ((name.len() * 7) % 100) as u32;
    let risk_level = if risk_score >= 70 {
        "LOW"
    } else if risk_score >= 40 {
        "MEDIUM"
    } else {
        "HIGH"
    }.to_string();

    let person_verification = format!(
        "Demo verification for {}{}: This is a demonstration of Guardr's OSINT capabilities. \
        In production, we would check 40+ databases, scan for breaches, verify identity claims, \
        and analyze behavioral patterns. Real analysis takes 60-120 seconds.",
        name, location_str
    );

    let recommendations = vec![
        "Video call before meeting in person to verify identity".to_string(),
        "Meet in a public place for your first meeting".to_string(),
        "Tell a trusted friend where you're going".to_string(),
        "Trust your instincts - if something feels off, it probably is".to_string(),
    ];

    let safety_tips = vec![
        SafetyTip {
            category: "Smart Habits".to_string(),
            message: "Always meet in public places for first dates".to_string(),
        },
        SafetyTip {
            category: "Did You Know?".to_string(),
            message: "69% of LGBTQ+ individuals experience harassment in online dating".to_string(),
        },
        SafetyTip {
            category: "You Decide".to_string(),
            message: "You are an adult. Use Guardr's insights to make informed decisions about your safety.".to_string(),
        },
    ];

    Json(DemoCheckResponse {
        name,
        risk_level,
        risk_score,
        person_verification,
        recommendations,
        safety_tips,
    })
}

// Request/Response models for each endpoint category

// Breach checking models
#[derive(Debug, Serialize, Deserialize)]
pub struct BreachCheckRequest {
    pub email: String,
    pub include_details: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreachCheckResponse {
    pub email: String,
    pub is_breached: bool,
    pub breach_count: u32,
    pub risk_score: u8,
    pub breaches: Option<Vec<BreachDetail>>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreachDetail {
    pub source: String,
    pub breach_date: String,
    pub data_types: Vec<String>,
    pub severity: String,
}

// Password checking models
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCheckRequest {
    pub password: String,
    pub email: Option<String>, // for personalized checks
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCheckResponse {
    pub is_weak: bool,
    pub strength_score: u8,
    pub is_breached: bool,
    pub recommendations: Vec<String>,
    pub entropy: f64,
}

// Dating safety models
#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationAnalysisRequest {
    pub messages: Vec<Message>,
    pub participant_info: Option<ParticipantClaims>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub timestamp: String,
    pub sender: String, // "user" or "match"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantClaims {
    pub name: Option<String>,
    pub age: Option<u8>,
    pub location: Option<String>,
    pub occupation: Option<String>,
    pub education: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SafetyAnalysisResponse {
    pub overall_risk_score: f32,
    pub risk_indicators: Vec<RiskIndicator>,
    pub identity_consistency: IdentityConsistency,
    pub conversation_patterns: ConversationPatterns,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskIndicator {
    pub indicator_type: String,
    pub confidence: f32,
    pub description: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityConsistency {
    pub consistency_score: f32,
    pub inconsistencies: Vec<String>,
    pub verified_claims: u32,
    pub unverified_claims: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationPatterns {
    pub love_bombing_score: f32,
    pub pressure_indicators: u32,
    pub red_flag_count: u32,
    pub response_time_analysis: ResponseTimeAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTimeAnalysis {
    pub avg_response_time: f32,
    pub suspicious_patterns: Vec<String>,
}

// Authentication models
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user: UserProfile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub subscription_tier: String,
    pub usage_stats: UsageStats,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageStats {
    pub monthly_queries: u32,
    pub queries_used: u32,
    pub queries_remaining: u32,
    pub reset_date: String,
}