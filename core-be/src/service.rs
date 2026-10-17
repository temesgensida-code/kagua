use reqwest::multipart::{Form, Part};
use serde_json::{json, Map, Value};
use tracing::error;

use crate::models::{
    AIExtractionResponse, AIFactReasoning, AIParams, AnalysisReport, ClauseSpan, EntitySpan,
    MappedViolation, ParserResponse, PrologViolation, ReasonerRequest, ReasonerResponse,
};
use crate::secure_buffer::{SecureBuffer, SecureString};
use crate::ws::WsState;

pub async fn process_analysis_pipeline(
    filename: String,
    content_type: Option<String>,
    file_bytes: Vec<u8>,
    domain_spec: Value,
    ai_params: AIParams,
    ws_state: &WsState,
) -> Result<AnalysisReport, (axum::http::StatusCode, String)> {
    let parser_base = std::env::var("PARSER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
    let parser_base = parser_base.trim_end_matches('/').to_string();

    let reasoner_base = std::env::var("REASONER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8081".to_string());
    let reasoner_base = reasoner_base.trim_end_matches('/').to_string();

    let fastapi_parse_url = format!("{}/parse", parser_base);
    let fastapi_ai_extract_url = format!("{}/ai-extract", parser_base);
    let prolog_reason_url = format!("{}/reason", reasoner_base);

    let bytes_len = file_bytes.len();

    // 1. Wrap raw bytes in SecureBuffer (auto-zeroizes memory when dropped)
    let secure_bytes = SecureBuffer::new(file_bytes);

    ws_state.broadcast_progress(
        "UPLOAD_RECEIVED",
        &format!("Received uploaded file '{}' ({} bytes) in memory", filename, bytes_len),
        Some(json!({ "filename": filename, "bytes": bytes_len })),
    );

    // 2. Forward byte stream to FastAPI /parse endpoint
    ws_state.broadcast_progress(
        "PARSING_DOCUMENT",
        "Forwarding document stream to FastAPI parser & Privacy-Preserving RAG service...",
        None,
    );

    let domain_requested = match &domain_spec {
        Value::String(s) => s.clone(),
        _ => "auto".to_string(),
    };

    let parse_url = if domain_requested != "auto" && domain_requested != "all" {
        format!("{}?domain={}", fastapi_parse_url, domain_requested)
    } else {
        fastapi_parse_url.clone()
    };

    let client = reqwest::Client::new();
    let mut part = Part::bytes(secure_bytes.as_slice().to_vec()).file_name(filename.clone());
    if let Some(ref mime) = content_type {
        if let Ok(p) = Part::bytes(secure_bytes.as_slice().to_vec())
            .file_name(filename.clone())
            .mime_str(mime)
        {
            part = p;
        }
    }

    let form = Form::new().part("file", part);

    let parse_res = client
        .post(&parse_url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to connect to FastAPI parser: {}", e);
            (
                axum::http::StatusCode::BAD_GATEWAY,
                format!("FastAPI parser connection failed: {}", e),
            )
        })?;

    if !parse_res.status().is_success() {
        let status_code = parse_res.status();
        let err_text = parse_res.text().await.unwrap_or_default();
        error!("FastAPI parser error ({}): {}", status_code, err_text);

        let user_msg = if err_text.contains("<html") || status_code == reqwest::StatusCode::BAD_GATEWAY {
            "FastAPI parser service is offline or waking up from sleep on Render. Please wait 30 seconds and try again.".to_string()
        } else {
            format!("FastAPI document parsing failed: {}", err_text)
        };

        return Err((
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            user_msg,
        ));
    }

    let parser_response: ParserResponse = parse_res.json().await.map_err(|e| {
        error!("Failed to deserialize parser response: {}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Parser JSON deserialization failed: {}", e),
        )
    })?;

    let pii_count = parser_response.pii_redacted_count.unwrap_or(0);
    ws_state.broadcast_progress(
        "RAG_PRIVACY_ANONYMIZED",
        &format!("PII Anonymizer redacted {} sensitive user entity instances in memory (0 disk writes)", pii_count),
        Some(json!({ "pii_redacted_count": pii_count })),
    );

    let suggested_domain = parser_response.summary.suggested_domain.clone();
    let detected_jurisdiction = parser_response.summary.detected_jurisdiction.clone();

    // Determine active legal domain (Labour Law vs Commercial Code vs All)
    let active_domain = if domain_requested == "all" {
        "all".to_string()
    } else if domain_requested != "auto" {
        if domain_requested.contains("commercial") {
            "commercial_code".to_string()
        } else {
            "ethiopian_labour_proclamation".to_string()
        }
    } else if let Some(ref sugg) = suggested_domain {
        if sugg == "all" {
            "all".to_string()
        } else if sugg.contains("commercial") {
            "commercial_code".to_string()
        } else {
            "ethiopian_labour_proclamation".to_string()
        }
    } else {
        "ethiopian_labour_proclamation".to_string()
    };

    let governing_law = if active_domain == "commercial_code" {
        "Ethiopian Commercial Code Proclamation No. 1243/2021"
    } else if active_domain == "all" {
        "Ethiopian Labour Proclamation No. 1156/2019 & Commercial Code No. 1243/2021"
    } else {
        "Ethiopian Labour Proclamation No. 1156/2019"
    };

    let final_domain_spec = if active_domain == "all" {
        json!(["ethiopian_labour_proclamation", "commercial_code"])
    } else {
        json!(active_domain)
    };

    ws_state.broadcast_progress(
        "DOMAIN_SELECTED",
        &format!("Using {} compliance rule pack", governing_law),
        Some(json!({
            "domain": active_domain,
            "governing_law": governing_law,
            "jurisdiction": detected_jurisdiction
        })),
    );

    ws_state.broadcast_progress(
        "PARSING_COMPLETED",
        &format!(
            "Extracted {} text chars, {} entities, {} clauses, and RAG facts",
            parser_response.text_length,
            parser_response.entities.len(),
            parser_response.clauses.len()
        ),
        Some(json!({
            "text_length": parser_response.text_length,
            "entities_count": parser_response.entities.len(),
            "clauses_count": parser_response.clauses.len(),
            "suggested_domain": suggested_domain,
            "detected_jurisdiction": detected_jurisdiction,
            "rag_facts": parser_response.rag_facts,
        })),
    );

    // 3. Extract Ethiopian legal compliance facts for Prolog reasoning
    let mut facts = extract_prolog_facts(&parser_response, &active_domain);

    // 3b. AI-Augmented Fact Extraction (if AI is enabled)
    let mut ai_augmented = false;
    let mut ai_model_used = None;
    let mut ai_facts_count = None;
    let mut ai_reasoning: Option<Vec<AIFactReasoning>> = None;

    if ai_params.enabled {
        if let (Some(provider), Some(model), Some(api_key)) = (
            ai_params.provider.as_ref(),
            ai_params.model.as_ref(),
            ai_params.api_key.as_ref(),
        ) {
            ws_state.broadcast_progress(
                "AI_EXTRACTING",
                &format!("Sending anonymized text to AI model '{}' ({}) for RAG fact extraction...", model, provider),
                Some(json!({ "provider": provider, "model": model, "domain": active_domain })),
            );

            let ai_req_body = json!({
                "anonymized_text": parser_response.raw_text,
                "provider": provider,
                "model": model,
                "api_key": api_key,
                "proclamation_articles": parser_response.retrieved_chunks,
                "domain": active_domain
            });

            match client.post(&fastapi_ai_extract_url).json(&ai_req_body).send().await {
                Ok(ai_res) if ai_res.status().is_success() => {
                    if let Ok(ai_extraction) = ai_res.json::<AIExtractionResponse>().await {
                        let count = ai_extraction.ai_facts.len();
                        ws_state.broadcast_progress(
                            "AI_FACTS_RECEIVED",
                            &format!("AI fact extraction complete. Received {} compliance facts.", count),
                            Some(json!({ "ai_facts_count": count, "model": model })),
                        );

                        // Harmonize, validate types, and reconcile conflicts against baseline RAG facts
                        let harmonized_stats = harmonize_and_validate_facts(&mut facts, &ai_extraction);

                        ai_augmented = true;
                        ai_model_used = Some(model.clone());
                        ai_facts_count = Some(harmonized_stats.merged_count);
                        ai_reasoning = Some(ai_extraction.ai_reasoning);

                        ws_state.broadcast_progress(
                            "AI_FACTS_HARMONIZED",
                            &format!(
                                "Harmonized {} AI facts with RAG facts ({} merged, {} conflicts reconciled, {} low-confidence filtered).",
                                count, harmonized_stats.merged_count, harmonized_stats.reconciled_count, harmonized_stats.dropped_count
                            ),
                            Some(json!({
                                "merged_count": harmonized_stats.merged_count,
                                "reconciled_count": harmonized_stats.reconciled_count,
                                "dropped_count": harmonized_stats.dropped_count
                            })),
                        );
                    }
                }
                Err(e) => {
                    error!("AI extraction call failed: {}", e);
                    ws_state.broadcast_progress(
                        "AI_EXTRACTION_WARNING",
                        &format!("AI fact extraction failed ({}), falling back to standard in-memory Prolog matching.", e),
                        None,
                    );
                }
                Ok(res) => {
                    let status = res.status();
                    let err_text = res.text().await.unwrap_or_default();
                    error!("AI extraction endpoint returned error status {}: {}", status, err_text);
                    ws_state.broadcast_progress(
                        "AI_EXTRACTION_WARNING",
                        &format!("AI extraction error ({}), falling back to in-memory Prolog matching.", err_text),
                        None,
                    );
                }
            }
        }
    }

    ws_state.broadcast_progress(
        "FACTS_EXTRACTED",
        &format!("Extracted {} total {} compliance facts for reasoning", facts.len(), governing_law),
        Some(json!({
            "facts": facts,
            "governing_law": governing_law,
            "ai_augmented": ai_augmented
        })),
    );

    ws_state.broadcast_progress(
        "REASONING_PROLOG",
        "Sending facts to SWI-Prolog reasoning engine for violation checking...",
        Some(json!({ "domain": final_domain_spec, "facts": facts })),
    );

    // 4. Forward to SWI-Prolog /reason endpoint
    let reasoner_req = ReasonerRequest {
        domain: final_domain_spec.clone(),
        facts,
    };

    let reason_res = client
        .post(&prolog_reason_url)
        .json(&reasoner_req)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to connect to Prolog reasoner: {}", e);
            (
                axum::http::StatusCode::BAD_GATEWAY,
                format!("SWI-Prolog reasoner connection failed: {}", e),
            )
        })?;

    if !reason_res.status().is_success() {
        let status_code = reason_res.status();
        let err_text = reason_res.text().await.unwrap_or_default();
        error!("SWI-Prolog reasoner error ({}): {}", status_code, err_text);

        let user_msg = if err_text.contains("<html") || status_code == reqwest::StatusCode::BAD_GATEWAY {
            "SWI-Prolog reasoner service is offline or waking up from sleep on Render. Please wait 30 seconds and try again.".to_string()
        } else {
            format!("Prolog reasoning failed: {}", err_text)
        };

        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            user_msg,
        ));
    }

    let reasoner_response: ReasonerResponse = reason_res.json().await.map_err(|e| {
        error!("Failed to deserialize reasoner response: {}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Reasoner JSON deserialization failed: {}", e),
        )
    })?;

    ws_state.broadcast_progress(
        "MAPPING_OFFSETS",
        &format!("Mapping {} violations to text character offsets...", reasoner_response.violations.len()),
        Some(json!({ "violations_count": reasoner_response.violations.len() })),
    );

    // 5. Zero-copy String Offset Mapping & Statutory Article Citation Mapping
    let raw_text_secure = SecureString::new(parser_response.raw_text.clone());
    let mapped_violations = map_violations_to_offsets(
        raw_text_secure.as_str(),
        &reasoner_response.violations,
        &parser_response.clauses,
        &parser_response.entities,
        parser_response.retrieved_chunks.as_ref(),
    );

    let critical_count = mapped_violations.iter().filter(|v| v.severity == "critical").count();
    let warning_count = mapped_violations.iter().filter(|v| v.severity == "warning").count();

    ws_state.broadcast_progress(
        "COMPLETED",
        &format!("Analysis complete. Found {} total violations ({} critical, {} warning)",
            mapped_violations.len(), critical_count, warning_count),
        Some(json!({
            "total_violations": mapped_violations.len(),
            "critical_count": critical_count,
            "warning_count": warning_count
        })),
    );

    // 6. Explicitly drop secure_bytes & raw_text_secure (zeroing memory)
    drop(secure_bytes);
    drop(raw_text_secure);

    let report = AnalysisReport {
        filename,
        text_length: parser_response.text_length,
        domains_checked: final_domain_spec,
        suggested_domain,
        detected_jurisdiction,
        total_violations: mapped_violations.len(),
        critical_count,
        warning_count,
        violations: mapped_violations,
        entities_extracted: parser_response.entities.len(),
        clauses_detected: parser_response.clauses.len(),
        pii_redacted_count: parser_response.pii_redacted_count,
        rag_facts: parser_response.rag_facts,
        fact_provenance: parser_response.fact_provenance,
        proclamation_metadata: parser_response.proclamation_metadata,
        matched_articles: parser_response.retrieved_chunks,
        raw_text: Some(parser_response.raw_text),
        ai_augmented: Some(ai_augmented),
        ai_model_used,
        ai_facts_count,
        ai_reasoning,
        id: None,
        compliance_score: None,
        created_at: None,
    };

    Ok(report)
}


