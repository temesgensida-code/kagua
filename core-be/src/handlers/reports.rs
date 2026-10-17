use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    auth::middleware::{AppState, AuthUser},
    models::{AnalysisReport, AuditReportSummary, AuditReportSummaryDto},
};

#[derive(Debug, Deserialize)]
pub struct ListReportsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListReportsResponse {
    pub total: i64,
    pub reports: Vec<AuditReportSummaryDto>,
}

/// GET /reports — List audit reports belonging to the authenticated user
pub async fn list_reports(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<ListReportsQuery>,
) -> Result<Json<ListReportsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let limit = query.limit.unwrap_or(20).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);

    let count_row = sqlx::query("SELECT COUNT(*) as total FROM audit_reports WHERE user_id = $1")
        .bind(auth_user.user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Failed to count audit reports: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "db_error", "message": "Failed to count reports" })),
            )
        })?;

    let total: i64 = count_row.try_get("total").unwrap_or(0);

    let rows: Vec<AuditReportSummary> = sqlx::query_as::<_, AuditReportSummary>(
        r#"
        SELECT
            id, document_name, document_hash, domain,
            total_violations, critical_count, warning_count,
            compliance_score, pii_redacted_count, created_at
        FROM audit_reports
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(auth_user.user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to list audit reports: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "db_error", "message": "Failed to retrieve reports" })),
        )
    })?;

    let dtos = rows
        .into_iter()
        .map(|r| AuditReportSummaryDto {
            id: r.id,
            document_name: r.document_name,
            document_hash: r.document_hash,
            domain: r.domain,
            total_violations: r.total_violations,
            critical_count: r.critical_count,
            warning_count: r.warning_count,
            compliance_score: r.compliance_score,
            pii_redacted_count: r.pii_redacted_count,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(ListReportsResponse {
        total,
        reports: dtos,
    }))
}

/// GET /reports/:id — Retrieve a full analysis report by ID
pub async fn get_report(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisReport>, (StatusCode, Json<serde_json::Value>)> {
    let row = sqlx::query(
        r#"
        SELECT report_payload
        FROM audit_reports
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(auth_user.user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to fetch audit report {}: {}", id, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "db_error", "message": "Database query failed" })),
        )
    })?;

    match row {
        Some(r) => {
            let payload: serde_json::Value = r.try_get("report_payload").map_err(|e| {
                error!("Corrupt report payload in db: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "corrupt_data", "message": "Failed to decode report payload" })),
                )
            })?;

            let mut report: AnalysisReport = serde_json::from_value(payload).map_err(|e| {
                error!("Failed to deserialize report: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "deserialization_error", "message": "Failed to deserialize report" })),
                )
            })?;

            report.id = Some(id);
            Ok(Json(report))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "not_found", "message": "Report not found" })),
        )),
    }
}

/// DELETE /reports/:id — Delete an audit report by ID
pub async fn delete_report(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let res = sqlx::query("DELETE FROM audit_reports WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(auth_user.user_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Failed to delete audit report {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "db_error", "message": "Database deletion failed" })),
            )
        })?;

    if res.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "not_found", "message": "Report not found or already deleted" })),
        ));
    }

    info!("Audit report {} deleted by user {}", id, auth_user.user_id);
    Ok(Json(json!({ "status": "deleted", "id": id })))
}
