# ---- Build stage ----
FROM rust:1.82-slim-bookworm AS builder
WORKDIR /usr/src/app

# 1) Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY vector-engine/Cargo.toml vector-engine/Cargo.toml

# 2) Prime cache with fake sources
RUN mkdir -p backend/src vector-engine/src \
 && echo 'fn main() {}' > backend/src/main.rs \
 && echo 'fn main() {}' > vector-engine/src/main.rs \
 && cargo build --release -p backend || true

# 3) Build with real backend sources
COPY backend/src backend/src
RUN cargo build --release -p backend

# ---- Runtime stage ----
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend .
EXPOSE 8080
CMD ["./backend"]
