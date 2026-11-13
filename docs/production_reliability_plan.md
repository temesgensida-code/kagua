# Production Reliability & Hardening Implementation Plan for Kagua

## Executive Summary

Kagua is a high-assurance compliance auditing platform combining a **Rust Axum core backend**, a **Python FastAPI / RAG parsing engine**, an **SWI-Prolog symbolic reasoning engine**, and a **SvelteKit 5 frontend**. 

To transition from a development prototype to an enterprise-grade, **production-ready system**, Kagua must eliminate single-point failure modes, thread-safety hazards, memory exhaustion risks, and cold-start fragilities inherent in distributed multi-runtime architectures.

This implementation plan outlines the prioritized architectural interventions, defensive mechanisms, and verification strategies required to achieve **99.9% availability, zero cross-tenant data leakage, sub-second error recovery, and robust resource governance**.

---

## 1. Architectural Risk Matrix & Priorities

| Priority | Component | Failure Mode / Vulnerability | Production Consequence | Mitigation |
| :--- | :--- | :--- | :--- | :--- |
| **P0 - Critical** | **SWI-Prolog Engine** (`engine.pl`) | Global dynamic predicate `user:doc_fact/2` shared across HTTP threads | Concurrent HTTP requests overwrite/retract each other's facts &rarr; cross-tenant data corruption & false positives | Refactor to `:- thread_local user:doc_fact/2.` with guaranteed cleanup via `setup_call_cleanup/3` |
| **P0 - Critical** | **PostgreSQL Connection** (`db.rs`) | Startup `expect()` without connection retry | Server panics and enters crash loop if DB is cold-starting or restarting on cloud hosting | Implement exponential backoff retry loop (5 attempts, 1s–16s) and pool idle reaping |
| **P1 - High** | **Axum Body Limit** (`main.rs`) | Default Axum 0.7 request body limit is 2MB | Scanned PDFs or multi-page contracts (>2MB) fail with silent HTTP 413 Payload Too Large | Configure `DefaultBodyLimit::max(25 * 1024 * 1024)` (25MB) |
| **P1 - High** | **Inter-Service Communication** (`service.rs`) | Ephemeral `reqwest::Client` without connection reuse, timeout, or retry | Cold-starting Render containers (15-45s) cause immediate 502/504 errors | Shared client pool with 10s connect / 90s read timeout and 3-attempt backoff retry |
| **P1 - High** | **Python Parser Memory** (`extractor.py`) | All PDF pages rasterized simultaneously at 250 DPI in RAM | 30+ page scanned PDFs consume >600MB RAM, triggering Linux OOM killer | Iterative page-by-page OCR at 150 DPI, 30-page cap, and explicit memory deallocation |
| **P1 - High** | **Container Dependencies** (`parser/Dockerfile`) | Missing `poppler-utils` and `tesseract-ocr` packages | OCR fallback silently returns empty strings in production containers | Install native Poppler and Tesseract packages in Dockerfile |
| **P2 - Medium** | **Observability & Probes** (`main.rs`) | Dummy `/health` check without dependency verification; no distributed trace ID | Orchestrator routes traffic to dead pods; logs cannot correlate across 3 services | Deep `/health/ready` probe + `x-request-id` propagation middleware |
| **P2 - Medium** | **Graceful Shutdown** (`main.rs`) | Immediate process exit on SIGTERM | In-flight analysis pipelines and database transactions terminated mid-execution | `tokio::signal` SIGTERM handler with 20s connection draining |
| **P2 - Medium** | **Rate Limiting & Security** (`main.rs`) | Unused `tower_governor` dependency | Auth endpoints vulnerable to credential stuffing; `/analyze` vulnerable to DoS | Wire Governor rate limiter on `/auth/*` (5 req/sec burst) and `/analyze` (2 req/min) |

---

## 2. Detailed Technical Specifications

### Phase 1: SWI-Prolog Thread Safety & Concurrency Hardening (P0)