/// Convert extracted NER entities, clauses, and RAG facts into Prolog facts.
/// Supports both Ethiopian Labour Proclamation No. 1156/2019 and Commercial Code No. 1243/2021.
fn extract_prolog_facts(parser_res: &ParserResponse, active_domain: &str) -> Map<String, Value> {
    let mut facts = Map::new();

    let governing_law = if active_domain == "commercial_code" {
        "Ethiopian Commercial Code Proclamation No. 1243/2021"
    } else {
        "Ethiopian Labour Proclamation No. 1156/2019"
    };

    // 1. Set domain and governing law
    facts.insert("domain".to_string(), json!(active_domain));
    facts.insert("governing_law".to_string(), json!(governing_law));

    // 2. Jurisdiction from NER detection
    if let Some(ref jurisdiction) = parser_res.summary.detected_jurisdiction {
        facts.insert("jurisdiction".to_string(), json!(jurisdiction));
    } else if let Some(jurisdiction) = parser_res.summary.jurisdictions.first() {
        facts.insert("jurisdiction".to_string(), json!(jurisdiction));
    }

    // 3. Merge RAG-extracted facts directly
    if let Some(ref rag_obj) = parser_res.rag_facts {
        if let Some(obj_map) = rag_obj.as_object() {
            for (k, v) in obj_map {
                facts.insert(k.clone(), v.clone());
            }
        }
    }

    // 4. Map detected clause types to boolean presence facts
    for clause_type in &parser_res.summary.detected_clause_types {
        match clause_type.as_str() {
            // Labour Law Clauses
            "ETHIOPIAN_PROBATION_PERIOD" => {
                facts.insert("has_probation_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_WORKING_HOURS" => {
                facts.insert("has_working_hours_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_OVERTIME" => {
                facts.insert("has_overtime_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_TERMINATION_NOTICE" => {
                facts.insert("has_termination_notice_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_ANNUAL_LEAVE" => {
                facts.insert("has_annual_leave_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_MATERNITY_LEAVE" => {
                facts.insert("has_maternity_leave_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_SICK_LEAVE" => {
                facts.insert("has_sick_leave_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_SEVERANCE_PAY" => {
                facts.insert("has_severance_provision".to_string(), json!(true));
            }
            "ETHIOPIAN_PROHIBITED_ACTS" => {
                facts.insert("has_anti_discrimination".to_string(), json!(true));
            }
            "ETHIOPIAN_MINIMUM_AGE" => {
                facts.insert("has_minimum_age_clause".to_string(), json!(true));
            }
            "ETHIOPIAN_CONTRACT_FORMATION" => {
                facts.insert("has_written_contract_provision".to_string(), json!(true));
            }

            // Commercial Code Clauses
            "COMMERCIAL_COMPANY_CAPITAL" => {
                facts.insert("has_capital_clause".to_string(), json!(true));
            }
            "COMMERCIAL_SHARE_PAR_VALUE" => {
                facts.insert("has_par_value_clause".to_string(), json!(true));
            }
            "COMMERCIAL_FOUNDERS_MEMBERS" => {
                facts.insert("has_founders_clause".to_string(), json!(true));
            }
            "COMMERCIAL_BOARD_DIRECTORS" => {
                facts.insert("has_board_clause".to_string(), json!(true));
            }
            "COMMERCIAL_LEGAL_RESERVE" => {
                facts.insert("has_reserve_clause".to_string(), json!(true));
            }
            "COMMERCIAL_DIVIDEND_DISTRIBUTION" => {
                facts.insert("has_dividend_clause".to_string(), json!(true));
            }
            "COMMERCIAL_ACCOUNTING_AUDIT" => {
                facts.insert("has_accounting_clause".to_string(), json!(true));
            }
            "COMMERCIAL_TRANSFERABLE_SECURITIES" => {
                facts.insert("has_securities_clause".to_string(), json!(true));
            }
            "COMMERCIAL_RECORD_RETENTION" => {
                facts.insert("has_record_retention_clause".to_string(), json!(true));
            }
            "COMMERCIAL_FICTITIOUS_DIVIDENDS" => {
                facts.insert("has_fictitious_dividend_clause".to_string(), json!(true));
            }
            _ => {}
        }
    }

    facts
}

/// Safely extract a substring by Unicode character offsets (not byte offsets).
/// Never panics on multi-byte UTF-8 character boundaries (e.g., bullet points, smart quotes, Amharic text).
fn safe_char_slice(text: &str, start_char: usize, end_char: usize) -> String {
    if text.is_empty() {
        return String::new();
    }
    let char_count = text.chars().count();
    let start = start_char.min(char_count);
    let end = end_char.min(char_count);
    if start >= end {
        return String::new();
    }
    text.chars().skip(start).take(end - start).collect()
}

/// Zero-copy string offset mapping of Prolog violations to exact character slices & statutory citations
fn map_violations_to_offsets(
    raw_text: &str,
    violations: &[PrologViolation],
    clauses: &[ClauseSpan],
    entities: &[EntitySpan],
    retrieved_chunks: Option<&Value>,
) -> Vec<MappedViolation> {
    let mut mapped = Vec::new();

    for v in violations {
        let mut start_char = None;
        let mut end_char = None;
        let mut snippet_str: Option<String> = None;

        // Match violations to clause spans (Labour Law & Commercial Code)
        for clause in clauses {
            let matches_clause = match v.rule.as_str() {
                // Labour Law rules
                r if r.contains("Article 11") => clause.clause_type == "ETHIOPIAN_PROBATION_PERIOD",
                r if r.contains("Article 61") => clause.clause_type == "ETHIOPIAN_WORKING_HOURS",
                r if r.contains("Article 14") => clause.clause_type == "ETHIOPIAN_PROHIBITED_ACTS",
                r if r.contains("Article 35") || r.contains("Article 44") => clause.clause_type == "ETHIOPIAN_TERMINATION_NOTICE",
                r if r.contains("Article 77") => clause.clause_type == "ETHIOPIAN_ANNUAL_LEAVE",
                r if r.contains("Article 88") => clause.clause_type == "ETHIOPIAN_MATERNITY_LEAVE",
                r if r.contains("Article 89") || r.contains("Article 90") => clause.clause_type == "ETHIOPIAN_MINIMUM_AGE",
                r if r.contains("Article 85") => clause.clause_type == "ETHIOPIAN_SICK_LEAVE",
                r if r.contains("Article 39") => clause.clause_type == "ETHIOPIAN_SEVERANCE_PAY",
                r if r.contains("Article 67") || r.contains("Article 68") => clause.clause_type == "ETHIOPIAN_OVERTIME",
                r if r.contains("Article 4") || r.contains("Article 6") || r.contains("Article 7") => clause.clause_type == "ETHIOPIAN_CONTRACT_FORMATION",

                // Commercial Code rules
                r if r.contains("Article 247") || r.contains("Article 496") || r.contains("Article 249") || r.contains("Article 498") => {
                    clause.clause_type == "COMMERCIAL_COMPANY_CAPITAL" || clause.clause_type == "COMMERCIAL_SHARE_PAR_VALUE"
                }
                r if r.contains("Article 252") || r.contains("Article 495") => clause.clause_type == "COMMERCIAL_FOUNDERS_MEMBERS",
                r if r.contains("Article 254") || r.contains("Article 499") => clause.clause_type == "COMMERCIAL_COMPANY_CAPITAL",
                r if r.contains("Article 296") => clause.clause_type == "COMMERCIAL_BOARD_DIRECTORS",
                r if r.contains("Article 307") || r.contains("Article 524") => clause.clause_type == "COMMERCIAL_LEGAL_RESERVE",
                r if r.contains("Article 306") => clause.clause_type == "COMMERCIAL_DIVIDEND_DISTRIBUTION" || clause.clause_type == "COMMERCIAL_FICTITIOUS_DIVIDENDS",
                r if r.contains("Article 502") => clause.clause_type == "COMMERCIAL_TRANSFERABLE_SECURITIES",
                r if r.contains("Article 288") => clause.clause_type == "COMMERCIAL_RECORD_RETENTION" || clause.clause_type == "COMMERCIAL_ACCOUNTING_AUDIT",
                _ => false,
            };

            if matches_clause {
                start_char = Some(clause.start_char);
                end_char = Some(clause.end_char);
                let slice = safe_char_slice(raw_text, clause.start_char, clause.end_char);
                if !slice.is_empty() {
                    snippet_str = Some(slice);
                } else {
                    snippet_str = Some(clause.text.clone());
                }
                break;
            }
        }

        // Fallback: search entity span
        if start_char.is_none() {
            if let Some(ent) = entities.first() {
                start_char = Some(ent.start_char);
                end_char = Some(ent.end_char);
                snippet_str = Some(ent.text.clone());
            }
        }

        // Fallback: safe slice of raw_text prefix
        if start_char.is_none() && !raw_text.is_empty() {
            let slice = safe_char_slice(raw_text, 0, 200);
            let char_count = raw_text.chars().count();
            start_char = Some(0);
            end_char = Some(200.min(char_count));
            snippet_str = Some(slice);
        }

        // Attach statutory article citation & exact statutory text
        let (article_citation, statutory_text) = lookup_statutory_citation(&v.rule, retrieved_chunks);

        mapped.push(MappedViolation {
            domain: v.domain.clone(),
            rule: v.rule.clone(),
            title: v.title.clone(),
            severity: v.severity.clone(),
            description: v.description.clone(),
            recommendation: v.recommendation.clone(),
            snippet: snippet_str,
            start_char,
            end_char,
            article_citation,
            statutory_text,
        });
    }

    mapped
}

/// Lookup statutory article citation and exact text for Ethiopian Proclamations (Labour & Commercial Code)
fn lookup_statutory_citation(
    rule: &str,
    retrieved_chunks: Option<&Value>,
) -> (Option<String>, Option<String>) {
    // 1. Check retrieved_chunks from Python RAG response
    if let Some(chunks) = retrieved_chunks.and_then(|v| v.as_array()) {
        for chk in chunks {
            let src = chk.get("source").and_then(|s| s.as_str()).unwrap_or("");
            let art_num = chk.get("article_number").and_then(|n| n.as_i64()).unwrap_or(0);
            let art_str = format!("Article {}", art_num);
            if rule.contains(&art_str) {
                let title = chk.get("article_title").and_then(|s| s.as_str()).unwrap_or("");
                let snippet = chk.get("snippet").and_then(|s| s.as_str()).unwrap_or("");
                let proc_name = if src.contains("commercial") {
                    "Ethiopian Commercial Code Proclamation No. 1243/2021"
                } else {
                    "Ethiopian Labour Proclamation No. 1156/2019"
                };
                let citation = format!("{} - Article {} ({})", proc_name, art_num, title);
                return (Some(citation), Some(snippet.to_string()));
            }
        }
    }

    // 2. Fallback statutory lookup table
    let (citation, statutory) = match rule {
        // Labour Law Rules (Proclamation No. 1156/2019)
        r if r.contains("Article 11") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 11(3)",
            "Article 11(3): The period of probation shall be fixed by mutual agreement of the parties; provided, however, that it shall not exceed 60 (sixty) working days."
        ),
        r if r.contains("Article 61") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 61(1)",
            "Article 61(1): Normal hours of work shall not exceed 8 hours a day or 48 hours a week."
        ),
        r if r.contains("Article 67") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 67",
            "Article 67: Overtime work shall not exceed 2 hours in a day, 20 hours in a month or 100 hours in a year."
        ),
        r if r.contains("Article 77") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 77(1)",
            "Article 77(1): An worker shall be entitled to uninterrupted paid annual leave of 16 (sixteen) working days for the first year of service."
        ),
        r if r.contains("Article 88") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 88(2-3)",
            "Article 88(2-3): A pregnant worker shall be granted a period of 30 consecutive days of leave with pay preceding her presumed date of confinement and a period of 90 consecutive days of leave post confinement."
        ),
        r if r.contains("Article 85") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 85",
            "Article 85: Where a worker, after having completed probation, rendered service for not less than 6 months, sick leave granted shall not exceed 6 months."
        ),
        r if r.contains("Article 35") || r.contains("Article 44") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Articles 35 & 44",
            "Articles 35 & 44: A contract of employment may be terminated by written notice given by one of the parties to the other. Minimum notice period is 30 days."
        ),
        r if r.contains("Article 39") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 39",
            "Article 39: A worker who has completed probation and whose contract is terminated under lawful grounds shall be entitled to severance pay of thirty days' wages for the first year of service."
        ),
        r if r.contains("Article 14") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 14(1)(f) & (h)",
            "Article 14: It shall be unlawful for an employer to discriminate between workers on grounds of sex, religion, race, or status, or commit sexual harassment or sexual assault."
        ),
        r if r.contains("Article 89") || r.contains("Article 90") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Articles 89 & 90",
            "Article 89(1): It is prohibited to employ persons under 15 years of age. Article 90: Normal daily working hours for young workers (ages 15-18) shall not exceed 7 hours per day."
        ),
        r if r.contains("Article 6") => (
            "Ethiopian Labour Proclamation No. 1156/2019 - Article 6",
            "Article 6: Where a contract of employment is not made in writing, the employer shall issue a signed letter of employment containing essential terms within 15 days."
        ),

        // Commercial Code Rules (Proclamation No. 1243/2021)
        r if r.contains("Article 247(1)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 247(1)",
            "Article 247(1): The capital of a share company shall not be less than 50,000 (fifty thousand) Ethiopian Birr."
        ),
        r if r.contains("Article 496(1)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 496(1)",
            "Article 496(1): The capital of a private limited company shall not be less than 15,000 (fifteen thousand) Ethiopian Birr."
        ),
        r if r.contains("Article 247(2)") || r.contains("Article 496(2)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Articles 247(2) & 496(2)",
            "Articles 247(2) & 496(2): The par value of a share in a share company or private limited company shall not be less than 100 Ethiopian Birr."
        ),
        r if r.contains("Article 252(2)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 252(2)",
            "Article 252(2): A share company shall be established by at least 5 (five) founders."
        ),
        r if r.contains("Article 495(4)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 495(4)",
            "Article 495(4): A private limited company (PLC) shall be established by at least 2 (two) members and not more than 50 members."
        ),
        r if r.contains("Article 254(1)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 254(1)",
            "Article 254(1): At least 25% of the subscribed capital of a share company shall be paid up at the time of establishment."
        ),
        r if r.contains("Article 499(1)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 499(1)",
            "Article 499(1): The capital of a private limited company (PLC) shall be fully paid up (100%) upon establishment."
        ),
        r if r.contains("Article 296(1)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 296(1)",
            "Article 296(1): A share company shall be managed by a Board of Directors comprising not less than 3 and not more than 12 members. The majority of directors shall be non-executive."
        ),
        r if r.contains("Article 296(2)") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 296(2)",
            "Article 296(2): Directors who are non-shareholders shall not exceed two-thirds (2/3) of the total members of the Board of Directors."
        ),
        r if r.contains("Article 307(1)") || r.contains("Article 524") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Articles 307(1) & 524",
            "Articles 307(1) & 524: At least 5% of net profits shall be transferred annually to the legal reserve fund until it reaches 20% of the capital."
        ),
        r if r.contains("Article 502") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 502",
            "Article 502: A private limited company (PLC) is strictly prohibited from issuing bonds, debentures, or transferable securities."
        ),
        r if r.contains("Article 288") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 288",
            "Article 288: Books of accounts and financial accounting records shall be preserved for a minimum period of 10 years."
        ),
        r if r.contains("Article 306") => (
            "Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 306",
            "Article 306: Dividends shall only be distributed from actual net profits realized. Distribution of fictitious dividends is prohibited."
        ),
        _ => return (None, None),
    };

    (Some(citation.to_string()), Some(statutory.to_string()))
}


