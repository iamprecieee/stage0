FROM rust:1.91-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies only
RUN cargo build --release || true
RUN rm -rf target/release/.fingerprint/stage0-* \
    target/release/deps/stage0* \
    target/release/deps/libstage0*

# Copy source code
COPY src ./src

# Build the actual application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/stage0 /app/stage0

ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

EXPOSE 3000

ENTRYPOINT ["/app/stage0"]