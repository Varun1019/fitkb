# Multi-stage Docker build for FitKB Rust PDF Compressor
FROM rust:1.80-slim-bullseye AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
# Cache dependency layer
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release || true

COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime container
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ghostscript \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/fitkb-rust /app/fitkb-rust

ENV PORT=8080
EXPOSE 8080

USER nobody
CMD ["/app/fitkb-rust"]
