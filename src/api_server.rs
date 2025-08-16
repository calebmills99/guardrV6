// Complete Guardr API Server
use anyhow::Result;
use axum::{
    middleware,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{info, warn, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import all modules
mod api;
mod auth;
mod config;
mod database;
mod errors;
mod middleware;
mod state;
mod filter;
mod filtermain;
mod fetch_dumps;
mod weak_pass;
mod risk_score;

use crate::{
    api,
    config::Settings,
    middleware::*,
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize configuration
    let settings = Settings::new()
        .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;
    
    // Validate configuration
    settings.validate()
        .map_err(|e| anyhow::anyhow!("Configuration validation failed: {}", e))?;

    // Initialize logging
    init_logging(&settings)?;
    
    info!("Starting Guardr API Server v{}", env!("CARGO_PKG_VERSION"));
    info!("Environment: {}", std::env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string()));

    // Initialize application state
    let app_state = AppState::new(settings.clone()).await
        .map_err(|e| anyhow::anyhow!("Failed to initialize application state: {}", e))?;

    info!("Database connection established");
    info!("Redis connection established");

    // Build the application router
    let app = build_app_router(app_state.clone());

    // Create server address
    let addr = SocketAddr::from((
        settings.server.host.parse::<std::net::IpAddr>()?
            .into(),
        settings.server.port,
    ));

    info!("Server starting on {}", addr);
    info!("API documentation available at http://{}/docs", addr);
    info!("Health check available at http://{}/health", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&addr).await
        .map_err(|e| anyhow::anyhow!("Failed to bind to address {}: {}", addr, e))?;

    info!("🚀 Guardr API Server is ready and listening on {}", addr);

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    info!("Server shut down gracefully");
    Ok(())
}

fn build_app_router(state: AppState) -> Router {
    // Create the main API router
    let api_router = api::create_router();

    // Build the complete application with middleware
    Router::new()
        .merge(api_router)
        // Add global middleware (order matters - first added = outermost layer)
        .layer(middleware::from_fn(gdpr_compliance_middleware))
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
        .layer(middleware::from_fn(request_logging_middleware))
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(
            state.settings.server.request_timeout_seconds
        )))
        .layer(TraceLayer::new_for_http())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CorsLayer::new()
            .allow_origin(state.settings.security.cors_allowed_origins.iter()
                .map(|origin| origin.parse().unwrap())
                .collect::<Vec<_>>()
            )
            .allow_methods(tower_http::cors::Any)
            .allow_headers(tower_http::cors::Any)
        )
        .with_state(state)
}

fn init_logging(settings: &Settings) -> Result<()> {
    let log_level = settings.logging.level.parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);

    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{},tower_http=debug,axum=debug", log_level).into())
        );

    if settings.logging.json_format {
        subscriber.with(tracing_subscriber::fmt::layer().json()).init();
    } else {
        subscriber.with(tracing_subscriber::fmt::layer()).init();
    }

    info!("Logging initialized with level: {}", log_level);
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, starting graceful shutdown");
        },
        _ = terminate => {
            info!("Received SIGTERM, starting graceful shutdown");
        },
    }
}