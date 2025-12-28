# Authentication Middleware

Middleware authentication untuk melindungi endpoint yang memerlukan login.

## Komponen

### 1. Middleware

#### `auth_middleware`

Middleware yang **wajib** memverifikasi JWT token. Jika token tidak valid atau tidak ada, request akan ditolak dengan error 401 Unauthorized.

#### `optional_auth_middleware`

Middleware yang **opsional** memverifikasi JWT token. Jika token ada dan valid, user_id akan disimpan. Jika tidak ada atau tidak valid, request tetap dilanjutkan tanpa user_id.

### 2. Extractors

#### `AuthUser(Uuid)`

Extractor untuk mendapatkan user ID yang sudah terverifikasi. **Akan error jika user tidak login**.

```rust
pub async fn create_product(
    AuthUser(user_id): AuthUser,  // Otomatis reject jika tidak login
    Json(payload): Json<CreateProductRequest>,
) -> Result<impl IntoResponse> {
    // user_id sudah pasti valid
    println!("User {} is creating product", user_id);
    // ...
}
```

#### `OptionalAuthUser(Option<Uuid>)`

Extractor untuk mendapatkan user ID jika ada. **Tidak akan error jika user tidak login**.

```rust
pub async fn list_products(
    OptionalAuthUser(user_id): OptionalAuthUser,
) -> Result<impl IntoResponse> {
    if let Some(user_id) = user_id {
        println!("Authenticated user {} viewing products", user_id);
    } else {
        println!("Anonymous user viewing products");
    }
    // ...
}
```

## Cara Penggunaan

### Metode 1: Menggunakan Layer Middleware (Recommended untuk route tertentu)

Gunakan ini jika hanya **beberapa route** yang perlu dilindungi:

```rust
use axum::{middleware, routing::post, Router};
use rust_reborn_auth::{auth_middleware, AuthUser};

fn create_routes(auth_state: AuthState) -> Router {
    Router::new()
        // Public routes - tidak perlu login
        .route("/products", get(list_products))

        // Protected route - harus login
        .route(
            "/products",
            post(create_product)
                .layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    auth_middleware,
                )),
        )
}
```

### Metode 2: Menggunakan Route Layer (Untuk grup routes)

Gunakan ini jika **banyak routes** yang perlu dilindungi:

```rust
use axum::{middleware, routing::{get, post}, Router};
use rust_reborn_auth::{auth_middleware, AuthUser};

fn create_routes(auth_state: AuthState) -> Router {
    // Public routes
    let public_routes = Router::new()
        .route("/products", get(list_products))
        .route("/products/:id", get(get_product));

    // Protected routes - semua route di sini perlu login
    let protected_routes = Router::new()
        .route("/products", post(create_product))
        .route("/products/:id", put(update_product))
        .route("/products/:id", delete(delete_product))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            auth_middleware,
        ));

    // Gabungkan
    public_routes.merge(protected_routes)
}
```

### Metode 3: Menggunakan Extractor Saja (Tanpa Middleware)

Gunakan ini jika ingin **lebih fleksibel** dalam handling auth di level handler:

```rust
use rust_reborn_auth::AuthUser;

// Tidak perlu middleware layer, cukup gunakan AuthUser extractor
pub async fn create_product(
    AuthUser(user_id): AuthUser,  // Ini akan otomatis reject jika tidak login
    Json(payload): Json<CreateProductRequest>,
) -> Result<impl IntoResponse> {
    // ...
}
```

**Note**: Metode ini tetap memerlukan middleware untuk menyimpan user_id ke request extensions. Jadi tetap perlu apply middleware di route atau router level.

## Contoh Lengkap

### Auth Routes (Login/Register - Tidak Perlu Auth)

```rust
// packages/auth/src/presentation/http/routes.rs
use axum::{routing::post, Router};

pub fn create_routes() -> Router<AuthState> {
    Router::new()
        .route("/auth/register", post(register))  // Public
        .route("/auth/login", post(login))        // Public
        .route("/auth/logout", post(logout))      // Public (stateless JWT)
        .route("/auth/me", get(get_current_user)) // Bisa pakai AuthUser extractor
}
```

### Product Routes (Create Perlu Auth, List/Get Tidak)

```rust
// packages/core/src/handlers.rs
use axum::{middleware, routing::{get, post}, Router};
use rust_reborn_auth::{auth_middleware, AuthUser, OptionalAuthUser};

pub fn create_product_routes(auth_state: AuthState) -> Router {
    Router::new()
        // Public - tidak perlu login
        .route("/products", get(list_products))
        .route("/products/:id", get(get_product))

        // Protected - harus login
        .route(
            "/products",
            post(create_product)
                .layer(middleware::from_fn_with_state(
                    auth_state,
                    auth_middleware,
                )),
        )
}

// Handler untuk create - PERLU LOGIN
pub async fn create_product(
    AuthUser(user_id): AuthUser,  // Otomatis reject jika tidak login
    Json(payload): Json<CreateProductRequest>,
) -> Result<impl IntoResponse> {
    // user_id pasti ada dan valid
    let product = create_product_in_db(payload, user_id).await?;
    Ok(created(product))
}

// Handler untuk list - TIDAK PERLU LOGIN
pub async fn list_products(
    OptionalAuthUser(user_id): OptionalAuthUser,
) -> Result<impl IntoResponse> {
    // user_id mungkin ada, mungkin tidak
    let products = get_products_from_db(user_id).await?;
    Ok(Json(products))
}
```

## Testing dengan cURL

### 1. Register User

```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "name": "John Doe"
  }'
```

### 2. Login

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

Response akan berisi token:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "...",
  "token_type": "Bearer"
}
```

### 3. List Products (Tidak Perlu Login)

```bash
curl http://localhost:3000/products
```

### 4. Create Product (Perlu Login)

```bash
# Tanpa token - akan error 401
curl -X POST http://localhost:3000/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Product",
    "description": "Product description",
    "price": 99.99
  }'

# Dengan token - berhasil
curl -X POST http://localhost:3000/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -d '{
    "name": "New Product",
    "description": "Product description",
    "price": 99.99
  }'
```

## Error Responses

### 401 Unauthorized - Missing Token

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Missing Authorization header"
  }
}
```

### 401 Unauthorized - Invalid Token

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or expired token"
  }
}
```

### 401 Unauthorized - Using AuthUser without Login

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Authentication required. Please login first."
  }
}
```
