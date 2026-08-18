CREATE TABLE clients (
    id UUID PRIMARY KEY,
    client_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    state TEXT NOT NULL
);
