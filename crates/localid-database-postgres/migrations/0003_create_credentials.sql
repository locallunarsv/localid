CREATE TABLE credentials (
    id UUID PRIMARY KEY,
    identity_id UUID NOT NULL,
    kind TEXT NOT NULL,
    lifecycle_state TEXT NOT NULL
);

CREATE INDEX idx_credentials_identity_id
ON credentials(identity_id);
