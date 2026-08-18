CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    identity_id UUID NOT NULL,
    client_id UUID NOT NULL,
    lifecycle_state TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_sessions_identity_id
ON sessions(identity_id);
