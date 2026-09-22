FROM rust:1.88 AS builder

RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY proto /app/proto
COPY coordinator /app/coordinator

RUN cargo build --manifest-path /app/coordinator/Cargo.toml

FROM debian:bookworm-slim

COPY --from=builder /app/coordinator/target/debug/coordinator /usr/local/bin/coordinator

CMD ["coordinator"]