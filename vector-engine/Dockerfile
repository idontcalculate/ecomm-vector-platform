FROM rust:1.82 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release -p vector-engine

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/vector-engine ./
EXPOSE 9000
CMD ["./vector-engine"]
