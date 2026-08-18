CREATE TABLE password_materials (
    credential_id UUID PRIMARY KEY,
    password_hash TEXT NOT NULL,

    CONSTRAINT fk_password_materials_credential
        FOREIGN KEY (credential_id)
        REFERENCES credentials(id)
        ON DELETE CASCADE
);
