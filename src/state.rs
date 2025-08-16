use std::sync::Arc;

use crate::auth::AuthService;
use crate::config::Settings;
use crate::database::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub auth: Arc<AuthService>,
    pub settings: Arc<Settings>,
    pub redis: Arc<redis::Client>,
}

impl AppState {
    pub async fn new(settings: Settings) -> anyhow::Result<Self> {
        // Initialize database
        let db = Arc::new(Database::new(&settings).await?);

        // Initialize authentication service
        let auth = Arc::new(AuthService::new(settings.clone()));

        // Initialize Redis client
        let redis_client = redis::Client::open(settings.redis.url.clone())?;
        let redis = Arc::new(redis_client);

        let settings = Arc::new(settings);

        Ok(AppState {
            db,
            auth,
            settings,
            redis,
        })
    }
}