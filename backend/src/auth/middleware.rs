use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use uuid::Uuid;

use crate::app::AppState;
use crate::errors::AppError;
use crate::auth::jwt::verify_token;
use crate::auth::repository;

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub public_id: Uuid,
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization")
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| &value[7..]);

        let token = match auth_header {
            Some(token) => token,
            None => return Err(AppError::Unauthorized("Missing or invalid authorization header".into())),
        };

        let claims = verify_token(token, &state.config)
            .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

        if claims.token_type != "access" {
            return Err(AppError::Unauthorized("Invalid token type".into()));
        }

        let public_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid token subject".into()))?;

        let user = repository::find_user_by_public_id(&state.db, public_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("User not found".into()))?;

        let cred = repository::find_credential_by_user_id(&state.db, user.id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::Unauthorized("User credentials not found".into()))?;

        Ok(AuthUser {
            user_id: user.id,
            public_id: user.public_id,
            email: cred.email,
        })
    }
}
