# Routes Structure

Dokumentasi struktur routing di Rust-Reborn API Server.

## Overview

API Server menggunakan modular routing pattern dimana setiap domain (auth, product, dll) memiliki routes sendiri yang didefinisikan di package masing-masing.

## Route Modules

### 1. Auth Routes (`/api/auth/*`)

**Location**: `packages/auth/src/presentation/http/routes.rs`

**Endpoints**:

- `POST /api/auth/register` - Register user baru
- `POST /api/auth/login` - Login dan dapatkan JWT token
- `POST /api/auth/logout` - Logout (client-side)
- `GET /api/auth/me` - Get current user info (requires auth)

**Authentication**: Tidak perlu auth kecuali `/me`

### 2. Product Routes (`/api/products/*`)

**Location**: `packages/core/src/handlers.rs`

**Endpoints**:

- `GET /api/products` - List semua products (public)
- `GET /api/products/:id` - Get product by ID (public)
- `POST /api/products` - Create product baru (requires auth)

**Authentication**:

- GET endpoints: Public, tidak perlu auth
- POST endpoint: Protected, harus login

## Route Registration

Routes didaftarkan di `services/api_server/src/main.rs`:

```rust
let app = Router::new()
    .merge(db_routes)
    .nest("/api/auth", routes::auth_routes(auth_state.clone()))
    .nest("/api", routes::product_routes(auth_state))
    .layer(TraceLayer::new_for_http())
    .layer(cors);
```

### Penjelasan:

1. **`nest("/api/auth", ...)`**

   - Semua routes dari `auth_routes()` akan di-prefix dengan `/api/auth`
   - Contoh: route `/register` menjadi `/api/auth/register`

2. **`nest("/api", ...)`**
   - Semua routes dari `product_routes()` akan di-prefix dengan `/api`
   - Contoh: route `/products` menjadi `/api/products`

## Adding New Routes

### Step 1: Create Handler Functions

Di package yang sesuai (misal `packages/core/src/handlers.rs`):

```rust
use rust_reborn_contracts::{AuthUser, Result};
use axum::{Json, response::IntoResponse};

// Public endpoint
pub async fn list_items() -> Result<impl IntoResponse> {
    // ...
}

// Protected endpoint
pub async fn create_item(
    AuthUser(user_id): AuthUser,  // Requires auth
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse> {
    // user_id is guaranteed to be valid here
    // ...
}
```

### Step 2: Create Routes Function

```rust
use axum::{routing::{get, post}, Router, middleware};
use rust_reborn_auth::{auth_middleware, AuthState};

pub fn create_item_routes(auth_state: AuthState) -> Router {
    Router::new()
        // Public routes
        .route("/items", get(list_items))

        // Protected routes
        .route(
            "/items",
            post(create_item)
                .layer(middleware::from_fn_with_state(
                    auth_state,
                    auth_middleware,
                )),
        )
}
```

### Step 3: Export from Package

Di `packages/core/src/lib.rs`:

```rust
pub mod handlers;
pub use handlers::create_item_routes;
```

### Step 4: Register in API Server

Di `services/api_server/src/routes.rs`:

```rust
pub fn item_routes(auth_state: AuthState) -> Router {
    rust_reborn_core::create_item_routes(auth_state)
}
```

Di `services/api_server/src/main.rs`:

```rust
let app = Router::new()
    .merge(db_routes)
    .nest("/api/auth", routes::auth_routes(auth_state.clone()))
    .nest("/api", routes::product_routes(auth_state.clone()))
    .nest("/api", routes::item_routes(auth_state))  // Add new routes
    .layer(TraceLayer::new_for_http())
    .layer(cors);
```

## Authentication Patterns

### Pattern 1: Public Endpoint (No Auth)

```rust
pub async fn public_handler() -> Result<impl IntoResponse> {
    // Anyone can access
    Ok(Json("public data"))
}

// In routes
Router::new()
    .route("/public", get(public_handler))
```

### Pattern 2: Protected Endpoint (Auth Required)

```rust
use rust_reborn_contracts::AuthUser;

pub async fn protected_handler(
    AuthUser(user_id): AuthUser,  // Will reject if not authenticated
) -> Result<impl IntoResponse> {
    // user_id is guaranteed to exist
    Ok(Json(format!("Hello user {}", user_id)))
}

// In routes
Router::new()
    .route(
        "/protected",
        get(protected_handler)
            .layer(middleware::from_fn_with_state(
                auth_state,
                auth_middleware,
            )),
    )
```

