.PHONY: help setup dev build test clean docker-up docker-down migrate

help:
	@echo "Rust-Reborn Framework - Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

setup: ## Initial project setup
	@echo "🔧 Setting up Rust-Reborn..."
	@cp .env.example .env || true
	@cargo install sqlx-cli --no-default-features --features postgres
	@docker-compose up -d postgres
	@sleep 5
	@sqlx migrate run
	@echo "✅ Setup complete!"

dev: ## Run development server with hot reload
	@cargo watch -x 'run --bin api-server' || cargo run --bin api-server

build: ## Build release binary
	@cargo build --release --bin api-server

run: ## Run the API server
	@cargo run --bin api-server

test: ## Run all tests
	@cargo test

clean: ## Clean build artifacts
	@cargo clean

docker-up: ## Start Docker services
	@docker-compose up -d

docker-down: ## Stop Docker services
	@docker-compose down

migrate: ## Run database migrations
	@sqlx migrate run

db-reset: ## Reset database
	@docker-compose down -v
	@docker-compose up -d postgres
	@sleep 5
	@sqlx migrate run

lint: ## Run clippy
	@cargo clippy -- -D warnings

fmt: ## Format code
	@cargo fmt
