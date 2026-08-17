CREATE TABLE oauth_clients (
    id UUID PRIMARY KEY,
    local_client_id UUID NOT NULL,

    client_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,

    secret_hash TEXT NOT NULL,

    redirect_uris JSONB NOT NULL,

    state TEXT NOT NULL CHECK (
        state IN ('active', 'disabled', 'deleted')
    ),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_oauth_clients_local_client_id
    ON oauth_clients(local_client_id);
