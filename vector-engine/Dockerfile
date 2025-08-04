FROM rust:1.82-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libopenblas-dev \
    clang \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY vector-engine/Cargo.toml ./vector-engine/Cargo.toml

RUN mkdir -p vector-engine/src \
    && echo 'fn main() {}' > vector-engine/src/main.rs \
    && cargo build --release -p vector-engine

COPY vector-engine/src ./vector-engine/src

RUN cargo build --release -p vector-engine

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libopenblas-base \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/vector-engine ./

EXPOSE 9000
CMD ["./vector-engine"]
