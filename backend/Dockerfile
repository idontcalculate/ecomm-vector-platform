FROM rust:1.82 as builder
WORKDIR /usr/src/app

# Copy top-level Cargo files and backend/vector-engine manifests first (for caching)
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY vector-engine/Cargo.toml vector-engine/Cargo.toml


COPY backend backend
COPY vector-engine vector-engine

 
RUN cargo build --release -p backend

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend ./
EXPOSE 8080
CMD ["./backend"]
