use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use governor::{Quota, RateLimiter};
use std::{
    collections::HashMap,
    net::IpAddr,
    num::NonZeroU32,
    sync::{Arc, Mutex},
    time::Duration,
};
use tracing::{info, warn};

use crate::{
    auth::AuthenticatedUser,
    errors::AppError,
    state::AppState,
};

// Rate limiting middleware
pub type IpRateLimiter = RateLimiter<String, governor::state::keyed::DefaultKeyedStateStore<String>, governor::clock::DefaultClock>;

pub fn create_rate_limiter(requests_per_minute: u32) -> Arc<IpRateLimiter> {
    let quota = Quota::per_minute(NonZeroU32::new(requests_per_minute).unwrap());
    Arc::new(RateLimiter::keyed(quota))
}

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract IP address
    let ip = extract_ip_from_headers(&headers)
        .unwrap_or_else(|| "unknown".to_string());

    // Get rate limiter for IP
    let rate_limiter = create_rate_limiter(state.settings.rate_limiting.requests_per_minute);

    // Check rate limit
    match rate_limiter.check_key(&ip) {
        Ok(_) => {
            // Request allowed
            let response = next.run(request).await;
            Ok(response)
        }
        Err(_) => {
            warn!("Rate limit exceeded for IP: {}", ip);
            Err(AppError::RateLimitExceeded(format!(
                "Rate limit exceeded for IP {}. Max {} requests per minute.",
                ip,
                state.settings.rate_limiting.requests_per_minute
            )))
        }
    }
}

fn extract_ip_from_headers(headers: &HeaderMap) -> Option<String> {
    // Check common headers for real IP
    if let Some(forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded_for.to_str() {
            // Take the first IP from the comma-separated list
            if let Some(ip) = value.split(',').next() {
                return Some(ip.trim().to_string());
            }
        }
    }

    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            return Some(value.to_string());
        }
    }

    if let Some(cf_ip) = headers.get("cf-connecting-ip") {
        if let Ok(value) = cf_ip.to_str() {
            return Some(value.to_string());
        }
    }

    None
}

// Request logging middleware
pub async fn request_logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start_time = std::time::Instant::now();

    let response = next.run(request).await;

    let duration = start_time.elapsed();
    let status = response.status();

    info!(
        method = %method,
        uri = %uri,
        status = %status.as_u16(),
        duration_ms = %duration.as_millis(),
        "Request completed"
    );

    response
}

// Security headers middleware
pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;

    let headers = response.headers_mut();
    
    // Add security headers
    headers.insert(
        "X-Content-Type-Options",
        "nosniff".parse().unwrap(),
    );
    headers.insert(
        "X-Frame-Options",
        "DENY".parse().unwrap(),
    );
    headers.insert(
        "X-XSS-Protection",
        "1; mode=block".parse().unwrap(),
    );
    headers.insert(
        "Strict-Transport-Security",
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    headers.insert(
        "Referrer-Policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    headers.insert(
        "Content-Security-Policy",
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'".parse().unwrap(),
    );

    response
}

// API key middleware (alternative to JWT)
pub async fn api_key_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if let Some(api_key) = headers.get("x-api-key") {
        if let Ok(key_str) = api_key.to_str() {
            let key_hash = state.auth.hash_api_key(key_str);
            
            if let Some((api_key_record, user)) = state.db.get_api_key_by_hash(&key_hash).await? {
                // Update last used timestamp
                state.db.update_api_key_last_used(api_key_record.id).await?;

                // Add user info to request extensions
                request.extensions_mut().insert(AuthenticatedUser {
                    user_id: user.id,
                    email: user.email.clone(),
                    subscription_tier: user.subscription_tier.clone(),
                    claims: crate::auth::Claims {
                        sub: user.id.to_string(),
                        email: user.email,
                        tier: user.subscription_tier.to_string(),
                        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
                        iat: chrono::Utc::now().timestamp(),
                        jti: uuid::Uuid::new_v4().to_string(),
                    },
                });

                return Ok(next.run(request).await);
            }
        }
    }

    Err(AppError::Unauthorized("Invalid or missing API key".to_string()))
}

// Usage tracking middleware
pub async fn usage_tracking_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    
    // Extract user from request (if authenticated)
    if let Some(user) = request.extensions().get::<AuthenticatedUser>() {
        // Track API usage in background
        let db = state.db.clone();
        let user_id = user.user_id;
        let endpoint = path.to_string();
        
        tokio::spawn(async move {
            if let Err(e) = db.track_api_usage(user_id, &endpoint).await {
                warn!("Failed to track API usage: {}", e);
            }
        });
    }

    Ok(next.run(request).await)
}

// Subscription tier validation middleware
pub async fn subscription_validation_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = request.uri().path();
    
    if let Some(user) = request.extensions().get::<AuthenticatedUser>() {
        // Check subscription limits for specific endpoints
        match path {
            path if path.contains("/bulk-") => {
                if user.subscription_tier == crate::database::UserSubscriptionTier::Free {
                    return Err(AppError::Forbidden(
                        "Bulk operations require a premium subscription".to_string()
                    ));
                }
            }
            path if path.contains("/admin/") => {
                if user.subscription_tier != crate::database::UserSubscriptionTier::Enterprise {
                    return Err(AppError::Forbidden(
                        "Admin operations require an enterprise subscription".to_string()
                    ));
                }
            }
            path if path.contains("/export") => {
                if user.subscription_tier == crate::database::UserSubscriptionTier::Free {
                    return Err(AppError::Forbidden(
                        "Export functionality requires a premium subscription".to_string()
                    ));
                }
            }
            _ => {}
        }
    }

    Ok(next.run(request).await)
}

// GDPR compliance middleware
pub async fn gdpr_compliance_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;

    // Add GDPR compliance headers
    let headers = response.headers_mut();
    headers.insert(
        "X-Data-Processing-Purpose",
        "security-analysis".parse().unwrap(),
    );
    headers.insert(
        "X-Data-Retention-Period",
        "30-days".parse().unwrap(),
    );
    headers.insert(
        "X-Data-Controller",
        "Guardr Security Platform".parse().unwrap(),
    );

    response
}