-- Add up migration script here
CREATE TABLE products (
    id BIGSERIAL PRIMARY KEY,
    code VARCHAR(100) NOT NULL UNIQUE,
    category_id BIGSERIAL NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price NUMERIC(10, 2) NOT NULL,
    stock INT NOT NULL DEFAULT 0,
    status INT NOT NULL DEFAULT 1,
    created_by VARCHAR(255) NULL,
    updated_by VARCHAR(255) NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- FK ke master category
ALTER TABLE products
    ADD CONSTRAINT fk_products_category
        FOREIGN KEY (category_id)
            REFERENCES categories (id)
            ON DELETE RESTRICT;

-- =====================
-- Indexes (ACTIVE DATA)
-- =====================

CREATE INDEX idx_products_active
    ON products (id)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_products_category_active
    ON products (category_id)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_products_created_at_active
    ON products (created_at DESC)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_products_status_active
    ON products (status)
    WHERE deleted_at IS NULL;
