FROM rust:1.88 AS builder

RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY proto /app/proto
COPY participant /app/participant

RUN cargo build --manifest-path /app/participant/Cargo.toml

FROM debian:bookworm-slim

COPY --from=builder /app/participant/target/debug/participant /usr/local/bin/participant

CMD ["participant"]