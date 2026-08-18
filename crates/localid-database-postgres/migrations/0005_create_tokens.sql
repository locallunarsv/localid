CREATE TABLE tokens (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL,
    secret_hash TEXT UNIQUE NOT NULL,
    lifecycle_state TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_tokens_session_id
ON tokens(session_id);
