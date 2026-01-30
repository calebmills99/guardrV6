use axum::{extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::info;
use validator::Validate;

use crate::auth::AuthenticatedUser;
use crate::errors::AppError;
use crate::state::AppState;
use crate::weak_pass;
use crate::risk_score;
use crate::filter;
use crate::filtermain;

// Request/Response models
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordCheckRequest {
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PasswordCheckResponse {
    pub is_weak: bool,
    pub strength_score: u8,
    pub is_breached: bool,
    pub recommendations: Vec<String>,
    pub entropy: f64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BreachCheckRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub include_details: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BreachCheckResponse {
    pub email: String,
    pub is_breached: bool,
    pub breach_count: u32,
    pub risk_score: u8,
    pub breaches: Option<Vec<BreachDetail>>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BreachDetail {
    pub source: String,
    pub breach_date: String,
    pub data_types: Vec<String>,
    pub severity: String,
}

#[derive(Debug, Deserialize)]
pub struct RiskScoreRequest {
    pub email: String,
    pub password: Option<String>,
    pub additional_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct RiskScoreResponse {
    pub risk_score: u8,
    pub email: String,
    pub factors: Vec<RiskFactor>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RiskFactor {
    pub factor_type: String,
    pub description: String,
    pub score_impact: u8,
    pub severity: String,
}

#[derive(Debug, Deserialize)]
pub struct BulkSecurityCheckRequest {
    pub emails: Vec<String>,
    pub passwords: Option<Vec<String>>,
    pub include_details: bool,
}

#[derive(Debug, Serialize)]
pub struct BulkSecurityCheckResponse {
    pub results: Vec<BulkCheckResult>,
    pub summary: BulkCheckSummary,
}

#[derive(Debug, Serialize)]
pub struct BulkCheckResult {
    pub email: String,
    pub is_breached: bool,
    pub risk_score: u8,
    pub password_weak: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BulkCheckSummary {
    pub total_checked: u32,
    pub breached_count: u32,
    pub high_risk_count: u32,
    pub weak_passwords: u32,
}

#[derive(Debug, Deserialize)]
pub struct DataFilterRequest {
    pub data: serde_json::Value,
    pub filter_type: String, // "basic" or "advanced"
}

#[derive(Debug, Serialize)]
pub struct DataFilterResponse {
    pub filtered_data: serde_json::Value,
    pub removed_entries: u32,
    pub filter_applied: String,
}

// Password strength calculation
fn calculate_password_strength(password: &str) -> u8 {
    let mut score = 0u8;
    
    // Length scoring
    if password.len() >= 8 { score += 20; }
    if password.len() >= 12 { score += 10; }
    if password.len() >= 16 { score += 10; }
    
    // Character variety
    if password.chars().any(|c| c.is_lowercase()) { score += 10; }
    if password.chars().any(|c| c.is_uppercase()) { score += 10; }
    if password.chars().any(|c| c.is_ascii_digit()) { score += 10; }
    if password.chars().any(|c| !c.is_alphanumeric()) { score += 15; }
    
    // Pattern checks (deduct points for common patterns)
    if is_common_pattern(password) { score = score.saturating_sub(20); }
    
    score.min(100)
}

fn is_common_pattern(password: &str) -> bool {
    let lower = password.to_lowercase();
    
    // Common patterns
    let patterns = [
        "123456", "password", "qwerty", "abc", "111", "000",
        "admin", "user", "login", "welcome", "monkey"
    ];
    
    patterns.iter().any(|&pattern| lower.contains(pattern))
}

fn calculate_password_entropy(password: &str) -> f64 {
    let mut charset_size = 0;
    
    if password.chars().any(|c| c.is_lowercase()) { charset_size += 26; }
    if password.chars().any(|c| c.is_uppercase()) { charset_size += 26; }
    if password.chars().any(|c| c.is_ascii_digit()) { charset_size += 10; }
    if password.chars().any(|c| !c.is_alphanumeric()) { charset_size += 32; }
    
    (password.len() as f64) * (charset_size as f64).log2()
}

fn hash_email(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(email.to_lowercase().as_bytes());
    hex::encode(hasher.finalize())
}

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hex::encode(hasher.finalize())
}

// API Handlers
pub async fn check_password_strength(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<PasswordCheckRequest>,
) -> Result<Json<PasswordCheckResponse>, AppError> {
    // Track usage
    state.db.track_api_usage(user.user_id, "check_password").await?;

    // Load weak passwords list
    let weak_passwords = weak_pass::load_password_list("top-passwords.txt")
        .map_err(|e| AppError::InternalServerError(format!("Failed to load password list: {}", e)))?;

    let is_weak = weak_pass::is_password_weak(&payload.password, &weak_passwords);
    let strength_score = calculate_password_strength(&payload.password);
    let entropy = calculate_password_entropy(&payload.password);

    // Check if password is in breaches
    let password_hash = hash_password(&payload.password);
    let password_breaches = state.db.check_password_breaches(&password_hash).await?;
    let is_breached = !password_breaches.is_empty();

    let mut recommendations = Vec::new();
    
    if is_weak {
        recommendations.push("Use a longer, more complex password".to_string());
        recommendations.push("Avoid common words and patterns".to_string());
    }
    
    if is_breached {
        recommendations.push("This password has been found in data breaches - change it immediately".to_string());
    }
    
    if strength_score < 60 {
        recommendations.push("Add more character variety (uppercase, lowercase, numbers, symbols)".to_string());
    }
    
    if entropy < 50.0 {
        recommendations.push("Increase password length for better security".to_string());
    }

    // Store security report
    let report_data = serde_json::json!({
        "password_analysis": {
            "strength_score": strength_score,
            "is_weak": is_weak,
            "is_breached": is_breached,
            "entropy": entropy,
            "recommendations": recommendations
        }
    });

    let input_hash = hash_password(&payload.password);
    state.db.create_security_report(
        user.user_id,
        "password_check",
        &input_hash,
        &report_data.to_string(),
        Some(strength_score as i32),
    ).await?;

    info!("Password check completed for user: {}", user.email);

    Ok(Json(PasswordCheckResponse {
        is_weak,
        strength_score,
        is_breached,
        recommendations,
        entropy,
    }))
}

pub async fn check_breach(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<BreachCheckRequest>,
) -> Result<Json<BreachCheckResponse>, AppError> {
    // Track usage
    state.db.track_api_usage(user.user_id, "check_breach").await?;

    // Hash email for privacy
    let email_hash = hash_email(&payload.email);

    // Check breaches
    let breaches = state.db.check_email_breaches(&email_hash).await?;
    let is_breached = !breaches.is_empty();
    let breach_count = breaches.len() as u32;

    // Calculate risk score based on breaches
    let risk_score = if is_breached {
        let recent_breaches = breaches.iter()
            .filter(|b| {
                let breach_date = chrono::DateTime::parse_from_rfc3339(&b.breach_date.to_rfc3339())
                    .unwrap_or_else(|_| chrono::DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap());
                let one_year_ago = Utc::now() - chrono::Duration::days(365);
                breach_date > one_year_ago
            })
            .count();
        
        let base_score = 30 + (breach_count * 10).min(40);
        if recent_breaches > 0 { base_score + 20 } else { base_score }
    } else {
        10 // Base score for no known breaches
    };

    let breach_details = if payload.include_details.unwrap_or(false) {
        Some(breaches.iter().map(|b| {
            let data_types: Vec<String> = serde_json::from_str(&b.data_types)
                .unwrap_or_else(|_| vec!["unknown".to_string()]);
            
            BreachDetail {
                source: b.source_name.clone(),
                breach_date: b.breach_date.format("%Y-%m-%d").to_string(),
                data_types,
                severity: b.severity.clone(),
            }
        }).collect())
    } else {
        None
    };

    let mut recommendations = Vec::new();
    if is_breached {
        recommendations.push("Change passwords for all accounts using this email".to_string());
        recommendations.push("Enable two-factor authentication where possible".to_string());
        recommendations.push("Monitor your accounts for suspicious activity".to_string());
        
        if breach_count > 2 {
            recommendations.push("Consider using a different email for sensitive accounts".to_string());
        }
    } else {
        recommendations.push("Continue using strong, unique passwords".to_string());
        recommendations.push("Regularly monitor for new breaches".to_string());
    }

    // Store security report
    let report_data = serde_json::json!({
        "breach_check": {
            "email": payload.email,
            "is_breached": is_breached,
            "breach_count": breach_count,
            "risk_score": risk_score,
            "breaches": breach_details,
            "recommendations": recommendations
        }
    });

    state.db.create_security_report(
        user.user_id,
        "breach_check",
        &email_hash,
        &report_data.to_string(),
        Some(risk_score as i32),
    ).await?;

    info!("Breach check completed for user: {} (breaches: {})", user.email, breach_count);

    Ok(Json(BreachCheckResponse {
        email: payload.email,
        is_breached,
        breach_count,
        risk_score: risk_score as u8,
        breaches: breach_details,
        recommendations,
    }))
}

pub async fn calculate_risk_score(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<RiskScoreRequest>,
) -> Result<Json<RiskScoreResponse>, AppError> {
    // Track usage
    state.db.track_api_usage(user.user_id, "risk_score").await?;

    // Load weak passwords
    let weak_passwords = weak_pass::load_password_list("top-passwords.txt")
        .map_err(|e| AppError::InternalServerError(format!("Failed to load password list: {}", e)))?;

    // Create a JSON entry for risk calculation
    let mut entry = serde_json::Map::new();
    entry.insert("email".to_string(), serde_json::Value::String(payload.email.clone()));
    entry.insert("occurrences".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));

    if let Some(password) = &payload.password {
        entry.insert("password".to_string(), serde_json::Value::String(password.clone()));
    }

    // Add breach date (simulate recent breach for higher risk)
    let mut source = serde_json::Map::new();
    source.insert("breach_date".to_string(), serde_json::Value::String("2024-01-01".to_string()));
    entry.insert("source".to_string(), serde_json::Value::Object(source));

    let json_value = serde_json::Value::Object(entry);
    let base_risk_score = risk_score::calculate_risk_score(&json_value, &weak_passwords);

    // Check for additional risk factors
    let mut risk_factors = Vec::new();
    let mut total_risk = base_risk_score;

    // Check email breaches
    let email_hash = hash_email(&payload.email);
    let email_breaches = state.db.check_email_breaches(&email_hash).await?;
    if !email_breaches.is_empty() {
        let breach_impact = (email_breaches.len() * 10).min(30) as u8;
        total_risk = (total_risk + breach_impact).min(100);
        
        risk_factors.push(RiskFactor {
            factor_type: "email_breaches".to_string(),
            description: format!("Email found in {} data breach(es)", email_breaches.len()),
            score_impact: breach_impact,
            severity: if email_breaches.len() > 2 { "high".to_string() } else { "medium".to_string() },
        });
    }

    // Check password if provided
    if let Some(password) = &payload.password {
        let password_hash = hash_password(password);
        let password_breaches = state.db.check_password_breaches(&password_hash).await?;
        
        if !password_breaches.is_empty() {
            total_risk = (total_risk + 40).min(100);
            risk_factors.push(RiskFactor {
                factor_type: "password_breaches".to_string(),
                description: "Password found in data breaches".to_string(),
                score_impact: 40,
                severity: "critical".to_string(),
            });
        }

        if weak_pass::is_password_weak(password, &weak_passwords) {
            total_risk = (total_risk + 20).min(100);
            risk_factors.push(RiskFactor {
                factor_type: "weak_password".to_string(),
                description: "Password is commonly used and weak".to_string(),
                score_impact: 20,
                severity: "high".to_string(),
            });
        }
    }

    let mut recommendations = Vec::new();
    if total_risk > 70 {
        recommendations.push("Immediate action required - high security risk detected".to_string());
        recommendations.push("Change all passwords associated with this email".to_string());
        recommendations.push("Enable two-factor authentication immediately".to_string());
        recommendations.push("Monitor accounts for unauthorized access".to_string());
    } else if total_risk > 40 {
        recommendations.push("Moderate risk - consider security improvements".to_string());
        recommendations.push("Use unique, strong passwords for each account".to_string());
        recommendations.push("Enable two-factor authentication".to_string());
    } else {
        recommendations.push("Good security posture - maintain current practices".to_string());
        recommendations.push("Continue monitoring for new threats".to_string());
    }

    // Store security report
    let report_data = serde_json::json!({
        "risk_assessment": {
            "email": payload.email,
            "risk_score": total_risk,
            "factors": risk_factors,
            "recommendations": recommendations
        }
    });

    let input_hash = hash_email(&payload.email);
    state.db.create_security_report(
        user.user_id,
        "risk_assessment",
        &input_hash,
        &report_data.to_string(),
        Some(total_risk as i32),
    ).await?;

    info!("Risk assessment completed for user: {} (score: {})", user.email, total_risk);

    Ok(Json(RiskScoreResponse {
        risk_score: total_risk,
        email: payload.email,
        factors: risk_factors,
        recommendations,
    }))
}

pub async fn bulk_security_check(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<BulkSecurityCheckRequest>,
) -> Result<Json<BulkSecurityCheckResponse>, AppError> {
    // Check if user has permission for bulk operations
    if user.subscription_tier == crate::database::UserSubscriptionTier::Free {
        return Err(AppError::Forbidden("Bulk operations require a premium subscription".to_string()));
    }

    // Limit bulk size
    let max_bulk_size = match user.subscription_tier {
        crate::database::UserSubscriptionTier::Pro => 100,
        crate::database::UserSubscriptionTier::Enterprise => 1000,
        _ => 10,
    };

    if payload.emails.len() > max_bulk_size {
        return Err(AppError::BadRequest(format!("Bulk operation limited to {} items", max_bulk_size)));
    }

    // Track usage
    state.db.track_api_usage(user.user_id, "bulk_check").await?;

    let weak_passwords = weak_pass::load_password_list("top-passwords.txt")
        .map_err(|e| AppError::InternalServerError(format!("Failed to load password list: {}", e)))?;

    let mut results = Vec::new();
    let mut breached_count = 0;
    let mut high_risk_count = 0;
    let mut weak_passwords_count = 0;

    for (index, email) in payload.emails.iter().enumerate() {
        let email_hash = hash_email(email);
        let breaches = state.db.check_email_breaches(&email_hash).await?;
        let is_breached = !breaches.is_empty();
        
        if is_breached {
            breached_count += 1;
        }

        let mut risk_score = if is_breached { 50 } else { 10 };
        let mut password_weak = None;

        // Check password if provided
        if let Some(passwords) = &payload.passwords {
            if let Some(password) = passwords.get(index) {
                let is_weak = weak_pass::is_password_weak(password, &weak_passwords);
                password_weak = Some(is_weak);
                
                if is_weak {
                    risk_score += 20;
                    weak_passwords_count += 1;
                }

                let password_hash = hash_password(password);
                let password_breaches = state.db.check_password_breaches(&password_hash).await?;
                if !password_breaches.is_empty() {
                    risk_score += 30;
                }
            }
        }

        if risk_score > 60 {
            high_risk_count += 1;
        }

        results.push(BulkCheckResult {
            email: email.clone(),
            is_breached,
            risk_score: risk_score as u8,
            password_weak,
        });
    }

    let summary = BulkCheckSummary {
        total_checked: payload.emails.len() as u32,
        breached_count,
        high_risk_count,
        weak_passwords: weak_passwords_count,
    };

    // Store bulk report
    let report_data = serde_json::json!({
        "bulk_security_check": {
            "results": results,
            "summary": summary,
            "checked_at": Utc::now()
        }
    });

    let input_hash = format!("bulk_{}", uuid::Uuid::new_v4());
    state.db.create_security_report(
        user.user_id,
        "bulk_check",
        &input_hash,
        &report_data.to_string(),
        None,
    ).await?;

    info!("Bulk security check completed for user: {} ({} items)", user.email, payload.emails.len());

    Ok(Json(BulkSecurityCheckResponse {
        results,
        summary,
    }))
}

pub async fn filter_data(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<DataFilterRequest>,
) -> Result<Json<DataFilterResponse>, AppError> {
    // Track usage
    state.db.track_api_usage(user.user_id, "filter_data").await?;

    // Convert payload data to JSON string for processing
    let input_json = serde_json::to_string_pretty(&payload.data)?;
    
    // Create temporary files for processing (in production, use in-memory processing)
    let temp_input = format!("/tmp/guardr_input_{}.json", uuid::Uuid::new_v4());
    let temp_output = format!("/tmp/guardr_output_{}.json", uuid::Uuid::new_v4());
    
    // Write input data to temporary file
    std::fs::write(&temp_input, &input_json)
        .map_err(|e| AppError::InternalServerError(format!("Failed to write temp file: {}", e)))?;

    let original_entries = count_json_entries(&payload.data);

    // Apply filter based on type
    let result = match payload.filter_type.as_str() {
        "basic" => {
            filter::run_filter(&temp_input, &temp_output)
                .map_err(|e| AppError::InternalServerError(format!("Filter failed: {}", e)))?;
            "basic"
        }
        "advanced" => {
            filtermain::run_advanced_filter(&temp_input, &temp_output)
                .map_err(|e| AppError::InternalServerError(format!("Advanced filter failed: {}", e)))?;
            "advanced"
        }
        _ => return Err(AppError::BadRequest("Invalid filter type. Use 'basic' or 'advanced'".to_string())),
    };

    // Read filtered data
    let output_json = std::fs::read_to_string(&temp_output)
        .map_err(|e| AppError::InternalServerError(format!("Failed to read output: {}", e)))?;
    
    let filtered_data: serde_json::Value = serde_json::from_str(&output_json)?;
    let filtered_entries = count_json_entries(&filtered_data);
    let removed_entries = original_entries.saturating_sub(filtered_entries);

    // Cleanup temporary files
    let _ = std::fs::remove_file(&temp_input);
    let _ = std::fs::remove_file(&temp_output);

    info!("Data filtering completed for user: {} (removed {} entries)", user.email, removed_entries);

    Ok(Json(DataFilterResponse {
        filtered_data,
        removed_entries,
        filter_applied: result.to_string(),
    }))
}

fn count_json_entries(data: &serde_json::Value) -> u32 {
    match data {
        serde_json::Value::Object(obj) => {
            if let Some(result) = obj.get("result") {
                if let Some(array) = result.as_array() {
                    return array.len() as u32;
                }
            }
        }
        serde_json::Value::Array(arr) => {
            return arr.len() as u32;
        }
        _ => {}
    }
    0
}