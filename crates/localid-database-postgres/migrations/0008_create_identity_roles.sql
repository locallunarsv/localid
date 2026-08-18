CREATE TABLE roles (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE permissions (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE role_permissions (
    role_id UUID NOT NULL,
    permission_id UUID NOT NULL,

    PRIMARY KEY (role_id, permission_id),

    CONSTRAINT fk_role_permissions_role
        FOREIGN KEY (role_id)
        REFERENCES roles(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_role_permissions_permission
        FOREIGN KEY (permission_id)
        REFERENCES permissions(id)
        ON DELETE CASCADE
);

CREATE TABLE identity_roles (
    identity_id UUID NOT NULL,
    role_id UUID NOT NULL,

    PRIMARY KEY (identity_id, role_id),

    CONSTRAINT fk_identity_roles_identity
        FOREIGN KEY (identity_id)
        REFERENCES identities(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_identity_roles_role
        FOREIGN KEY (role_id)
        REFERENCES roles(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_identity_roles_identity_id
ON identity_roles(identity_id);
