-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NULL,
    status INT NOT NULL DEFAULT 1,
    created_by UUID NULL,
    updated_by UUID NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ NULL
);

-- indexing
CREATE INDEX idx_categories_active
    ON categories (id)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_categories_code_active
    ON categories (code)
    WHERE deleted_at IS NULL;