#### Root Cause
In [`reasoner-engine/engine.pl`](file:///home/temesgensida/tom/prolog/Kagua/reasoner-engine/engine.pl#L26), the dynamic predicate is declared globally:
```prolog
:- dynamic user:doc_fact/2.
```
In SWI-Prolog's multithreaded HTTP server (`library(http/thread_httpd)`), all worker threads share the global `user` module. When Request A executes `retract_all_facts` and `assert_facts`, a simultaneous Request B executing `retract_all_facts` wipes Request A's facts mid-execution, resulting in incomplete reasoning or mixed violation reports.

#### Implementation
1. **Thread-Local Declaration**:
   Replace `:- dynamic user:doc_fact/2.` with:
   ```prolog
   :- thread_local user:doc_fact/2.
   ```
   In SWI-Prolog, `thread_local` predicates ensure that asserted clauses are isolated strictly to the calling thread. Multiple threads can assert facts with the same functor without collision. When a thread terminates, its thread-local clauses are automatically reclaimed by the runtime.

2. **Guaranteed Cleanup via `setup_call_cleanup/3`**:
   In `engine.pl`, replace the procedural assertion and cleanup with an atomic transactional block:
   ```prolog
   find_violations(DomainsSpec, Facts, Violations) :-
       setup_call_cleanup(
           ( retract_all_facts, assert_facts(Facts) ),
           (
               resolve_domains(DomainsSpec, Domains),
               load_domains(Domains),
               findall(
                   violation{domain: Domain, rule: Rule, title: Title,
                             severity: Sev, description: Desc, recommendation: Rec},
                   (
                       member(Domain, Domains),
                       domain_module(Domain, Mod),
                       Mod:check_violation(Domain, V),
                       get_dict(rule, V, Rule),
                       get_dict(title, V, Title),
                       get_dict(severity, V, Sev),
                       get_dict(description, V, Desc),
                       get_dict(recommendation, V, Rec)
                   ),
                   ViolationsRaw
               ),
               sort(ViolationsRaw, Violations)
           ),
           retract_all_facts
       ).
   ```

3. **Exception Boundary in HTTP Server**:
   In [`reasoner-engine/server.pl`](file:///home/temesgensida/tom/prolog/Kagua/reasoner-engine/server.pl#L55), enclose the handler inside `catch/3` so syntax errors, invalid terms, or evaluation timeouts yield a structured HTTP 500 JSON response rather than dropping the socket:
   ```prolog
   handle_reason(Request) :-
       cors_enable(Request, [methods([post])]),
       catch(
           process_reason_request(Request),
           Error,
           (
               print_message(error, Error),
               reply_json_dict(
                   _{ status: error, message: "Reasoning query execution failed" },
                   [status(500)]
               )
           )
       ).
   ```

---

### Phase 2: PostgreSQL Connection Resilience & Pool Tuning (P0)

#### Root Cause
In [`core-be/src/db.rs`](file:///home/temesgensida/tom/prolog/Kagua/core-be/src/db.rs#L9-L11), the database connection uses `.expect(...)` immediately:
```rust
let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(database_url)
    .await
    .expect("Failed to connect to PostgreSQL. Check DATABASE_URL in .env");
```
If the database container or managed instance (Render Postgres, Neon, Supabase) is cold-starting, the Rust process crashes immediately.

#### Implementation
1. **Exponential Backoff Connection Loop**:
   Attempt database connection up to 5 times with incremental delays (1s, 2s, 4s, 8s, 16s) before failing.
2. **Connection Pool Lifecycle Tuning**:
   Configure `PgPoolOptions` to survive cloud database idle disconnects:
   - `acquire_timeout`: 15 seconds (fail fast if pool is exhausted instead of hanging).
   - `idle_timeout`: 5 minutes (reap stale TCP connections before the database host terminates them).
   - `max_lifetime`: 30 minutes (prevent long-lived memory or state drift).
   - `min_connections`: 2 (keep warm connections ready for low latency).
   - `max_connections`: 10.

```rust
pub async fn init_db(database_url: &str) -> PgPool {
    info!("Connecting to PostgreSQL database with exponential retry...");
    let mut attempts = 0;
    let max_attempts = 5;
    let mut delay = std::time::Duration::from_secs(1);

    let pool = loop {
        attempts += 1;
        match PgPoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(15))
            .idle_timeout(std::time::Duration::from_secs(300))
            .max_lifetime(std::time::Duration::from_secs(1800))
            .connect(database_url)
            .await
        {
            Ok(p) => {
                info!("PostgreSQL connected successfully on attempt {}", attempts);
                break p;
            }
            Err(e) if attempts < max_attempts => {
                tracing::warn!("DB connect attempt {}/{} failed: {}. Retrying in {:?}...", attempts, max_attempts, e, delay);
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            Err(e) => {
                panic!("Fatal: Failed to connect to PostgreSQL after {} attempts: {}", attempts, e);
            }
        }
    };
    // Run migrations...
}
```

---

### Phase 3: Rust Core Backend HTTP Client Resilience & Cold-Start Retry (P1)

#### Root Cause
In [`core-be/src/service.rs`](file:///home/temesgensida/tom/prolog/Kagua/core-be/src/service.rs#L61), `let client = reqwest::Client::new();` is instantiated per request with no timeouts. When calling Render downstream services that are sleeping, requests fail with 502/504 instantly.

#### Implementation
1. **Shared Client in `AppState`**:
   Initialize a singleton `reqwest::Client` in `main.rs` and store it in `AppState`:
   ```rust
   let http_client = reqwest::Client::builder()
       .connect_timeout(std::time::Duration::from_secs(10))
       .timeout(std::time::Duration::from_secs(90))
       .pool_idle_timeout(std::time::Duration::from_secs(90))
       .tcp_keepalive(std::time::Duration::from_secs(30))
       .build()
       .expect("Failed to build HTTP client");
   ```
2. **Cold-Start Backoff Retry Wrapper**:
   Implement `send_with_retry` for inter-service calls (FastAPI parser and SWI-Prolog reasoner):
   - Automatically retry up to 3 times on `502 Bad Gateway`, `503 Service Unavailable`, `504 Gateway Timeout`, or connection refusal.
   - Delay between retries: 2 seconds, 4 seconds, 8 seconds.
   - Broadcast progress event `DOWNSTREAM_WARMUP` to WebSocket so the user interface displays *"Services waking from cold standby, holding request..."* instead of an abrupt failure.

---

### Phase 4: Request Body Limit & Rate Limiting (P1)

#### Root Cause
Axum 0.7 enforces a default 2MB body limit on incoming requests unless explicitly configured. High-resolution scanned contracts and multi-page agreements frequently exceed 2MB.

#### Implementation
1. **Configure Axum Body Limit**:
   In [`core-be/src/main.rs`](file:///home/temesgensida/tom/prolog/Kagua/core-be/src/main.rs#L101), add `DefaultBodyLimit::max(25 * 1024 * 1024)` (25MB):
   ```rust
   use axum::extract::DefaultBodyLimit;

   let app = Router::new()
       // routes...
       .layer(DefaultBodyLimit::max(25 * 1024 * 1024))
       .layer(cors)
       .with_state(app_state);
   ```
2. **Activate Governor Rate Limiting**:
   Wire up the existing `tower_governor` crate:
   - Public auth endpoints (`/auth/login`, `/auth/register`): 5 requests / second per IP with burst allowance of 10.
   - Document analysis (`/analyze`): 10 requests / minute per user.

---

### Phase 5: Python Parser Memory Safety & Scanned PDF OCR Hardening (P1)

#### Root Cause
1. [`parser/services/extractor.py`](file:///home/temesgensida/tom/prolog/Kagua/parser/services/extractor.py#L23) uses `convert_from_bytes(stream_bytes, dpi=250)`. Rasterizing an entire 30-page PDF in RAM simultaneously allocates over 600MB of uncompressed bitmaps, exceeding the 512MB RAM limit on cloud containers and triggering Linux OOM killer.
2. [`parser/Dockerfile`](file:///home/temesgensida/tom/prolog/Kagua/parser/Dockerfile#L6) does not install `poppler-utils` (`pdftoppm`) or `tesseract-ocr`.

#### Implementation
1. **Install Native Binaries in `parser/Dockerfile`**:
   ```dockerfile
   RUN apt-get update && apt-get install -y --no-install-recommends \
       build-essential \
       poppler-utils \
       tesseract-ocr \
       tesseract-ocr-eng \
       && rm -rf /var/lib/apt/lists/*
   ```
2. **Iterative Memory-Safe OCR Pipeline**:
   In `parser/services/extractor.py`:
   - Downscale OCR rasterization from 250 DPI to **150 DPI** (adequate for legal text recognition; uses 64% less memory).
   - Limit OCR fallback to a maximum of **35 pages** (contracts exceeding 35 pages are truncated with a warning notice).
   - Use `first_page` and `last_page` chunking or iterate page-by-page, explicitly deleting the image object after each page text extraction to trigger immediate Python garbage collection:
   ```python
   def _ocr_pdf_bytes(stream_bytes: bytes, max_pages: int = 35) -> str:
       try:
           from pdf2image import convert_from_bytes
           import pytesseract
           import gc

           # Rasterize at 150 DPI with page cap
           images = convert_from_bytes(stream_bytes, dpi=150, first_page=1, last_page=max_pages)
           pages_text = []
           for img in images:
               text = pytesseract.image_to_string(img, lang="eng")
               if text.strip():
                   pages_text.append(text.strip())
               del img
           del images
           gc.collect()
           return "\n\n".join(pages_text)
       except Exception as e:
           logger.warning(f"OCR rasterization failed: {e}")
           return ""
   ```

---

### Phase 6: Deep Health Checks & Observability Probes (P2)

#### Implementation
1. **Readiness Probe (`GET /health/ready`)**:
   Implement a comprehensive readiness probe in `core-be` that verifies:
   - PostgreSQL connection pool responds to `SELECT 1;`.
   - Python parser responds to `GET /health` within 2 seconds.
   - SWI-Prolog reasoner responds to `GET /health` within 2 seconds.
   - Returns HTTP 200 `{"status": "ready"}` if all three pass, or HTTP 503 `{"status": "degraded", "dependencies": {...}}` if any fail.
2. **Liveness Probe (`GET /health/live`)**:
   Returns HTTP 200 immediately to confirm the Axum event loop is responsive.
3. **Distributed Request ID (`x-request-id`)**:
   Add request correlation middleware in Axum using `tower_http::request_id::SetRequestIdLayer`. Forward `x-request-id` in downstream HTTP headers to Python and Prolog so errors can be correlated end-to-end across service logs.

---

### Phase 7: Graceful Shutdown & Connection Draining (P2)

#### Implementation
In `core-be/src/main.rs`, bind Axum with a graceful shutdown signal listener:
```rust
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Termination signal received. Draining connections...");
}
```
Apply to server:
```rust
axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Axum server error");
```

---

### Phase 8: Frontend Network Resilience & Auto-Reconnect (P2)

#### Implementation
In [`ui/src/lib/services/api.ts`](file:///home/temesgensida/tom/prolog/Kagua/ui/src/lib/services/api.ts#L172):
1. **WebSocket Exponential Backoff**: Add auto-reconnection with jitter (1s, 2s, 4s, 8s, up to 15s) so transient network disconnects do not permanently break progress tracking.
2. **Request Timeout & AbortController**: Wrap `analyzeDocument` in an `AbortController` with a 120-second timeout, providing a user-friendly error dialog if the request times out.

---

## 3. Implementation Phasing & Work Breakdown

```mermaid
flowchart TD
    subgraph Phase 1 [Phase 1: Concurrency & Database Safety]
        P1A["Prolog thread_local user:doc_fact/2"]
        P1B["Prolog setup_call_cleanup/3 transactional wrapping"]
        P1C["PostgreSQL exponential retry loop & pool tuning"]
    end

    subgraph Phase 2 [Phase 2: Network & Resource Governance]
        P2A["Axum DefaultBodyLimit: 25MB"]
        P2B["Shared Reqwest client with 10s/90s timeouts"]
        P2C["Inter-service 3-attempt backoff retry"]
        P2D["Governor rate limiting on /auth and /analyze"]
    end

    subgraph Phase 3 [Phase 3: Parser Hardening & Memory Safety]
        P3A["Add poppler-utils and tesseract-ocr to Dockerfile"]
        P3B["Iterative 150 DPI page-by-page OCR with 35-page cap"]
        P3C["Explicit garbage collection deallocation"]
    end

    subgraph Phase 4 [Phase 4: Observability & Operational Resilience]
        P4A["Deep /health/ready probe for DB + Parser + Reasoner"]
        P4B["x-request-id tracing propagation across microservices"]
        P4C["SIGTERM graceful shutdown connection draining"]
        P4D["Frontend WebSocket auto-reconnect with backoff"]
    end

    Phase 1 --> Phase 2
    Phase 2 --> Phase 3
    Phase 3 --> Phase 4
```

---

## 4. Verification & Testing Strategy

1. **Concurrent Load Test (Prolog Thread Isolation)**:
   - Run a test firing 10 concurrent requests with contrasting facts (Contract A: 90 probation days; Contract B: 30 probation days).
   - Verify that Contract B never receives a probation violation and Contract A always does (0 cross-talk).
2. **Cold-Start Simulation Test**:
   - Stop or delay downstream parser and Prolog containers.
   - Verify that `core-be` retries with backoff and reports `DOWNSTREAM_WARMUP` over WebSocket rather than immediately failing with 502.
3. **Database Disconnect & Reconnect Test**:
   - Restart the PostgreSQL container during `core-be` launch.
   - Verify that `core-be` loops through retry attempts and successfully connects without crashing.
4. **Large File & OOM Stress Test**:
   - Upload a 15MB 40-page scanned PDF.
   - Verify that Axum accepts the payload (>2MB) without HTTP 413, and the Python parser stays below 250MB RAM usage during OCR.
5. **Readiness Probe Verification**:
   - Query `GET /health/ready` with services up &rarr; HTTP 200.
   - Temporarily kill SWI-Prolog &rarr; `GET /health/ready` returns HTTP 503 degraded.
