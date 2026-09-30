use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod auth;
mod chat;
mod colleges;
mod comments;
mod communities;
mod config;
mod db;
mod errors;
mod events;
mod feed;
mod identity;
mod marketplace;
mod middleware;
mod notifications;
mod posts;
mod search;
mod users;
mod votes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "digital_campus=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Digital Campus backend...");

    // Load config
    let config = match config::Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    let config = Arc::new(config);

    // Init DB pool
    let db = db::init_pool(&config).await?;

    // Run migrations
    db::run_migrations(&db).await?;

    // Init Redis
    let redis_client = redis::Client::open(config.redis_url.clone())?;

    // Build App State
    let app_state = app::AppState {
        db,
        redis: redis_client,
        config: config.clone(),
    };

    // Build Router
    let app = app::create_router(app_state);

    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