struct HarmonizationResult {
    merged_count: usize,
    dropped_count: usize,
    reconciled_count: usize,
}

fn coerce_value(v: &Value) -> Option<Value> {
    match v {
        Value::Null => None,
        Value::Bool(b) => Some(Value::Bool(*b)),
        Value::Number(n) => Some(Value::Number(n.clone())),
        Value::String(s) => {
            let s_lower = s.trim().to_lowercase();
            if s_lower == "true" || s_lower == "yes" {
                return Some(Value::Bool(true));
            }
            if s_lower == "false" || s_lower == "no" {
                return Some(Value::Bool(false));
            }
            // Parse number
            if let Ok(num) = s.trim().parse::<i64>() {
                return Some(json!(num));
            }
            if let Ok(num) = s.trim().parse::<f64>() {
                return Some(json!(num));
            }
            // Strip unit suffix like "60 days", "8.5 hours"
            let cleaned = s.replace(",", "");
            let parts: Vec<&str> = cleaned.split_whitespace().collect();
            if let Some(first) = parts.first() {
                if let Ok(num) = first.parse::<i64>() {
                    return Some(json!(num));
                }
                if let Ok(num) = first.parse::<f64>() {
                    return Some(json!(num));
                }
            }
            Some(Value::String(s.clone()))
        }
        _ => Some(v.clone()),
    }
}

