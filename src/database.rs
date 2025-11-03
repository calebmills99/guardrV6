use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool, Row, FromRow};
use std::str::FromStr;
use uuid::Uuid;

use crate::config::Settings;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(settings: &Settings) -> Result<Self> {
        let pool = SqlitePool::connect(&settings.database.sqlite_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Database { pool })
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }
}

// User models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum UserSubscriptionTier {
    Free,
    Pro,
    Enterprise,
}

impl ToString for UserSubscriptionTier {
    fn to_string(&self) -> String {
        match self {
            UserSubscriptionTier::Free => "free".to_string(),
            UserSubscriptionTier::Pro => "pro".to_string(),
            UserSubscriptionTier::Enterprise => "enterprise".to_string(),
        }
    }
}

impl FromStr for UserSubscriptionTier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "free" => Ok(UserSubscriptionTier::Free),
            "pro" => Ok(UserSubscriptionTier::Pro),
            "enterprise" => Ok(UserSubscriptionTier::Enterprise),
            _ => Err(anyhow!("Invalid subscription tier: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub subscription_tier: UserSubscriptionTier,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub key_prefix: String, // First 8 characters for display
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct UsageTracking {
    pub id: Uuid,
    pub user_id: Uuid,
    pub month_year: String, // Format: "2024-01"
    pub endpoint: String,
    pub requests_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SecurityReport {
    pub id: Uuid,
    pub user_id: Uuid,
    pub report_type: String,
    pub input_data_hash: String, // Hash of input for privacy
    pub results: String, // JSON string
    pub risk_score: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct BreachData {
    pub id: Uuid,
    pub email_hash: String,
    pub password_hash: Option<String>,
    pub source_name: String,
    pub breach_date: DateTime<Utc>,
    pub data_types: String, // JSON array of strings
    pub severity: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
}

// User repository
impl Database {
    pub async fn create_user(&self, email: &str, password_hash: &str, name: Option<&str>) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, name, subscription_tier, email_verified, created_at, updated_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(name)
        .bind(UserSubscriptionTier::Free.to_string())
        .bind(false)
        .bind(now)
        .bind(now)
        .bind(true)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND is_active = true"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user_last_login(&self, user_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE users SET last_login = $1, updated_at = $1 WHERE id = $2"
        )
        .bind(Utc::now())
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user_subscription(&self, user_id: Uuid, tier: UserSubscriptionTier) -> Result<()> {
        sqlx::query(
            "UPDATE users SET subscription_tier = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(tier.to_string())
        .bind(Utc::now())
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // API Key methods
    pub async fn create_api_key(&self, user_id: Uuid, name: &str, key_hash: &str, key_prefix: &str) -> Result<ApiKey> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let api_key = sqlx::query_as::<_, ApiKey>(
            r#"
            INSERT INTO api_keys (id, user_id, name, key_hash, key_prefix, created_at, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(name)
        .bind(key_hash)
        .bind(key_prefix)
        .bind(now)
        .bind(true)
        .fetch_one(&self.pool)
        .await?;

        Ok(api_key)
    }

    pub async fn get_api_key_by_hash(&self, key_hash: &str) -> Result<Option<(ApiKey, User)>> {
        let result = sqlx::query(
            r#"
            SELECT 
                ak.id, ak.user_id, ak.name, ak.key_hash, ak.key_prefix, 
                ak.last_used, ak.created_at, ak.expires_at, ak.is_active,
                u.id as user_id, u.email, u.password_hash, u.name as user_name, 
                u.subscription_tier, u.email_verified, u.created_at as user_created_at, 
                u.updated_at, u.last_login, u.is_active as user_is_active
            FROM api_keys ak
            JOIN users u ON ak.user_id = u.id
            WHERE ak.key_hash = $1 AND ak.is_active = true AND u.is_active = true
            "#
        )
        .bind(key_hash)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let api_key = ApiKey {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    key_hash: row.get("key_hash"),
                    key_prefix: row.get("key_prefix"),
                    last_used: row.get("last_used"),
                    created_at: row.get("created_at"),
                    expires_at: row.get("expires_at"),
                    is_active: row.get("is_active"),
                };

                let user = User {
                    id: row.get("user_id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    name: row.get("user_name"),
                    subscription_tier: UserSubscriptionTier::from_str(&row.get::<String, _>("subscription_tier"))?,
                    email_verified: row.get("email_verified"),
                    created_at: row.get("user_created_at"),
                    updated_at: row.get("updated_at"),
                    last_login: row.get("last_login"),
                    is_active: row.get("user_is_active"),
                };

                Ok(Some((api_key, user)))
            }
            None => Ok(None),
        }
    }

    pub async fn update_api_key_last_used(&self, api_key_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE api_keys SET last_used = $1 WHERE id = $2"
        )
        .bind(Utc::now())
        .bind(api_key_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_user_api_keys(&self, user_id: Uuid) -> Result<Vec<ApiKey>> {
        let api_keys = sqlx::query_as::<_, ApiKey>(
            "SELECT * FROM api_keys WHERE user_id = $1 AND is_active = true ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(api_keys)
    }

    pub async fn revoke_api_key(&self, user_id: Uuid, api_key_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE api_keys SET is_active = false WHERE id = $1 AND user_id = $2"
        )
        .bind(api_key_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // Usage tracking
    pub async fn track_api_usage(&self, user_id: Uuid, endpoint: &str) -> Result<()> {
        let now = Utc::now();
        let month_year = now.format("%Y-%m").to_string();
        
        sqlx::query(
            r#"
            INSERT INTO usage_tracking (id, user_id, month_year, endpoint, requests_count, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 1, $5, $6)
            ON CONFLICT(user_id, month_year, endpoint) 
            DO UPDATE SET 
                requests_count = requests_count + 1,
                updated_at = $6
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(month_year)
        .bind(endpoint)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_usage(&self, user_id: Uuid, month_year: &str) -> Result<Vec<UsageTracking>> {
        let usage = sqlx::query_as::<_, UsageTracking>(
            "SELECT * FROM usage_tracking WHERE user_id = $1 AND month_year = $2"
        )
        .bind(user_id)
        .bind(month_year)
        .fetch_all(&self.pool)
        .await?;

        Ok(usage)
    }

    // Security reports
    pub async fn create_security_report(
        &self,
        user_id: Uuid,
        report_type: &str,
        input_data_hash: &str,
        results: &str,
        risk_score: Option<i32>,
    ) -> Result<SecurityReport> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = Some(now + chrono::Duration::days(30)); // Reports expire after 30 days
        
        let report = sqlx::query_as::<_, SecurityReport>(
            r#"
            INSERT INTO security_reports (id, user_id, report_type, input_data_hash, results, risk_score, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(report_type)
        .bind(input_data_hash)
        .bind(results)
        .bind(risk_score)
        .bind(now)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(report)
    }

    pub async fn get_user_reports(&self, user_id: Uuid, limit: i64, offset: i64) -> Result<Vec<SecurityReport>> {
        let reports = sqlx::query_as::<_, SecurityReport>(
            r#"
            SELECT * FROM security_reports 
            WHERE user_id = $1 AND (expires_at IS NULL OR expires_at > $2)
            ORDER BY created_at DESC 
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(user_id)
        .bind(Utc::now())
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(reports)
    }

    pub async fn get_report_by_id(&self, user_id: Uuid, report_id: Uuid) -> Result<Option<SecurityReport>> {
        let report = sqlx::query_as::<_, SecurityReport>(
            r#"
            SELECT * FROM security_reports 
            WHERE id = $1 AND user_id = $2 AND (expires_at IS NULL OR expires_at > $3)
            "#
        )
        .bind(report_id)
        .bind(user_id)
        .bind(Utc::now())
        .fetch_optional(&self.pool)
        .await?;

        Ok(report)
    }

    // Breach data management
    pub async fn store_breach_data(
        &self,
        email_hash: &str,
        password_hash: Option<&str>,
        source_name: &str,
        breach_date: DateTime<Utc>,
        data_types: &[String],
        severity: &str,
    ) -> Result<BreachData> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let data_types_json = serde_json::to_string(data_types)?;
        
        let breach_data = sqlx::query_as::<_, BreachData>(
            r#"
            INSERT INTO breach_data (id, email_hash, password_hash, source_name, breach_date, data_types, severity, verified, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(email_hash)
        .bind(password_hash)
        .bind(source_name)
        .bind(breach_date)
        .bind(data_types_json)
        .bind(severity)
        .bind(true)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(breach_data)
    }

    pub async fn check_email_breaches(&self, email_hash: &str) -> Result<Vec<BreachData>> {
        let breaches = sqlx::query_as::<_, BreachData>(
            "SELECT * FROM breach_data WHERE email_hash = $1 AND verified = true ORDER BY breach_date DESC"
        )
        .bind(email_hash)
        .fetch_all(&self.pool)
        .await?;

        Ok(breaches)
    }

    pub async fn check_password_breaches(&self, password_hash: &str) -> Result<Vec<BreachData>> {
        let breaches = sqlx::query_as::<_, BreachData>(
            "SELECT * FROM breach_data WHERE password_hash = $1 AND verified = true ORDER BY breach_date DESC"
        )
        .bind(password_hash)
        .fetch_all(&self.pool)
        .await?;

        Ok(breaches)
    }
}