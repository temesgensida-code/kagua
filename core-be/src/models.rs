use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entity returned from Python `/parse` endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySpan {
    pub text: String,
    pub label: String,
    pub start_char: usize,
    pub end_char: usize,
    pub confidence: f64,
}

/// Clause returned from Python `/parse` endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClauseSpan {
    pub clause_type: String,
    pub text: String,
    pub start_char: usize,
    pub end_char: usize,
}

/// Summary returned from Python `/parse` endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseSummary {
    pub total_entities: usize,
    pub dates_count: usize,
    pub monetary_count: usize,
    pub jurisdictions: Vec<String>,
    pub detected_clause_types: Vec<String>,
    pub suggested_domain: Option<String>,
    pub detected_jurisdiction: Option<String>,
}

/// Full JSON payload returned from FastAPI `/parse`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserResponse {
    pub filename: String,
    pub content_type: Option<String>,
    pub text_length: usize,
    pub raw_text: String,
    pub entities: Vec<EntitySpan>,
    pub clauses: Vec<ClauseSpan>,
    pub summary: ParseSummary,
    pub rag_facts: Option<serde_json::Value>,
    pub fact_provenance: Option<serde_json::Value>,
    pub pii_redacted_count: Option<usize>,
    pub retrieved_chunks: Option<serde_json::Value>,
    pub proclamation_metadata: Option<serde_json::Value>,
}

/// Request sent to SWI-Prolog `/reason` endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonerRequest {
    pub domain: serde_json::Value,
    pub facts: serde_json::Map<String, serde_json::Value>,
}

/// Individual raw violation from SWI-Prolog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologViolation {
    pub domain: String,
    pub rule: String,
    pub title: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
}

/// Response returned from SWI-Prolog `/reason` endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonerResponse {
    pub status: String,
    pub domain: serde_json::Value,
    pub violations_count: usize,
    pub violations: Vec<PrologViolation>,
}

/// Mapped violation with character offset slice snippet in the original text & statutory citations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedViolation {
    pub domain: String,
    pub rule: String,
    pub title: String,
    pub severity: String,
    pub description: String,
    pub recommendation: String,
    pub snippet: Option<String>,
    pub start_char: Option<usize>,
    pub end_char: Option<usize>,
    pub article_citation: Option<String>,
    pub statutory_text: Option<String>,
}

/// AI configuration passed from frontend
#[derive(Debug, Clone, Default)]
pub struct AIParams {
    pub enabled: bool,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub api_key: Option<String>,
}

/// AI reasoning detail per extracted fact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFactReasoning {
    pub fact_key: String,
    pub value: serde_json::Value,
    pub confidence: f64,
    pub explanation: String,
    pub source_quote: Option<String>,
}

/// Response returned from FastAPI `/ai-extract`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIExtractionResponse {
    pub status: String,
    pub provider: String,
    pub model: String,
    pub ai_facts: serde_json::Map<String, serde_json::Value>,
    pub ai_reasoning: Vec<AIFactReasoning>,
    pub facts_extracted_count: usize,
}

/// Final comprehensive analysis report returned by `POST /analyze`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub filename: String,
    pub text_length: usize,
    pub domains_checked: serde_json::Value,
    pub suggested_domain: Option<String>,
    pub detected_jurisdiction: Option<String>,
    pub total_violations: usize,
    pub critical_count: usize,
    pub warning_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_score: Option<f64>,
    pub violations: Vec<MappedViolation>,
    pub entities_extracted: usize,
    pub clauses_detected: usize,
    pub pii_redacted_count: Option<usize>,
    pub rag_facts: Option<serde_json::Value>,
    pub fact_provenance: Option<serde_json::Value>,
    pub proclamation_metadata: Option<serde_json::Value>,
    pub matched_articles: Option<serde_json::Value>,
    pub raw_text: Option<String>,
    pub ai_augmented: Option<bool>,
    pub ai_model_used: Option<String>,
    pub ai_facts_count: Option<usize>,
    pub ai_reasoning: Option<Vec<AIFactReasoning>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

/// Summary item for listing historical audit reports
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditReportSummary {
    pub id: Uuid,
    pub document_name: String,
    pub document_hash: String,
    pub domain: String,
    pub total_violations: i32,
    pub critical_count: i32,
    pub warning_count: i32,
    pub compliance_score: f64,
    pub pii_redacted_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Client-friendly serializable summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReportSummaryDto {
    pub id: Uuid,
    pub document_name: String,
    pub document_hash: String,
    pub domain: String,
    pub total_violations: i32,
    pub critical_count: i32,
    pub warning_count: i32,
    pub compliance_score: f64,
    pub pii_redacted_count: i32,
    pub created_at: DateTime<Utc>,
}

/// WebSocket Progress Stage Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub stage: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp_ms: u64,
}

