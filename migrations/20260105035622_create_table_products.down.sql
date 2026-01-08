-- Add down migration script here
-- Drop indexes
DROP INDEX IF EXISTS idx_products_category_active;
DROP INDEX IF EXISTS idx_products_active;
DROP INDEX IF EXISTS idx_products_created_at_active;
DROP INDEX IF EXISTS idx_products_status_active;

-- Drop table
DROP TABLE IF EXISTS products;