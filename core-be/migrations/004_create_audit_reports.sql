-- Migration 004: Create audit_reports table for persistent compliance audit storage
CREATE TABLE IF NOT EXISTS audit_reports (
    id                 UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id            UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    document_name      TEXT NOT NULL,
    document_hash      TEXT NOT NULL,
    domain             TEXT NOT NULL,
    total_violations   INT NOT NULL DEFAULT 0,
    critical_count     INT NOT NULL DEFAULT 0,
    warning_count      INT NOT NULL DEFAULT 0,
    compliance_score   DOUBLE PRECISION NOT NULL DEFAULT 100.00,
    pii_redacted_count INT NOT NULL DEFAULT 0,
    report_payload     JSONB NOT NULL,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indices for fast lookups and user isolation
CREATE INDEX IF NOT EXISTS idx_audit_reports_user_id ON audit_reports(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_reports_created_at ON audit_reports(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_reports_domain ON audit_reports(domain);

-- GIN Index for deep querying on JSONB violation attributes
CREATE INDEX IF NOT EXISTS idx_audit_reports_payload_gin ON audit_reports USING gin (report_payload jsonb_path_ops);
