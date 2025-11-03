use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub redis: RedisConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
    pub rate_limiting: RateLimitConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub max_connections: Option<u32>,
    pub request_timeout_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub sqlite_url: String,
    pub mongodb_url: Option<String>,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub refresh_token_expiration_days: u64,
    pub bcrypt_cost: u32,
    pub api_key_length: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
    pub command_timeout_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    pub cors_allowed_origins: Vec<String>,
    pub encryption_key: String,
    pub password_min_length: usize,
    pub password_require_uppercase: bool,
    pub password_require_lowercase: bool,
    pub password_require_numbers: bool,
    pub password_require_special: bool,
    pub max_login_attempts: u32,
    pub lockout_duration_minutes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file_path: Option<String>,
    pub max_file_size_mb: u64,
    pub max_files: u32,
    pub json_format: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub premium_multiplier: u32,
    pub enterprise_multiplier: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
                workers: None,
                max_connections: Some(1000),
                request_timeout_seconds: 30,
            },
            database: DatabaseConfig {
                sqlite_url: "sqlite:./guardr.db".to_string(),
                mongodb_url: None,
                max_connections: 20,
                min_connections: 1,
                acquire_timeout_seconds: 30,
                idle_timeout_seconds: 300,
            },
            auth: AuthConfig {
                jwt_secret: "your-super-secret-jwt-key".to_string(),
                jwt_expiration_hours: 24,
                refresh_token_expiration_days: 30,
                bcrypt_cost: 12,
                api_key_length: 32,
            },
            redis: RedisConfig {
                url: "redis://127.0.0.1:6379".to_string(),
                max_connections: 10,
                connection_timeout_seconds: 5,
                command_timeout_seconds: 3,
            },
            security: SecurityConfig {
                cors_allowed_origins: vec!["https://guardr.app".to_string(), "https://www.guardr.app".to_string()],
                encryption_key: "your-encryption-key-32-chars-long!!".to_string(),
                password_min_length: 8,
                password_require_uppercase: true,
                password_require_lowercase: true,
                password_require_numbers: true,
                password_require_special: true,
                max_login_attempts: 5,
                lockout_duration_minutes: 15,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_path: Some("logs/guardr.log".to_string()),
                max_file_size_mb: 10,
                max_files: 5,
                json_format: true,
            },
            rate_limiting: RateLimitConfig {
                requests_per_minute: 60,
                burst_size: 10,
                premium_multiplier: 5,
                enterprise_multiplier: 20,
            },
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start off by merging in the default configuration
            .add_source(config::File::with_name("config/default").required(false))
            
            // Add in environment-specific config file (e.g., config/production.toml)
            .add_source(
                File::with_name(&format!(
                    "config/{}",
                    env::var("RUN_MODE").unwrap_or_else(|_| "development".into())
                ))
                .required(false),
            )
            
            // Add in a local configuration file (config/local.toml)
            .add_source(File::with_name("config/local").required(false))
            
            // Add in settings from the environment (with a prefix of GUARDR)
            // Eg.. `GUARDR_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(
                Environment::with_prefix("GUARDR")
                    .prefix_separator("_")
                    .separator("__")
            )
            
            .build()?;

        let mut settings: Settings = config.try_deserialize()?;
        
        // Override with environment variables for sensitive data
        if let Ok(jwt_secret) = env::var("JWT_SECRET") {
            settings.auth.jwt_secret = jwt_secret;
        }
        
        if let Ok(database_url) = env::var("DATABASE_URL") {
            settings.database.sqlite_url = database_url;
        }
        
        if let Ok(mongodb_url) = env::var("MONGODB_URL") {
            settings.database.mongodb_url = Some(mongodb_url);
        }
        
        if let Ok(redis_url) = env::var("REDIS_URL") {
            settings.redis.url = redis_url;
        }
        
        if let Ok(encryption_key) = env::var("ENCRYPTION_KEY") {
            settings.security.encryption_key = encryption_key;
        }

        Ok(settings)
    }

    pub fn validate(&self) -> Result<()> {
        // Validate JWT secret length
        if self.auth.jwt_secret.len() < 32 {
            return Err(anyhow::anyhow!("JWT secret must be at least 32 characters"));
        }

        // Validate encryption key length
        if self.security.encryption_key.len() != 32 {
            return Err(anyhow::anyhow!("Encryption key must be exactly 32 characters"));
        }

        // Validate bcrypt cost
        if self.auth.bcrypt_cost < 10 || self.auth.bcrypt_cost > 15 {
            return Err(anyhow::anyhow!("BCrypt cost must be between 10 and 15"));
        }

        Ok(())
    }
}