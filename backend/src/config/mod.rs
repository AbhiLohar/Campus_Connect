use std::env;
use anyhow::{Result, Context};

#[derive(Clone, Debug)]
pub struct Config {
    pub app_env: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_access_token_expires_secs: usize,
    pub jwt_refresh_token_expires_secs: usize,
    pub cors_allowed_origins: Vec<String>,
    pub rate_limit_rps: u64,
    pub rate_limit_burst: u32,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .context("DATABASE_MAX_CONNECTIONS must be a number")?;

        let jwt_access_token_expires_secs = env::var("JWT_ACCESS_TOKEN_EXPIRES_SECS")
            .unwrap_or_else(|_| "900".to_string())
            .parse()
            .context("JWT_ACCESS_TOKEN_EXPIRES_SECS must be a number")?;

        let jwt_refresh_token_expires_secs = env::var("JWT_REFRESH_TOKEN_EXPIRES_SECS")
            .unwrap_or_else(|_| "604800".to_string())
            .parse()
            .context("JWT_REFRESH_TOKEN_EXPIRES_SECS must be a number")?;

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let rate_limit_rps = env::var("RATE_LIMIT_REQUESTS_PER_SECOND")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .context("RATE_LIMIT_REQUESTS_PER_SECOND must be a number")?;

        let rate_limit_burst = env::var("RATE_LIMIT_BURST")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .context("RATE_LIMIT_BURST must be a number")?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .context("SERVER_PORT must be a number")?;

        Ok(Self {
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            database_max_connections,
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET").context("JWT_SECRET must be set")?,
            jwt_access_token_expires_secs,
            jwt_refresh_token_expires_secs,
            cors_allowed_origins,
            rate_limit_rps,
            rate_limit_burst,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port,
        })
    }
}
