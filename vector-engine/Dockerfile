# ---- Build stage ----
FROM rust:1.82-slim-bookworm AS builder
WORKDIR /usr/src/app

# 1) Copy workspace manifests so cargo can resolve members without source files yet
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY vector-engine/Cargo.toml vector-engine/Cargo.toml

# 2) Prime cargo cache with fake sources (speeds rebuilds and avoids workspace errors)
RUN mkdir -p backend/src vector-engine/src \
 && echo 'fn main() {}' > backend/src/main.rs \
 && echo 'fn main() {}' > vector-engine/src/main.rs \
 && cargo build --release -p vector-engine || true

# 3) Now copy real source for vector-engine only and build it
COPY vector-engine/src vector-engine/src
RUN cargo build --release -p vector-engine

# ---- Runtime stage ----
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/vector-engine .
EXPOSE 9000
CMD ["./vector-engine"]
