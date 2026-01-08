-- Add up migration script here
CREATE TABLE categories (
    id BIGSERIAL PRIMARY KEY,
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NULL,
    status INT NOT NULL DEFAULT 1,
    created_by VARCHAR(255) NULL,
    updated_by VARCHAR(255) NULL,
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
