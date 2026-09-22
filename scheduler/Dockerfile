FROM rust:1.88 AS builder

RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY proto /app/proto
COPY scheduler /app/scheduler

RUN cargo build --manifest-path /app/scheduler/Cargo.toml

FROM debian:bookworm-slim

COPY --from=builder /app/scheduler/target/debug/scheduler /usr/local/bin/scheduler

CMD ["scheduler"]