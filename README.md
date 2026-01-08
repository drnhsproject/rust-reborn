# 🦀 Rust-Reborn Framework

A modern, scalable web framework built with Rust, implementing Domain-Driven Design (DDD) and Clean Architecture principles. Designed to be monolithic-first but microservices-ready.

## ✨ Features

- 🏗️ **Clean Architecture** - Separation of concerns with clear boundaries
- 🎯 **Domain-Driven Design** - Business logic at the core
- 🔐 **Authentication System** - JWT-based auth with password hashing
- 📦 **Package-based Structure** - Easy to extract into microservices
- 🚀 **High Performance** - Built with Axum and Tokio
- 🔍 **Observability** - Structured logging with tracing
- 🛡️ **Type Safety** - Leveraging Rust's type system
- 📝 **OpenAPI Ready** - Easy API documentation

## 📁 Project Structure

```
rust-reborn/
├── packages/             # Domain packages
│   ├── core/             # Core utilities
│   ├── auth/             # Authentication domain
│   └── media/            # Media management (future)
├── services/             # Runnable services
│   └── api_server/       # Main API service
├── shared/               # Shared libraries
│   └── contracts/        # API contracts
├── migrations/           # Database migrations
├── config/               # Configuration files
└── docker/               # Docker configurations
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 14+ ([Install PostgreSQL](https://www.postgresql.org/download/))
- Docker & Docker Compose (optional)

### 1. Clone the Repository

```bash
git clone git@github.com:drnhsproject/rust-reborn.git
cd rust-reborn
```

### 2. Setup Database

Using Docker Compose (recommended):

```bash
docker-compose up -d postgres
```

Or install PostgreSQL locally and create database:

```bash
createdb rust_reborn
```

### 3. Configure Environment

```bash
cp .env.example .env
# Edit .env with your configuration
```

### 4. Run Migrations

install sqlx-cli in your terminal:

```bash
cargo install sqlx-cli
sqlx migrate run
```

### 5. Run the Server

```bash
cargo run
```

The server will start at `http://localhost:8000`

if you want using watcher, you can install:

```bash
cargo install cargo-watch
```

and run using:

```bash
cargo watch -x "run"
```

## 📚 API Documentation

### Authentication Endpoints

#### Register User

```bash
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "SecurePass123!",
  "full_name": "John Doe"
}
```

**Response:**

```json
{
  "success": true,
  "message": "your account registered successfully",
  "data": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "is_verified": false
  }
}
```

#### Login

```bash
POST /api/auth/login
Content-Type: application/json

{
  "username": "user@example.com",  # or can using e-mail
  "password": "SecurePass123!"
}
```
**Response:**

```json
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "is_verified": false
  },
  "token": {
    "access_token": "token",
    "token_type": "Bearer",
    "expires_in": 123,
    "refresh_token": null
  }
}
```

#### Get Current User

```bash
GET /api/auth/me
Authorization: Bearer <token>
```

#### Logout

```bash
POST /api/auth/logout
Authorization: Bearer <token>
```

## 🏗️ Architecture

### Clean Architecture Layers

```
┌─────────────────────────────────────┐
│         Presentation Layer          │  <- HTTP Handlers, Routes
├─────────────────────────────────────┤
│         Application Layer           │  <- Use Cases, DTOs
├─────────────────────────────────────┤
│           Domain Layer              │  <- Entities, Value Objects
├─────────────────────────────────────┤
│       Infrastructure Layer          │  <- Database, External APIs
└─────────────────────────────────────┘
```

### Dependency Flow

- **Presentation** depends on **Application**
- **Application** depends on **Domain**
- **Infrastructure** implements **Domain** interfaces
- **Domain** has no dependencies (pure business logic)

## 🔧 Development

### Running Tests

```bash
cargo test
```

### Running with Hot Reload

```bash
cargo install cargo-watch
cargo watch -x 'run --bin api_server'
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Formatting

```bash
cargo fmt
```

## 📦 Adding New Features

### Creating a New Package

1. Create package structure:

```bash
mkdir -p packages/my-feature/src/{domain,application,infrastructure,presentation}
```

2. Add to workspace in root `Cargo.toml`:

```toml
[workspace]
members = [
    # ... existing members
    "packages/my-feature",
]
```

3. Implement DDD layers following the auth package pattern

### Adding to API Server

In `services/api_server/src/routes.rs`:

```rust
pub fn my_feature_routes(state: MyFeatureState) -> Router {
    rust_reborn_my_feature::http::create_routes(state)
}
```

In `services/api_server/src/main.rs`:

```rust
let app = Router::new()
    .nest("/api/v1/my-feature", routes::my_feature_routes(state))
    // ... other routes
```

## 🚢 Deployment

### Building for Production

```bash
cargo build --release --bin api_server
```

### Docker Build

```bash
docker build -f docker/api_server.Dockerfile -t rust-reborn-api .
```

### Environment Variables

See `.env.example` for all available configuration options.

## 🔮 Roadmap

- [x] Core framework setup
- [x] Authentication system
- [ ] Media management
- [ ] Role-based access control (RBAC)
- [ ] API rate limiting
- [ ] WebSocket support
- [ ] GraphQL API
- [ ] Microservices extraction
- [ ] Service mesh integration
- [ ] Monitoring dashboard

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum)
- Inspired by Clean Architecture principles
- Powered by the Rust community

---

**Made with ❤️ and 🦀 by the Rust-Reborn Team**
