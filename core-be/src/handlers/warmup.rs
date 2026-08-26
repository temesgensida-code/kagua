use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};
use tokio::time::{timeout, Duration};
use tracing::info;

/// GET /warmup
///
/// Public (no JWT required) endpoint that pings the downstream Python Parser
/// and SWI-Prolog Reasoner services in parallel. Render Free Tier containers
/// spin down after 15 min of inactivity; hitting their /health endpoints
/// forces them to cold-start before the user uploads a document.
///
/// Returns 200 {"ready": true} once both services respond, or 503 {"ready": false}
/// with per-service status while they are still waking up.
pub async fn warmup_handler() -> (StatusCode, Json<Value>) {
    let parser_base = std::env::var("PARSER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
    let parser_base = parser_base.trim_end_matches('/').to_string();

    let reasoner_base = std::env::var("REASONER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8081".to_string());
    let reasoner_base = reasoner_base.trim_end_matches('/').to_string();

    let parser_url   = format!("{}/health", parser_base);
    let reasoner_url = format!("{}/health", reasoner_base);

    info!("Warmup: pinging {} and {}", parser_url, reasoner_url);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(28))
        .build()
        .unwrap_or_default();

    // Ping both services in parallel — 28s timeout each
    let (parser_res, reasoner_res) = tokio::join!(
        timeout(Duration::from_secs(28), client.get(&parser_url).send()),
        timeout(Duration::from_secs(28), client.get(&reasoner_url).send()),
    );

    let parser_ok = parser_res
        .ok()
        .and_then(|r| r.ok())
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    let reasoner_ok = reasoner_res
        .ok()
        .and_then(|r| r.ok())
        .map(|r| r.status().is_success())
        .unwrap_or(false);

    let all_ready = parser_ok && reasoner_ok;

    info!(
        "Warmup result: parser={}, reasoner={}, ready={}",
        parser_ok, reasoner_ok, all_ready
    );

    let status = if all_ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status,
        Json(json!({
            "ready": all_ready,
            "services": {
                "parser":   if parser_ok   { "online" } else { "warming" },
                "reasoner": if reasoner_ok { "online" } else { "warming" }
            }
        })),
    )
}
