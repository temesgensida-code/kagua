use axum::{
    Json,
    extract::State,
    http::StatusCode,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{error, info};

use crate::auth::{
    jwt::create_access_token,
    middleware::AppState,
    password::{hash_password, verify_password},
};
use crate::email::{
    send_password_changed_email, send_password_reset_email, send_verification_email,
};

// ─── Request / Response DTOs ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    /// If true, refresh token TTL is extended to 30 days.
    pub remember_me: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ResendVerificationRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub user: UserDto,
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub is_verified: bool,
}

// ─── Internal DB row types (for query_as) ────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_verified: bool,
}

#[derive(Debug, sqlx::FromRow)]
struct UserIdRow {
    pub id: Uuid,
}

#[derive(Debug, sqlx::FromRow)]
struct UserVerifiedRow {
    pub id: Uuid,
    pub is_verified: bool,
}

#[derive(Debug, sqlx::FromRow)]
struct EmailTokenRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
struct RefreshTokenRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub email: String,
}

#[derive(Debug, sqlx::FromRow)]
struct MeRow {
    pub id: Uuid,
    pub email: String,
    pub is_verified: bool,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn error_json(error: &str, message: &str) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "error": error, "message": message }))
}

fn generate_secure_token() -> String {
    let bytes: [u8; 32] = rand::random();
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn sha256_hex(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// ─── Handlers ────────────────────────────────────────────────────────────────

/// POST /auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let email = payload.email.trim().to_lowercase();

    if !email.contains('@') || !email.contains('.') {
        return Err((StatusCode::BAD_REQUEST, error_json("invalid_email", "Please provide a valid email address.")));
    }
    if payload.password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, error_json("weak_password", "Password must be at least 8 characters.")));
    }

    let existing: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = $1")
        .bind(&email)
        .fetch_one(&state.db)
        .await
        .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    if existing > 0 {
        return Err((StatusCode::CONFLICT, error_json("email_taken", "An account with this email already exists.")));
    }

    let password_hash = hash_password(&payload.password)
        .map_err(|e| { error!("Hash: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("hash_error", "Failed to process password.")) })?;

    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id"
    )
    .bind(&email)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await
    .map_err(|e| { error!("DB insert user: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Failed to create account.")) })?;

    let verify_token = generate_secure_token();
    let expires_at = Utc::now() + Duration::hours(24);

    sqlx::query("INSERT INTO email_tokens (user_id, token, token_type, expires_at) VALUES ($1, $2, 'verify_email', $3)")
        .bind(user_id)
        .bind(&verify_token)
        .bind(expires_at)
        .execute(&state.db)
        .await
        .map_err(|e| { error!("DB insert email_token: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Failed to create verification token.")) })?;

    let verify_url = format!("{}/verify-email?token={}", state.frontend_url, verify_token);
    let from_addr = format!("{} <{}>", state.from_name, state.from_email);
    if let Err(e) = send_verification_email(&state.resend_api_key, &from_addr, &email, &verify_url).await {
        error!("Resend verification email failed: {}", e);
    }

    info!("Registered: {} ({})", email, user_id);
    Ok((StatusCode::CREATED, Json(serde_json::json!({
        "message": "Account created. Please check your email to verify your account.",
        "user_id": user_id
    }))))
}

/// POST /auth/verify-email
pub async fn verify_email(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let row: Option<EmailTokenRow> = sqlx::query_as(
        "SELECT id, user_id, expires_at, used_at FROM email_tokens WHERE token = $1 AND token_type = 'verify_email'"
    )
    .bind(&payload.token)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    let row = row.ok_or_else(|| (StatusCode::BAD_REQUEST, error_json("invalid_token", "Verification link is invalid or has already been used.")))?;

    if row.used_at.is_some() {
        return Err((StatusCode::BAD_REQUEST, error_json("token_used", "This verification link has already been used.")));
    }
    if row.expires_at < Utc::now() {
        return Err((StatusCode::BAD_REQUEST, error_json("token_expired", "Verification link has expired. Please request a new one.")));
    }

    sqlx::query("UPDATE email_tokens SET used_at = NOW() WHERE id = $1")
        .bind(row.id).execute(&state.db).await
        .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    sqlx::query("UPDATE users SET is_verified = TRUE, updated_at = NOW() WHERE id = $1")
        .bind(row.user_id).execute(&state.db).await
        .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    info!("Email verified: {}", row.user_id);
    Ok(Json(serde_json::json!({ "message": "Email verified successfully. You can now sign in." })))
}

