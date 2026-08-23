-- Migration 003: Create email_tokens table (verification + password reset)
CREATE TABLE IF NOT EXISTS email_tokens (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token       TEXT UNIQUE NOT NULL,
    token_type  TEXT NOT NULL CHECK (token_type IN ('verify_email', 'reset_password')),
    expires_at  TIMESTAMPTZ NOT NULL,
    used_at     TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_email_tokens_token ON email_tokens (token);
CREATE INDEX IF NOT EXISTS idx_email_tokens_user_id ON email_tokens (user_id);
