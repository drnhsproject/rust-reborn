# Testing Product API Endpoints

Panduan lengkap untuk testing product API endpoints dengan authentication.

## Prerequisites

1. **Start the server**:

```bash
cargo run --bin api_server
```

Server akan berjalan di `http://localhost:8000`

## API Endpoints

### 📝 Authentication Endpoints (Public)

#### 1. Register User

```bash
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123",
    "name": "John Doe"
  }'
```

**Response:**

```json
{
  "id": "uuid-here",
  "email": "user@example.com",
  "name": "John Doe",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### 2. Login

```bash
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "password123"
  }'
```

**Response:**

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

**💡 Simpan `access_token` untuk digunakan di request berikutnya!**

---

### 📦 Product Endpoints

#### 3. List Products (Public - Tidak Perlu Login)

```bash
curl http://localhost:8000/api/products
```

**Response:**

```json
[
  {
    "id": "uuid-1",
    "name": "Product 1",
    "description": "Description 1",
    "price": 100.0,
    "created_by": "uuid-user-1"
  },
  {
    "id": "uuid-2",
    "name": "Product 2",
    "description": "Description 2",
    "price": 200.0,
    "created_by": "uuid-user-2"
  }
]
```

#### 4. Get Product by ID (Public - Tidak Perlu Login)

```bash
curl http://localhost:8000/api/products/some-uuid
```

**Response:**

```json
{
  "id": "uuid-1",
  "name": "Sample Product",
  "description": "Sample Description",
  "price": 150.0,
  "created_by": "uuid-user-1"
}
```

#### 5. Create Product (Protected - HARUS LOGIN)

**❌ Tanpa Token (akan error 401):**

```bash
curl -X POST http://localhost:8000/api/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "New Product",
    "description": "Product description",
    "price": 99.99
  }'
```

**Error Response:**

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Missing Authorization header"
  }
}
```

**✅ Dengan Token (berhasil):**

```bash
# Ganti YOUR_ACCESS_TOKEN dengan token dari login
curl -X POST http://localhost:8000/api/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -d '{
    "name": "New Product",
    "description": "Product description",
    "price": 99.99
  }'
```

**Success Response:**

```json
{
  "id": "new-uuid",
  "name": "New Product",
  "description": "Product description",
  "price": 99.99,
  "created_by": "your-user-uuid"
}
```

---

## Complete Testing Flow

### Step 1: Register a User

```bash
curl -X POST http://localhost:8000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "test123",
    "name": "Test User"
  }'
```

### Step 2: Login to Get Token

```bash
curl -X POST http://localhost:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "test123"
  }'
```

Copy the `access_token` from response.

### Step 3: List Products (No Auth Needed)

```bash
curl http://localhost:8000/api/products
```

### Step 4: Create Product (Auth Required)

```bash
# Replace TOKEN with your actual token
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

curl -X POST http://localhost:8000/api/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "My Awesome Product",
    "description": "This is a great product",
    "price": 149.99
  }'
```

### Step 5: List Products Again (Should See New Product)

```bash
curl http://localhost:8000/api/products
```

---

## Using Postman or Thunder Client

### Setup Environment Variables

1. Create variable `BASE_URL` = `http://localhost:3000`
2. Create variable `TOKEN` (will be set after login)

### Collection Structure

#### 1. Auth > Register

- Method: `POST`
- URL: `{{BASE_URL}}/api/auth/register`
- Body (JSON):

```json
{
  "email": "test@example.com",
  "password": "test123",
  "name": "Test User"
}
```

#### 2. Auth > Login

- Method: `POST`
- URL: `{{BASE_URL}}/api/auth/login`
- Body (JSON):

```json
{
  "email": "test@example.com",
  "password": "test123"
}
```

- **Test Script** (to save token):

```javascript
// For Postman
pm.environment.set("TOKEN", pm.response.json().access_token);
```

#### 3. Products > List (Public)

- Method: `GET`
- URL: `{{BASE_URL}}/api/products`
- Headers: (none needed)

#### 4. Products > Get by ID (Public)

- Method: `GET`
- URL: `{{BASE_URL}}/api/products/:id`
- Headers: (none needed)

#### 5. Products > Create (Protected)

- Method: `POST`
- URL: `{{BASE_URL}}/api/products`
- Headers:
  - `Authorization`: `Bearer {{TOKEN}}`
  - `Content-Type`: `application/json`
- Body (JSON):

```json
{
  "name": "New Product",
  "description": "Product description",
  "price": 99.99
}
```

---

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

### 400 Bad Request - Validation Error

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Validation failed",
    "details": [
      {
        "field": "price",
        "message": "Price must be greater than 0"
      }
    ]
  }
}
```

---

## Tips

1. **Token Expiration**: Token berlaku selama 24 jam (sesuai config). Setelah itu harus login ulang.

2. **Testing dengan jq**: Untuk parsing JSON response yang lebih mudah:

```bash
# Install jq
sudo apt install jq  # Ubuntu/Debian
brew install jq      # macOS

# Login dan simpan token ke variable
TOKEN=$(curl -s -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123"}' \
  | jq -r '.access_token')

# Gunakan token
curl -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -X POST http://localhost:3000/api/products \
  -d '{"name":"Test","description":"Test","price":99.99}' \
  | jq '.'
```

3. **Debug Mode**: Untuk melihat full request/response:

```bash
curl -v http://localhost:3000/api/products
```

4. **Health Check**: Untuk memastikan server berjalan:

```bash
curl http://localhost:3000/api/health
```

---

## Route Summary

| Method | Endpoint             | Auth Required | Description         |
| ------ | -------------------- | ------------- | ------------------- |
| POST   | `/api/auth/register` | ❌ No         | Register new user   |
| POST   | `/api/auth/login`    | ❌ No         | Login and get token |
| GET    | `/api/products`      | ❌ No         | List all products   |
| GET    | `/api/products/:id`  | ❌ No         | Get product by ID   |
| POST   | `/api/products`      | ✅ Yes        | Create new product  |

**Legend:**

- ❌ No = Public endpoint, tidak perlu authentication
- ✅ Yes = Protected endpoint, harus kirim Bearer token