/// POST /auth/resend-verification
pub async fn resend_verification(
    State(state): State<AppState>,
    Json(payload): Json<ResendVerificationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email = payload.email.trim().to_lowercase();

    let user: Option<UserVerifiedRow> = sqlx::query_as(
        "SELECT id, is_verified FROM users WHERE email = $1"
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    if let Some(u) = user {
        if !u.is_verified {
            let token = generate_secure_token();
            let expires = Utc::now() + Duration::hours(24);

            sqlx::query("UPDATE email_tokens SET used_at = NOW() WHERE user_id = $1 AND token_type = 'verify_email' AND used_at IS NULL")
                .bind(u.id).execute(&state.db).await.ok();

            sqlx::query("INSERT INTO email_tokens (user_id, token, token_type, expires_at) VALUES ($1, $2, 'verify_email', $3)")
                .bind(u.id).bind(&token).bind(expires).execute(&state.db).await.ok();

            let url = format!("{}/verify-email?token={}", state.frontend_url, token);
            let from = format!("{} <{}>", state.from_name, state.from_email);
            send_verification_email(&state.resend_api_key, &from, &email, &url).await.ok();
        }
    }

    Ok(Json(serde_json::json!({
        "message": "If an unverified account with that email exists, a new verification link has been sent."
    })))
}

/// POST /auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    let email = payload.email.trim().to_lowercase();

    let user: Option<UserRow> = sqlx::query_as(
        "SELECT id, email, password_hash, is_verified FROM users WHERE email = $1"
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { error!("DB login: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    let user = user.ok_or_else(|| (StatusCode::UNAUTHORIZED, error_json("invalid_credentials", "Email or password is incorrect.")))?;

    let valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, error_json("hash_error", "Authentication error.")))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, error_json("invalid_credentials", "Email or password is incorrect.")));
    }
    if !user.is_verified {
        return Err((StatusCode::FORBIDDEN, error_json("email_not_verified", "Please verify your email address before signing in.")));
    }

    let access_token = create_access_token(user.id, &user.email, &state.jwt_secret)
        .map_err(|e| { error!("JWT: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("token_error", "Failed to create session.")) })?;

    let raw_refresh = generate_secure_token();
    let token_hash = sha256_hex(&raw_refresh);
    let ttl = if payload.remember_me.unwrap_or(false) { Duration::days(30) } else { Duration::days(7) };
    let refresh_expires = Utc::now() + ttl;

    sqlx::query("INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
        .bind(user.id).bind(&token_hash).bind(refresh_expires)
        .execute(&state.db).await
        .map_err(|e| { error!("DB refresh insert: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Failed to create session.")) })?;

    info!("Login: {} ({})", user.email, user.id);
    Ok(Json(AuthResponse {
        access_token,
        refresh_token: raw_refresh,
        token_type: "Bearer".to_string(),
        user: UserDto { id: user.id, email: user.email, is_verified: user.is_verified },
    }))
}

/// POST /auth/refresh
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let token_hash = sha256_hex(&payload.refresh_token);

    let row: Option<RefreshTokenRow> = sqlx::query_as(
        "SELECT rt.id, rt.user_id, rt.expires_at, u.email \
         FROM refresh_tokens rt JOIN users u ON u.id = rt.user_id \
         WHERE rt.token_hash = $1"
    )
    .bind(&token_hash)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    let row = row.ok_or_else(|| (StatusCode::UNAUTHORIZED, error_json("invalid_refresh_token", "Refresh token is invalid or has expired.")))?;

    if row.expires_at < Utc::now() {
        sqlx::query("DELETE FROM refresh_tokens WHERE id = $1").bind(row.id).execute(&state.db).await.ok();
        return Err((StatusCode::UNAUTHORIZED, error_json("token_expired", "Session has expired. Please sign in again.")));
    }

    sqlx::query("DELETE FROM refresh_tokens WHERE id = $1").bind(row.id).execute(&state.db).await.ok();

    let new_access = create_access_token(row.user_id, &row.email, &state.jwt_secret)
        .map_err(|e| { error!("JWT: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("token_error", "Failed to refresh session.")) })?;

    let new_raw = generate_secure_token();
    let new_hash = sha256_hex(&new_raw);
    let new_exp = Utc::now() + Duration::days(7);

    sqlx::query("INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
        .bind(row.user_id).bind(&new_hash).bind(new_exp)
        .execute(&state.db).await
        .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    Ok(Json(serde_json::json!({
        "access_token": new_access,
        "refresh_token": new_raw,
        "token_type": "Bearer"
    })))
}

/// POST /auth/logout
pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let token_hash = sha256_hex(&payload.refresh_token);
    sqlx::query("DELETE FROM refresh_tokens WHERE token_hash = $1")
        .bind(&token_hash).execute(&state.db).await.ok();
    Ok(Json(serde_json::json!({ "message": "Logged out successfully." })))
}

/// POST /auth/forgot-password
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email = payload.email.trim().to_lowercase();

    let user: Option<UserIdRow> = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(&email).fetch_optional(&state.db).await
        .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    if let Some(u) = user {
        let token = generate_secure_token();
        let expires = Utc::now() + Duration::hours(1);

        sqlx::query("UPDATE email_tokens SET used_at = NOW() WHERE user_id = $1 AND token_type = 'reset_password' AND used_at IS NULL")
            .bind(u.id).execute(&state.db).await.ok();

        sqlx::query("INSERT INTO email_tokens (user_id, token, token_type, expires_at) VALUES ($1, $2, 'reset_password', $3)")
            .bind(u.id).bind(&token).bind(expires).execute(&state.db).await.ok();

        let url = format!("{}/reset-password?token={}", state.frontend_url, token);
        let from = format!("{} <{}>", state.from_name, state.from_email);
        send_password_reset_email(&state.resend_api_key, &from, &email, &url).await.ok();
    }

    Ok(Json(serde_json::json!({
        "message": "If an account with that email exists, a password reset link has been sent."
    })))
}

/// POST /auth/reset-password
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if payload.new_password.len() < 8 {
        return Err((StatusCode::BAD_REQUEST, error_json("weak_password", "Password must be at least 8 characters.")));
    }

    let row: Option<EmailTokenRow> = sqlx::query_as(
        "SELECT id, user_id, expires_at, used_at FROM email_tokens WHERE token = $1 AND token_type = 'reset_password'"
    )
    .bind(&payload.token)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    let row = row.ok_or_else(|| (StatusCode::BAD_REQUEST, error_json("invalid_token", "Password reset link is invalid or has already been used.")))?;

    if row.used_at.is_some() {
        return Err((StatusCode::BAD_REQUEST, error_json("token_used", "This reset link has already been used.")));
    }
    if row.expires_at < Utc::now() {
        return Err((StatusCode::BAD_REQUEST, error_json("token_expired", "Reset link has expired. Please request a new one.")));
    }

    let new_hash = hash_password(&payload.new_password)
        .map_err(|e| { error!("Hash: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("hash_error", "Failed to process password.")) })?;

    sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
        .bind(&new_hash).bind(row.user_id).execute(&state.db).await
        .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    sqlx::query("UPDATE email_tokens SET used_at = NOW() WHERE id = $1")
        .bind(row.id).execute(&state.db).await.ok();

    sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
        .bind(row.user_id).execute(&state.db).await.ok();

    let user_email: Option<String> = sqlx::query_scalar("SELECT email FROM users WHERE id = $1")
        .bind(row.user_id).fetch_optional(&state.db).await.unwrap_or(None);

    if let Some(ue) = user_email {
        let from = format!("{} <{}>", state.from_name, state.from_email);
        send_password_changed_email(&state.resend_api_key, &from, &ue).await.ok();
    }

    Ok(Json(serde_json::json!({ "message": "Password reset successfully. You can now sign in with your new password." })))
}

/// GET /auth/me  (protected)
pub async fn me(
    State(state): State<AppState>,
    auth_user: crate::auth::middleware::AuthUser,
) -> Result<Json<UserDto>, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<MeRow> = sqlx::query_as(
        "SELECT id, email, is_verified FROM users WHERE id = $1"
    )
    .bind(auth_user.user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { error!("DB: {}", e); (StatusCode::INTERNAL_SERVER_ERROR, error_json("db_error", "Database error.")) })?;

    let user = user.ok_or_else(|| (StatusCode::NOT_FOUND, error_json("user_not_found", "User not found.")))?;

    Ok(Json(UserDto { id: user.id, email: user.email, is_verified: user.is_verified }))
}
