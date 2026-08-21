CREATE TABLE authorization_codes (
    id UUID PRIMARY KEY,

    oauth_client_id UUID NOT NULL,
    identity_id UUID NOT NULL,

    code_hash TEXT UNIQUE NOT NULL,

    redirect_uri TEXT NOT NULL,

    nonce TEXT,

    scope JSONB NOT NULL,

    request_state TEXT,

    pkce_challenge TEXT,
    pkce_method TEXT,

    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,

    state TEXT NOT NULL CHECK (
        state IN ('active', 'consumed')
    )
);

CREATE INDEX idx_authorization_codes_oauth_client_id
    ON authorization_codes(oauth_client_id);

CREATE INDEX idx_authorization_codes_identity_id
    ON authorization_codes(identity_id);

CREATE INDEX idx_authorization_codes_code_hash
    ON authorization_codes(code_hash);
