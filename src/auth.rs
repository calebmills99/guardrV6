use anyhow::{anyhow, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use axum::{
    async_trait,
    extract::{FromRequest, Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    Json, RequestExt,
};
use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::config::Settings;
use crate::database::{Database, User, UserSubscriptionTier};
use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // User ID
    pub email: String,       // User email
    pub tier: String,        // Subscription tier
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub jti: String,        // JWT ID for blacklisting
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,         // User ID
    pub jti: String,         // Token ID
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
}

#[derive(Debug, Clone)]
pub struct AuthService {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub settings: Settings,
}

impl AuthService {
    pub fn new(settings: Settings) -> Self {
        let key = settings.auth.jwt_secret.as_bytes();
        Self {
            encoding_key: EncodingKey::from_secret(key),
            decoding_key: DecodingKey::from_secret(key),
            settings,
        }
    }

    pub async fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(e) => Err(anyhow!("Failed to hash password: {}", e)),
        }
    }

    pub async fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow!("Invalid password hash: {}", e))?;
        
        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn generate_tokens(&self, user: &User) -> Result<(String, String)> {
        let now = Utc::now();
        let access_exp = now + Duration::hours(self.settings.auth.jwt_expiration_hours as i64);
        let refresh_exp = now + Duration::days(self.settings.auth.refresh_token_expiration_days as i64);
        
        let access_jti = Uuid::new_v4().to_string();
        let refresh_jti = Uuid::new_v4().to_string();

        let access_claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            tier: user.subscription_tier.to_string(),
            exp: access_exp.timestamp(),
            iat: now.timestamp(),
            jti: access_jti,
        };

        let refresh_claims = RefreshTokenClaims {
            sub: user.id.to_string(),
            jti: refresh_jti,
            exp: refresh_exp.timestamp(),
            iat: now.timestamp(),
        };

        let access_token = encode(&Header::default(), &access_claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to encode access token: {}", e))?;

        let refresh_token = encode(&Header::default(), &refresh_claims, &self.encoding_key)
            .map_err(|e| anyhow!("Failed to encode refresh token: {}", e))?;

        Ok((access_token, refresh_token))
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();
        
        match decode::<Claims>(token, &self.decoding_key, &validation) {
            Ok(token_data) => {
                if token_data.claims.exp < Utc::now().timestamp() {
                    return Err(anyhow!("Token has expired"));
                }
                Ok(token_data.claims)
            }
            Err(e) => Err(anyhow!("Invalid token: {}", e)),
        }
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims> {
        let validation = Validation::default();
        
        match decode::<RefreshTokenClaims>(token, &self.decoding_key, &validation) {
            Ok(token_data) => {
                if token_data.claims.exp < Utc::now().timestamp() {
                    return Err(anyhow!("Refresh token has expired"));
                }
                Ok(token_data.claims)
            }
            Err(e) => Err(anyhow!("Invalid refresh token: {}", e)),
        }
    }

    pub async fn blacklist_token(&self, redis: &mut redis::Connection, jti: &str, exp: i64) -> Result<()> {
        let ttl = exp - Utc::now().timestamp();
        if ttl > 0 {
            redis.set_ex::<&str, &str, ()>(&format!("blacklist:{}", jti), "true", ttl as u64)
                .await
                .map_err(|e| anyhow!("Failed to blacklist token: {}", e))?;
        }
        Ok(())
    }

    pub async fn is_token_blacklisted(&self, redis: &mut redis::Connection, jti: &str) -> Result<bool> {
        let exists: bool = redis.exists(&format!("blacklist:{}", jti))
            .await
            .map_err(|e| anyhow!("Failed to check token blacklist: {}", e))?;
        Ok(exists)
    }

    pub fn generate_api_key(&self) -> String {
        let random_bytes: Vec<u8> = (0..self.settings.auth.api_key_length)
            .map(|_| rand::thread_rng().gen())
            .collect();
        
        general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes)
    }

    pub fn hash_api_key(&self, api_key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize())
    }
}

// Password validation
fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let min_length = 8;
    if password.len() < min_length {
        return Err(ValidationError::new("password_too_short"));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_uppercase {
        return Err(ValidationError::new("missing_uppercase"));
    }
    if !has_lowercase {
        return Err(ValidationError::new("missing_lowercase"));
    }
    if !has_digit {
        return Err(ValidationError::new("missing_digit"));
    }
    if !has_special {
        return Err(ValidationError::new("missing_special_char"));
    }

    Ok(())
}

// Request/Response models
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom = "validate_password_strength")]
    pub password: String,
    
    pub confirm_password: String,
    
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user: UserProfile,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub subscription_tier: String,
    pub usage_stats: UsageStats,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UsageStats {
    pub monthly_queries: u32,
    pub queries_used: u32,
    pub queries_remaining: u32,
    pub reset_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

// JWT authentication extractor
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
    pub subscription_tier: UserSubscriptionTier,
    pub claims: Claims,
}

#[async_trait]
impl<S> FromRequest<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(token) => token,
            None => return Err(AppError::Unauthorized("Missing authorization header".to_string())),
        };

        // Extract auth service from app state (this would need to be implemented based on your state structure)
        let auth_service = AuthService::new(crate::config::Settings::default()); // This should come from state
        
        let claims = auth_service.verify_access_token(token)
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;
        
        let subscription_tier = claims.tier.parse::<UserSubscriptionTier>()
            .map_err(|_| AppError::Unauthorized("Invalid subscription tier in token".to_string()))?;

        Ok(AuthenticatedUser {
            user_id,
            email: claims.email.clone(),
            subscription_tier,
            claims,
        })
    }
}

// API Key authentication
pub struct ApiKeyAuth {
    pub user_id: Uuid,
    pub key_id: String,
    pub subscription_tier: UserSubscriptionTier,
}

#[async_trait]
impl<S> FromRequest<S> for ApiKeyAuth
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let api_key = req
            .headers()
            .get("x-api-key")
            .and_then(|header| header.to_str().ok());

        let api_key = match api_key {
            Some(key) => key,
            None => return Err(AppError::Unauthorized("Missing API key".to_string())),
        };

        // Hash the provided API key
        let auth_service = AuthService::new(crate::config::Settings::default()); // This should come from state
        let hashed_key = auth_service.hash_api_key(api_key);

        // Look up the API key in the database (this would need database access from state)
        // For now, this is a placeholder
        return Err(AppError::Unauthorized("API key authentication not implemented".to_string()));
    }
}