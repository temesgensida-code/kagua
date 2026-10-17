mod auth;
mod db;
mod email;
mod handlers;
mod models;
mod pdf;
mod secure_buffer;
mod service;
mod ws;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    extract::{Multipart, State},
    http::{header, HeaderMap, StatusCode},

    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer};
use axum::http::{Method, HeaderValue};
use tracing::info;

use crate::auth::middleware::AppState;
use crate::models::{AIParams, AnalysisReport};
use crate::pdf::generate_audit_pdf;
use crate::service::process_analysis_pipeline;
use crate::ws::{ws_handler, WsState};
use crate::handlers::auth as auth_handlers;
use crate::handlers::reports as report_handlers;
use crate::handlers::warmup::warmup_handler;
use sha2::{Digest, Sha256};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load .env file
    dotenvy::dotenv().ok();

    // Read required env vars
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set in .env");
    let resend_api_key = std::env::var("RESEND_API_KEY")
        .unwrap_or_default();
    let from_email = std::env::var("FROM_EMAIL")
        .unwrap_or_else(|_| "noreply@kagua.temesgen.works".to_string());
    let from_name = std::env::var("FROM_NAME")
        .unwrap_or_else(|_| "Kagua".to_string());
    let frontend_url = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    // Initialize DB pool and run migrations
    let db = db::init_db(&database_url).await;

    let ws_state = Arc::new(WsState::new());

    // Build shared application state
    let app_state = AppState {
        db,
        jwt_secret,
        resend_api_key,
        from_email,
        from_name,
        frontend_url,
        ws_state: ws_state.clone(),
    };

    let cors = if let Ok(frontend_url_str) = std::env::var("FRONTEND_URL") {
        if frontend_url_str == "*" {
            CorsLayer::permissive()
        } else if let Ok(origin) = frontend_url_str.parse::<HeaderValue>() {
            CorsLayer::new()
                .allow_origin(origin)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
                .allow_credentials(true)
        } else {
            CorsLayer::permissive()
        }
    } else {
        CorsLayer::permissive()
    };

    // ── Auth routes (public) ──────────────────────────────────────────────
    let auth_router = Router::new()
        .route("/register", post(auth_handlers::register))
        .route("/verify-email", post(auth_handlers::verify_email))
        .route("/resend-verification", post(auth_handlers::resend_verification))
        .route("/login", post(auth_handlers::login))
        .route("/refresh", post(auth_handlers::refresh_token))
        .route("/logout", post(auth_handlers::logout))
        .route("/forgot-password", post(auth_handlers::forgot_password))
        .route("/reset-password", post(auth_handlers::reset_password))
        .route("/me", get(auth_handlers::me));

    // ── Main application ──────────────────────────────────────────────────
    let app = Router::new()
        .route("/health", get(health_check))
        // Public warm-up endpoint — pings parser & reasoner, no JWT required
        .route("/warmup", get(warmup_handler))
        .route("/ws", get(ws_handler))
        // Protected routes — require a valid Bearer JWT
        .route("/analyze", post(analyze_handler))
        .route("/report/pdf", post(generate_pdf_handler))
        .route("/reports", get(report_handlers::list_reports))
        .route("/reports/:id", get(report_handlers::get_report).delete(report_handlers::delete_report))
        // Auth routes mounted under /auth
        .nest("/auth", auth_router)
        .layer(cors)
        .with_state(app_state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Kagua Rust Core Backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Axum server error");
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "kagua-core-be",
        "status": "online",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }))
}

async fn analyze_handler(
    State(state): State<AppState>,
    auth_user: crate::auth::middleware::AuthUser,
    mut multipart: Multipart,
) -> Result<Json<AnalysisReport>, (StatusCode, String)> {
    let _ = auth_user; // Auth is verified; user info available if needed for auditing

    let mut filename = String::from("document.pdf");
    let mut content_type = None;
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut domain_spec = serde_json::json!("all");
    let mut ai_params = AIParams::default();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        if name == "file" {
            if let Some(name_attr) = field.file_name() {
                filename = name_attr.to_string();
            }
            content_type = field.content_type().map(|c| c.to_string());
            let bytes = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read field bytes: {}", e)))?;
            file_bytes = Some(bytes.to_vec());
        } else if name == "domain" {
            let text = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to read domain text: {}", e)))?;
            if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&text) {
                domain_spec = parsed_json;
            } else {
                domain_spec = serde_json::json!(text);
            }
        } else if name == "ai_enabled" {
            let text = field.text().await.unwrap_or_default();
            ai_params.enabled = text == "true" || text == "1";
        } else if name == "ai_provider" {
            let text = field.text().await.unwrap_or_default();
            if !text.is_empty() { ai_params.provider = Some(text); }
        } else if name == "ai_model" {
            let text = field.text().await.unwrap_or_default();
            if !text.is_empty() { ai_params.model = Some(text); }
        } else if name == "ai_api_key" {
            let text = field.text().await.unwrap_or_default();
            if !text.is_empty() { ai_params.api_key = Some(text); }
        }
    }

    let bytes = file_bytes.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, "Missing 'file' field in multipart form upload".to_string())
    })?;

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let document_hash = format!("{:x}", hasher.finalize());

    let report = process_analysis_pipeline(
        filename, content_type, bytes, domain_spec, ai_params, &state.ws_state
    ).await?;

    let compliance_score = (100.0 - (report.critical_count as f64 * 15.0 + report.warning_count as f64 * 5.0)).max(0.0);
    let report_id = uuid::Uuid::new_v4();
    let created_at = chrono::Utc::now();
    let mut final_report = report;
    final_report.id = Some(report_id);
    final_report.compliance_score = Some(compliance_score);
    final_report.created_at = Some(created_at);

    let domain_str = final_report.suggested_domain.clone().unwrap_or_else(|| "all".to_string());
    let pii_count = final_report.pii_redacted_count.unwrap_or(0) as i32;

    if let Ok(report_payload) = serde_json::to_value(&final_report) {
        let insert_res = sqlx::query(
            r#"
            INSERT INTO audit_reports (
                id, user_id, document_name, document_hash, domain,
                total_violations, critical_count, warning_count,
                compliance_score, pii_redacted_count, report_payload, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#
        )
        .bind(report_id)
        .bind(auth_user.user_id)
        .bind(&final_report.filename)
        .bind(&document_hash)
        .bind(&domain_str)
        .bind(final_report.total_violations as i32)
        .bind(final_report.critical_count as i32)
        .bind(final_report.warning_count as i32)
        .bind(compliance_score)
        .bind(pii_count)
        .bind(report_payload)
        .bind(created_at)
        .execute(&state.db)
        .await;

        if let Err(e) = insert_res {
            tracing::warn!("Failed to persist audit report to DB: {}", e);
        } else {
            info!("Audit report {} persisted for user {}", report_id, auth_user.user_id);
        }
    }

    Ok(Json(final_report))
}

async fn generate_pdf_handler(
    State(state): State<AppState>,
    auth_user: crate::auth::middleware::AuthUser,
    Json(report): Json<AnalysisReport>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _ = (state, auth_user);

    let pdf_bytes = generate_audit_pdf(&report).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("PDF generation error: {}", e))
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!(
            "attachment; filename=\"kagua_audit_{}\"",
            report.filename.replace(".txt", ".pdf").replace(".docx", ".pdf")
        )
        .parse()
        .unwrap(),
    );

    Ok((headers, pdf_bytes))
}