fn harmonize_and_validate_facts(
    facts: &mut Map<String, Value>,
    ai_extraction: &AIExtractionResponse,
) -> HarmonizationResult {
    let mut merged = 0;
    let mut dropped = 0;
    let mut reconciled = 0;

    let mut confidence_map = std::collections::HashMap::new();
    for reasoning in &ai_extraction.ai_reasoning {
        confidence_map.insert(reasoning.fact_key.clone(), reasoning.confidence);
    }

    for (k, raw_v) in &ai_extraction.ai_facts {
        let coerced = match coerce_value(raw_v) {
            Some(val) => val,
            None => continue,
        };

        let confidence = confidence_map.get(k).copied().unwrap_or(0.70);

        // Confidence threshold filtering (< 0.65)
        if confidence < 0.65 && !facts.contains_key(k) {
            dropped += 1;
            continue;
        }

        if let Some(existing_v) = facts.get(k) {
            if existing_v == &coerced {
                merged += 1;
            } else {
                // Conflict resolution: AI takes precedence only if high confidence (>= 0.85)
                if confidence >= 0.85 {
                    facts.insert(k.clone(), coerced);
                    reconciled += 1;
                    merged += 1;
                } else {
                    reconciled += 1;
                }
            }
        } else {
            facts.insert(k.clone(), coerced);
            merged += 1;
        }
    }

    HarmonizationResult {
        merged_count: merged,
        dropped_count: dropped,
        reconciled_count: reconciled,
    }
}

