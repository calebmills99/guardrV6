use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    #[error("Access forbidden: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Password hashing error: {0}")]
    HashError(String),

    #[error("External API error: {0}")]
    ExternalApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Request timeout: {0}")]
    RequestTimeout(String),

    #[error("Insufficient storage: {0}")]
    InsufficientStorage(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::RateLimitExceeded(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::RequestTimeout(_) => StatusCode::REQUEST_TIMEOUT,
            AppError::InsufficientStorage(_) => StatusCode::INSUFFICIENT_STORAGE,
            AppError::DatabaseError(sqlx_err) => {
                match sqlx_err {
                    sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                    sqlx::Error::Database(db_err) => {
                        if db_err.is_unique_violation() {
                            StatusCode::CONFLICT
                        } else if db_err.is_foreign_key_violation() {
                            StatusCode::BAD_REQUEST
                        } else {
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
                    }
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_type(&self) -> &'static str {
        match self {
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::RateLimitExceeded(_) => "RATE_LIMIT_EXCEEDED",
            AppError::TooManyRequests(_) => "TOO_MANY_REQUESTS",
            AppError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            AppError::RequestTimeout(_) => "REQUEST_TIMEOUT",
            AppError::InsufficientStorage(_) => "INSUFFICIENT_STORAGE",
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::RedisError(_) => "REDIS_ERROR",
            AppError::JsonError(_) => "JSON_ERROR",
            AppError::JwtError(_) => "JWT_ERROR",
            AppError::HashError(_) => "HASH_ERROR",
            AppError::ExternalApiError(_) => "EXTERNAL_API_ERROR",
            AppError::ConfigError(_) => "CONFIG_ERROR",
            AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            AppError::Unauthorized(_) => "Authentication required or invalid credentials".to_string(),
            AppError::Forbidden(_) => "Access denied for this resource".to_string(),
            AppError::NotFound(_) => "The requested resource was not found".to_string(),
            AppError::ValidationError(msg) => format!("Validation failed: {}", msg),
            AppError::BadRequest(msg) => format!("Invalid request: {}", msg),
            AppError::RateLimitExceeded(_) => "Rate limit exceeded. Please try again later".to_string(),
            AppError::TooManyRequests(_) => "Too many requests. Please slow down".to_string(),
            AppError::ServiceUnavailable(_) => "Service temporarily unavailable. Please try again later".to_string(),
            AppError::RequestTimeout(_) => "Request timeout. Please try again".to_string(),
            AppError::InsufficientStorage(_) => "Storage quota exceeded".to_string(),
            AppError::DatabaseError(sqlx_err) => {
                match sqlx_err {
                    sqlx::Error::RowNotFound => "Resource not found".to_string(),
                    sqlx::Error::Database(db_err) => {
                        if db_err.is_unique_violation() {
                            "Resource already exists".to_string()
                        } else if db_err.is_foreign_key_violation() {
                            "Invalid reference to related resource".to_string()
                        } else {
                            "Database operation failed".to_string()
                        }
                    }
                    _ => "Database operation failed".to_string(),
                }
            }
            _ => "An unexpected error occurred. Please try again later".to_string(),
        }
    }

    pub fn should_log(&self) -> bool {
        match self {
            AppError::Unauthorized(_) => false, // Common, don't log
            AppError::Forbidden(_) => false,    // Common, don't log
            AppError::NotFound(_) => false,     // Common, don't log
            AppError::ValidationError(_) => false, // User error, don't log
            AppError::BadRequest(_) => false,   // User error, don't log
            AppError::RateLimitExceeded(_) => false, // Expected behavior
            AppError::TooManyRequests(_) => false, // Expected behavior
            _ => true, // Log all other errors
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_type = self.error_type();
        let user_message = self.user_message();
        
        // Log error if it should be logged
        if self.should_log() {
            error!("Application error: {} - {}", error_type, self);
        }

        let body = json!({
            "success": false,
            "error": {
                "type": error_type,
                "message": user_message,
                "code": status.as_u16()
            },
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "request_id": uuid::Uuid::new_v4().to_string() // In production, get from request context
        });

        (status, Json(body)).into_response()
    }
}

// Helper functions for common validation errors
pub fn validation_error_response(errors: &validator::ValidationErrors) -> AppError {
    let mut error_map = HashMap::new();
    
    for (field, field_errors) in errors.errors() {
        let messages: Vec<String> = field_errors
            .iter()
            .map(|error| {
                match error.code.as_ref() {
                    "email" => "Invalid email format".to_string(),
                    "length" => {
                        if let Some(min) = error.params.get("min") {
                            format!("Must be at least {} characters long", min)
                        } else if let Some(max) = error.params.get("max") {
                            format!("Must be at most {} characters long", max)
                        } else {
                            "Invalid length".to_string()
                        }
                    }
                    "range" => "Value out of allowed range".to_string(),
                    "password_too_short" => "Password must be at least 8 characters long".to_string(),
                    "missing_uppercase" => "Password must contain at least one uppercase letter".to_string(),
                    "missing_lowercase" => "Password must contain at least one lowercase letter".to_string(),
                    "missing_digit" => "Password must contain at least one number".to_string(),
                    "missing_special_char" => "Password must contain at least one special character".to_string(),
                    _ => error.message.as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "Invalid value".to_string()),
                }
            })
            .collect();
        
        error_map.insert(field.to_string(), messages);
    }

    let error_message = serde_json::to_string(&error_map)
        .unwrap_or_else(|_| "Validation failed".to_string());
    
    AppError::ValidationError(error_message)
}

// Convert anyhow errors to AppError
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

// Helper macros for common error patterns
#[macro_export]
macro_rules! unauthorized {
    ($msg:expr) => {
        return Err(AppError::Unauthorized($msg.to_string()))
    };
    () => {
        return Err(AppError::Unauthorized("Unauthorized".to_string()))
    };
}

#[macro_export]
macro_rules! forbidden {
    ($msg:expr) => {
        return Err(AppError::Forbidden($msg.to_string()))
    };
    () => {
        return Err(AppError::Forbidden("Forbidden".to_string()))
    };
}

#[macro_export]
macro_rules! not_found {
    ($msg:expr) => {
        return Err(AppError::NotFound($msg.to_string()))
    };
    ($resource:expr, $id:expr) => {
        return Err(AppError::NotFound(format!("{} with id {} not found", $resource, $id)))
    };
}

#[macro_export]
macro_rules! bad_request {
    ($msg:expr) => {
        return Err(AppError::BadRequest($msg.to_string()))
    };
}

#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        return Err(AppError::InternalServerError($msg.to_string()))
    };
}

// Rate limiting error helper
pub fn rate_limit_error(limit: u32, window: &str) -> AppError {
    AppError::RateLimitExceeded(format!(
        "Rate limit of {} requests per {} exceeded", 
        limit, 
        window
    ))
}

// Service unavailable error helper
pub fn service_unavailable_error(service: &str) -> AppError {
    AppError::ServiceUnavailable(format!("{} service is currently unavailable", service))
}

// Custom error types for specific business logic
#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Weak password detected")]
    WeakPassword,
    
    #[error("Email found in data breach")]
    EmailBreached,
    
    #[error("Password found in data breach")]
    PasswordBreached,
    
    #[error("High risk score: {0}")]
    HighRiskScore(u8),
    
    #[error("Invalid security analysis request")]
    InvalidAnalysisRequest,
}

impl From<SecurityError> for AppError {
    fn from(err: SecurityError) -> Self {
        match err {
            SecurityError::WeakPassword => AppError::BadRequest("Password is too weak".to_string()),
            SecurityError::EmailBreached => AppError::BadRequest("Email found in known data breaches".to_string()),
            SecurityError::PasswordBreached => AppError::BadRequest("Password found in known data breaches".to_string()),
            SecurityError::HighRiskScore(score) => AppError::BadRequest(format!("High security risk detected (score: {})", score)),
            SecurityError::InvalidAnalysisRequest => AppError::BadRequest("Invalid security analysis request".to_string()),
        }
    }
}