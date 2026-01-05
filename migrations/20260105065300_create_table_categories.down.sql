-- Add down migration script here
-- Drop indexes
DROP INDEX IF EXISTS idx_categories_deleted_at;
DROP INDEX IF EXISTS idx_categories_status;
DROP INDEX IF EXISTS idx_categories_active;
DROP INDEX IF EXISTS idx_categories_code_active;

DROP TABLE IF EXISTS categories;
DROP EXTENSION IF EXISTS "uuid-ossp";
