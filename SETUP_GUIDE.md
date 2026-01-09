# 🚀 Rust-Reborn Setup Guide

Panduan lengkap untuk setup dan menjalankan Rust-Reborn framework dari nol.

## 📋 Prerequisites

Pastikan tools berikut sudah terinstall:

```bash
# Check Rust
rustc --version
# Should be 1.75.0 or higher

# Check Cargo
cargo --version

# Check PostgreSQL (optional jika pakai Docker)
psql --version

# Check Docker (optional)
docker --version
docker-compose --version
```

Jika belum ada:

- **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Docker**: https://docs.docker.com/get-docker/

## 📁 Complete Folder Structure

Berikut struktur lengkap yang perlu dibuat:

```
rust-reborn/
│
├── Cargo.toml                          # Workspace root
├── .env.example                        # Environment template
├── .env                                # Your environment (gitignored)
├── .gitignore
├── Makefile                            # Development commands
├── README.md
├── docker-compose.yml
│
├── config/
│   ├── default.yaml                    # Default config
│   ├── development.yaml                # Dev overrides (optional)
│   └── production.yaml                 # Prod config (gitignored)
│
├── migrations/                         # Database migrations
│   └── 20240101000001_create_users_table.sql
│
├── packages/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs
│   │       ├── error.rs
│   │       ├── telemetry.rs
│   │       ├── validation.rs
│   │       └── utils/
│   │           ├── mod.rs
│   │           ├── pagination.rs
│   │           └── response.rs
│   │
│   ├── auth/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── domain/
│   │       │   ├── mod.rs
│   │       │   ├── entities/
│   │       │   │   ├── mod.rs
│   │       │   │   └── user.rs
│   │       │   ├── repositories/
│   │       │   │   ├── mod.rs
│   │       │   │   └── user_repository.rs
│   │       │   └── value_objects/
│   │       │       └── mod.rs
│   │       │
│   │       ├── application/
│   │       │   ├── mod.rs
│   │       │   ├── dto/
│   │       │   │   └── mod.rs
│   │       │   └── services/
│   │       │       └── auth_service.rs
│   │       │
│   │       ├── infrastructure/
│   │       │   ├── mod.rs
│   │       │   ├── jwt.rs
│   │       │   ├── password.rs
│   │       │   └── repositories/
│   │       │       ├── mod.rs
│   │       │       └── postgres_user_repository.rs
│   │       │
│   │       └── presentation/
│   │           ├── mod.rs
│   │           ├── http/
│   │           │   ├── mod.rs
│   │           │   ├── handlers.rs
│   │           │   └── routes.rs
│   │           └── middleware/
│   │               ├── mod.rs
│   │               └── auth_middleware.rs
│   │
│   └── media/                          # Future feature
│       └── (similar structure to auth)
│
├── services/
│   └── api_server/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           └── routes.rs
│
├── shared/
│   └── contracts/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
│
└── docker/
    └── api_server.Dockerfile           # Future Dockerfile
```

## 🔧 Step-by-Step Setup

### Step 1: Create Project Structure

```bash
# Create main directory
mkdir rust-reborn && cd rust-reborn

# Create all directories
mkdir -p packages/{core,auth,media}/src
mkdir -p services/api_server/src
mkdir -p shared/contracts/src
mkdir -p config migrations docker

# Create package subdirectories for auth
mkdir -p packages/auth/src/{domain,application,infrastructure,presentation}
mkdir -p packages/auth/src/domain/{entity,repository,value_objects}
mkdir -p packages/auth/src/application/{dto,services}
mkdir -p packages/auth/src/infrastructure/repository
mkdir -p packages/auth/src/presentation/{http,middleware}

# Create core subdirectories
mkdir -p packages/core/src/utils
```

### Step 2: Copy All Files

Salin semua file yang sudah saya generate di atas ke lokasi masing-masing:

1. Root files: `Cargo.toml`, `.env.example`, `.gitignore`, `Makefile`, `README.md`, `docker-compose.yml`
2. Config: `config/default.yaml`
3. Migrations: `migrations/20240101000001_create_users_table.sql`
4. Core package files
5. Auth package files
6. API server files

### Step 3: Initialize Git

```bash
git init
git add .
git commit -m "Initial commit: Rust-Reborn framework setup"
```

### Step 4: One-Command Setup

```bash
make setup
```

Ini akan:
- Copy `.env.example` ke `.env`
- Install `sqlx-cli`
- Start Docker PostgreSQL
- Run migrations

### Step 5: Run Development Server

```bash
make dev
```

Server akan jalan di `http://localhost:8000` dengan hot reload!

## 🧪 Testing the API

### 1. Register a New User

```bash
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "SecurePass123!",
    "full_name": "Test User"
  }'
```

### 2. Login

```bash
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "login": "test@example.com",
    "password": "SecurePass123!"
  }'
```

Save the `access_token` from response!

### 3. Get Current User

```bash
curl -X GET http://localhost:8000/api/auth/me \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

## 📝 Available Make Commands

```bash
make help           # Show all commands
make setup          # Initial setup
make dev            # Run with hot reload
make build          # Build release
make test           # Run tests
make lint           # Run clippy
make fmt            # Format code
make docker-up      # Start Docker services
make docker-down    # Stop Docker services
make migrate        # Run migrations
make db-reset       # Reset database
```

## 🐛 Troubleshooting

### Database Connection Failed

```bash
# Check if PostgreSQL is running
docker-compose ps

# Restart PostgreSQL
docker-compose restart postgres

# Check logs
docker-compose logs postgres
```

### Migration Errors

```bash
# Reset database and re-run migrations
make db-reset
```

### Port Already in Use

Edit `.env` and change:

```bash
APP__SERVER__PORT=8080  # or any available port
```

### Hot Reload Not Working

```bash
# Install cargo-watch
cargo install cargo-watch

# Then run
make dev
```

## 🎯 Next Steps

After basic setup:

1. **Add Media Package** - Similar structure to auth
2. **Implement RBAC** - Role-based access control
3. **Add API Documentation** - Using OpenAPI/Swagger
4. **Setup CI/CD** - GitHub Actions or GitLab CI
5. **Add Integration Tests** - Test API endpoints
6. **Implement Caching** - Redis integration
7. **Add Rate Limiting** - Protect your APIs
8. **Setup Monitoring** - Prometheus + Grafana

## 📚 Learning Resources

- **Axum Documentation**: https://docs.rs/axum/
- **SQLx Guide**: https://github.com/launchbadge/sqlx
- **DDD in Rust**: Clean Architecture principles
- **Rust Book**: https://doc.rust-lang.org/book/

## 💡 Tips

1. **Use `make dev`** for development - it has hot reload
2. **Check logs** with `RUST_LOG=debug cargo run`
3. **Test endpoints** with Postman or Thunder Client (VS Code)
4. **Read error messages** - Rust compiler is very helpful
5. **Use `cargo check`** for fast feedback while coding

---

Happy coding! 🦀✨