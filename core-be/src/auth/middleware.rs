use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts, header},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::jwt::verify_access_token;

/// The authenticated user extracted from the Authorization header.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    #[allow(dead_code)]
    pub email: String,
}


/// Application state that holds the DB pool and JWT secret for the extractor.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub resend_api_key: String,
    pub from_email: String,
    pub from_name: String,
    pub frontend_url: String,
    pub ws_state: std::sync::Arc<crate::ws::WsState>,
}


#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({
                        "error": "missing_token",
                        "message": "Authorization header is required."
                    })),
                )
            })?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({
                        "error": "invalid_token_format",
                        "message": "Authorization header must be 'Bearer <token>'."
                    })),
                )
            })?;

        let claims = verify_access_token(token, &state.jwt_secret).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": "invalid_token",
                    "message": "Access token is invalid or has expired."
                })),
            )
        })?;

        Ok(AuthUser {
            user_id: claims.sub,
            email: claims.email,
        })
    }
}