### Pattern 3: Optional Auth (Works with or without auth)

```rust
use rust_reborn_contracts::OptionalAuthUser;

pub async fn optional_auth_handler(
    OptionalAuthUser(user_id): OptionalAuthUser,
) -> Result<impl IntoResponse> {
    match user_id {
        Some(id) => Ok(Json(format!("Hello user {}", id))),
        None => Ok(Json("Hello anonymous user")),
    }
}

// In routes - no middleware needed
Router::new()
    .route("/optional", get(optional_auth_handler))
```

### Pattern 4: Group Protected Routes

Jika banyak routes yang perlu auth, gunakan layer di Router level:

```rust
// Public routes
let public_routes = Router::new()
    .route("/items", get(list_items))
    .route("/items/:id", get(get_item));

// Protected routes
let protected_routes = Router::new()
    .route("/items", post(create_item))
    .route("/items/:id", put(update_item))
    .route("/items/:id", delete(delete_item))
    .layer(middleware::from_fn_with_state(
        auth_state,
        auth_middleware,
    ));

// Merge
public_routes.merge(protected_routes)
```

## Middleware Order

Middleware dijalankan dari **luar ke dalam** (bottom to top):

```rust
Router::new()
    .route("/api/products", post(create_product))
    .layer(auth_middleware)      // 2. Runs second
    .layer(TraceLayer::new())    // 1. Runs first
    .layer(cors);                // 0. Runs first (outermost)
```

Order eksekusi:

1. CORS layer
2. Trace layer
3. Auth middleware
4. Handler

## State Management

Setiap route function bisa menerima state:

```rust
use axum::extract::State;

pub async fn handler(
    State(auth_state): State<AuthState>,
    AuthUser(user_id): AuthUser,
) -> Result<impl IntoResponse> {
    // Access auth_state.auth_service
    // ...
}
```

## Error Handling

Semua handlers return `Result<impl IntoResponse>` dimana:

- `Result` adalah `rust_reborn_contracts::Result`
- Error type adalah `AppError` yang otomatis dikonversi ke HTTP response

```rust
use rust_reborn_contracts::{AppError, Result};

pub async fn handler() -> Result<impl IntoResponse> {
    // Return error
    return Err(AppError::not_found("Item not found"));

    // Or success
    Ok(Json(data))
}
```

Error codes yang tersedia:

- `AppError::unauthorized(msg)` → 401
- `AppError::forbidden(msg)` → 403
- `AppError::not_found(msg)` → 404
- `AppError::bad_request(msg)` → 400
- `AppError::internal_server_error(msg)` → 500

## Best Practices

1. **Organize by Domain**: Setiap domain (auth, product, media) punya package sendiri
2. **Use Extractors**: Gunakan `AuthUser` untuk protected endpoints
3. **Validate Input**: Gunakan `validate()` dari contracts untuk validasi
4. **Consistent Responses**: Gunakan helper dari `contracts::common::response`
5. **Document Endpoints**: Tambahkan komentar di handler functions
6. **Test Routes**: Buat integration tests untuk setiap route

## Example: Complete CRUD Routes

```rust
use axum::{
    routing::{get, post, put, delete},
    Router, middleware, Json,
};
use rust_reborn_auth::{auth_middleware, AuthState};
use rust_reborn_contracts::{AuthUser, Result, validation::validate};

// Handlers
pub async fn list(OptionalAuthUser(user_id): OptionalAuthUser) -> Result<impl IntoResponse> {
    // List logic
}

pub async fn get_by_id(Path(id): Path<Uuid>) -> Result<impl IntoResponse> {
    // Get logic
}

pub async fn create(
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateRequest>,
) -> Result<impl IntoResponse> {
    validate(&payload)?;
    // Create logic
}

pub async fn update(
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRequest>,
) -> Result<impl IntoResponse> {
    validate(&payload)?;
    // Update logic
}

pub async fn delete_item(
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse> {
    // Delete logic
}

// Routes
pub fn create_routes(auth_state: AuthState) -> Router {
    let public = Router::new()
        .route("/items", get(list))
        .route("/items/:id", get(get_by_id));

    let protected = Router::new()
        .route("/items", post(create))
        .route("/items/:id", put(update))
        .route("/items/:id", delete(delete_item))
        .layer(middleware::from_fn_with_state(
            auth_state,
            auth_middleware,
        ));

    public.merge(protected)
}
```

## Testing Routes

See [TESTING_API.md](./TESTING_API.md) for complete testing guide.
