FROM rust:1.82-slim AS builder

WORKDIR /app

# System deps
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first (for caching)
COPY Cargo.toml Cargo.lock ./
COPY packages/core/Cargo.toml packages/core/
COPY packages/auth/Cargo.toml packages/auth/
COPY packages/media/Cargo.toml packages/media/
COPY services/api_server/Cargo.toml services/api_server/
COPY shared/Cargo.toml shared/

# Dummy build to cache deps
RUN cargo build -p rust-reborn-api || true

# Copy full source
COPY packages ./packages
COPY services ./services
COPY shared ./shared
COPY migrations ./migrations

# Final build
RUN cargo build -p rust-reborn-api

FROM alpine:3.18

WORKDIR /app

RUN apk add --no-cache \
    libpq \
    openssl \
    ca-certificates

COPY --from=builder /app/target/release/api_server .

RUN addgroup -g 1001 -S nonroot && \
    adduser -u 1001 -S nonroot -G nonroot

USER nonroot

EXPOSE 3000

ENTRYPOINT ["./api_server"]
