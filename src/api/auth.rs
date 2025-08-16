use axum::{extract::State, Json};
use chrono::Utc;
use tracing::{info, warn};
use validator::Validate;

use crate::auth::{AuthService, RegisterRequest, LoginRequest, AuthResponse, UserProfile, UsageStats, RefreshTokenRequest};
use crate::database::Database;
use crate::errors::{AppError, validation_error_response};
use crate::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate input
    if let Err(errors) = payload.validate() {
        return Err(validation_error_response(&errors));
    }

    // Check password confirmation
    if payload.password != payload.confirm_password {
        return Err(AppError::ValidationError("Passwords do not match".to_string()));
    }

    // Check if user already exists
    if let Some(_) = state.db.get_user_by_email(&payload.email).await? {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = state.auth.hash_password(&payload.password).await?;

    // Create user
    let user = state.db.create_user(&payload.email, &password_hash, payload.name.as_deref()).await?;

    // Generate tokens
    let (access_token, refresh_token) = state.auth.generate_tokens(&user)?;

    // Log successful registration
    info!("User registered successfully: {}", user.email);

    let user_profile = UserProfile {
        id: user.id.to_string(),
        email: user.email.clone(),
        name: user.name.clone(),
        subscription_tier: user.subscription_tier.to_string(),
        usage_stats: UsageStats {
            monthly_queries: 100, // Free tier default
            queries_used: 0,
            queries_remaining: 100,
            reset_date: Utc::now(),
        },
        created_at: user.created_at,
    };

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        expires_in: state.auth.settings.auth.jwt_expiration_hours * 3600,
        user: user_profile,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Validate input
    if let Err(errors) = payload.validate() {
        return Err(validation_error_response(&errors));
    }

    // Get user by email
    let user = state.db.get_user_by_email(&payload.email).await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    // Verify password
    if !state.auth.verify_password(&payload.password, &user.password_hash).await? {
        warn!("Failed login attempt for user: {}", payload.email);
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    // Update last login
    state.db.update_user_last_login(user.id).await?;

    // Generate tokens
    let (access_token, refresh_token) = state.auth.generate_tokens(&user)?;

    // Get usage stats (simplified for now)
    let current_month = Utc::now().format("%Y-%m").to_string();
    let usage = state.db.get_user_usage(user.id, &current_month).await?;
    let total_usage: i32 = usage.iter().map(|u| u.requests_count).sum();

    let monthly_limit = match user.subscription_tier {
        crate::database::UserSubscriptionTier::Free => 100,
        crate::database::UserSubscriptionTier::Pro => 5000,
        crate::database::UserSubscriptionTier::Enterprise => 50000,
    };

    info!("User logged in successfully: {}", user.email);

    let user_profile = UserProfile {
        id: user.id.to_string(),
        email: user.email.clone(),
        name: user.name.clone(),
        subscription_tier: user.subscription_tier.to_string(),
        usage_stats: UsageStats {
            monthly_queries: monthly_limit,
            queries_used: total_usage as u32,
            queries_remaining: (monthly_limit as i32 - total_usage).max(0) as u32,
            reset_date: Utc::now(),
        },
        created_at: user.created_at,
    };

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        expires_in: state.auth.settings.auth.jwt_expiration_hours * 3600,
        user: user_profile,
    }))
}

pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Verify refresh token
    let claims = state.auth.verify_refresh_token(&payload.refresh_token)?;

    // Get user
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;
    
    let user = state.db.get_user_by_id(user_id).await?
        .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

    // Generate new tokens
    let (access_token, refresh_token) = state.auth.generate_tokens(&user)?;

    // Get usage stats
    let current_month = Utc::now().format("%Y-%m").to_string();
    let usage = state.db.get_user_usage(user.id, &current_month).await?;
    let total_usage: i32 = usage.iter().map(|u| u.requests_count).sum();

    let monthly_limit = match user.subscription_tier {
        crate::database::UserSubscriptionTier::Free => 100,
        crate::database::UserSubscriptionTier::Pro => 5000,
        crate::database::UserSubscriptionTier::Enterprise => 50000,
    };

    let user_profile = UserProfile {
        id: user.id.to_string(),
        email: user.email.clone(),
        name: user.name.clone(),
        subscription_tier: user.subscription_tier.to_string(),
        usage_stats: UsageStats {
            monthly_queries: monthly_limit,
            queries_used: total_usage as u32,
            queries_remaining: (monthly_limit as i32 - total_usage).max(0) as u32,
            reset_date: Utc::now(),
        },
        created_at: user.created_at,
    };

    Ok(Json(AuthResponse {
        access_token,
        refresh_token,
        expires_in: state.auth.settings.auth.jwt_expiration_hours * 3600,
        user: user_profile,
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    // Extract JWT from the authorization header would be done via middleware
) -> Result<Json<serde_json::Value>, AppError> {
    // In a full implementation, we would:
    // 1. Extract the JWT from the authorization header
    // 2. Add it to the blacklist
    // 3. Optionally revoke the refresh token
    
    info!("User logged out");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Logged out successfully"
    })))
}