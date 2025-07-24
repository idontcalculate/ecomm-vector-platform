FROM rust:1.82 AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY vector-engine/Cargo.toml vector-engine/Cargo.toml

RUN mkdir -p backend/src vector-engine/src \
 && echo 'fn main() {}' > backend/src/main.rs \
 && echo 'pub fn dummy() {}' > vector-engine/src/lib.rs \
 && cargo build --release -p backend

COPY backend src/backend
COPY vector-engine src/vector-engine
COPY . .
RUN cargo build --release -p backend

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend ./
EXPOSE 8080
CMD ["./backend"]
