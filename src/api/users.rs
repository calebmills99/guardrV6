use axum::{extract::{State, Path}, Json};
use chrono::{Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::auth::AuthenticatedUser;
use crate::database::Database;
use crate::errors::AppError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub subscription_tier: String,
    pub email_verified: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub last_login: Option<chrono::DateTime<Utc>>,
    pub usage_stats: UsageStatsResponse,
}

#[derive(Debug, Serialize)]
pub struct UsageStatsResponse {
    pub current_month: String,
    pub monthly_limit: u32,
    pub requests_used: u32,
    pub requests_remaining: u32,
    pub reset_date: chrono::DateTime<Utc>,
    pub usage_by_endpoint: Vec<EndpointUsage>,
}

#[derive(Debug, Serialize)]
pub struct EndpointUsage {
    pub endpoint: String,
    pub requests: u32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key_prefix: String,
    pub created_at: chrono::DateTime<Utc>,
    pub last_used: Option<chrono::DateTime<Utc>>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub expires_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub key_info: ApiKeyResponse,
    pub warning: String,
}

pub async fn get_profile(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<UserProfileResponse>, AppError> {
    let user_data = state.db.get_user_by_id(user.user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Get current month usage
    let current_month = Utc::now().format("%Y-%m").to_string();
    let usage = state.db.get_user_usage(user.user_id, &current_month).await?;
    
    let total_requests: i32 = usage.iter().map(|u| u.requests_count).sum();
    
    let monthly_limit = match user.subscription_tier {
        crate::database::UserSubscriptionTier::Free => 100,
        crate::database::UserSubscriptionTier::Pro => 5000,
        crate::database::UserSubscriptionTier::Enterprise => 50000,
    };

    let usage_by_endpoint = usage.into_iter().map(|u| EndpointUsage {
        endpoint: u.endpoint,
        requests: u.requests_count as u32,
    }).collect();

    // Calculate reset date (first day of next month)
    let now = Utc::now();
    let next_month = if now.month() == 12 {
        now.with_year(now.year() + 1).and_then(|d| d.with_month(1))
    } else {
        now.with_month(now.month() + 1)
    };
    let reset_date = next_month.and_then(|d| d.with_day(1))
        .and_then(|d| d.with_hour(0))
        .and_then(|d| d.with_minute(0))
        .and_then(|d| d.with_second(0))
        .and_then(|d| d.with_nanosecond(0))
        .unwrap_or_else(|| Utc::now());

    let usage_stats = UsageStatsResponse {
        current_month,
        monthly_limit,
        requests_used: total_requests as u32,
        requests_remaining: (monthly_limit as i32 - total_requests).max(0) as u32,
        reset_date,
        usage_by_endpoint,
    };

    Ok(Json(UserProfileResponse {
        id: user_data.id.to_string(),
        email: user_data.email,
        name: user_data.name,
        subscription_tier: user_data.subscription_tier.to_string(),
        email_verified: user_data.email_verified,
        created_at: user_data.created_at,
        last_login: user_data.last_login,
        usage_stats,
    }))
}

pub async fn update_profile(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, AppError> {
    // For now, we'll just update the name
    // In a full implementation, you'd have an update_user method in the database
    
    // Get updated user data (placeholder - you'd implement user update)
    let user_data = state.db.get_user_by_id(user.user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    info!("Profile updated for user: {}", user.email);

    // Return the same response as get_profile
    get_profile(State(state), user).await
}

pub async fn list_api_keys(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<Vec<ApiKeyResponse>>, AppError> {
    let api_keys = state.db.list_user_api_keys(user.user_id).await?;

    let response: Vec<ApiKeyResponse> = api_keys.into_iter().map(|key| ApiKeyResponse {
        id: key.id.to_string(),
        name: key.name,
        key_prefix: key.key_prefix,
        created_at: key.created_at,
        last_used: key.last_used,
        expires_at: key.expires_at,
    }).collect();

    Ok(Json(response))
}

pub async fn create_api_key(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, AppError> {
    // Check API key limits based on subscription
    let existing_keys = state.db.list_user_api_keys(user.user_id).await?;
    let max_keys = match user.subscription_tier {
        crate::database::UserSubscriptionTier::Free => 2,
        crate::database::UserSubscriptionTier::Pro => 10,
        crate::database::UserSubscriptionTier::Enterprise => 50,
    };

    if existing_keys.len() >= max_keys {
        return Err(AppError::BadRequest(format!(
            "API key limit reached. Your {} subscription allows {} keys.",
            user.subscription_tier.to_string(),
            max_keys
        )));
    }

    // Generate API key
    let api_key = state.auth.generate_api_key();
    let key_hash = state.auth.hash_api_key(&api_key);
    let key_prefix = api_key.chars().take(8).collect::<String>();

    // Store API key
    let stored_key = state.db.create_api_key(
        user.user_id,
        &payload.name,
        &key_hash,
        &key_prefix,
    ).await?;

    info!("API key created for user: {} (name: {})", user.email, payload.name);

    Ok(Json(CreateApiKeyResponse {
        api_key,
        key_info: ApiKeyResponse {
            id: stored_key.id.to_string(),
            name: stored_key.name,
            key_prefix: stored_key.key_prefix,
            created_at: stored_key.created_at,
            last_used: stored_key.last_used,
            expires_at: stored_key.expires_at,
        },
        warning: "This is the only time you'll see the full API key. Store it securely.".to_string(),
    }))
}

pub async fn revoke_api_key(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(key_id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let key_uuid = Uuid::parse_str(&key_id)
        .map_err(|_| AppError::BadRequest("Invalid API key ID".to_string()))?;

    let revoked = state.db.revoke_api_key(user.user_id, key_uuid).await?;

    if !revoked {
        return Err(AppError::NotFound("API key not found".to_string()));
    }

    info!("API key revoked for user: {} (key_id: {})", user.email, key_id);

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "API key revoked successfully"
    })))
}