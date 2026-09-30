use anyhow::Result;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::config::Config;
use crate::auth::models::TokenClaims;

pub fn create_access_token(user_public_id: &Uuid, config: &Config) -> Result<String> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    let claims = TokenClaims {
        sub: user_public_id.to_string(),
        exp: now + config.jwt_access_token_expires_secs,
        iat: now,
        token_type: "access".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn create_refresh_token(user_public_id: &Uuid, config: &Config) -> Result<String> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
    let claims = TokenClaims {
        sub: user_public_id.to_string(),
        exp: now + config.jwt_refresh_token_expires_secs,
        iat: now,
        token_type: "refresh".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_token(token: &str, config: &Config) -> Result<TokenClaims> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    
    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    )?;

    Ok(token_data.claims)
}
