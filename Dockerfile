FROM rust:1.91-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    ca-certificates \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src for dependency caching
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies only
RUN cargo build --release --target x86_64-unknown-linux-musl || true
RUN rm -rf target/x86_64-unknown-linux-musl/release/.fingerprint/stage0-* \
    target/x86_64-unknown-linux-musl/release/deps/stage0* \
    target/x86_64-unknown-linux-musl/release/deps/libstage0*

# Copy source code
COPY src ./src

# Build the actual application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime image
FROM alpine:3.20

RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/stage0 /app/stage0

ENV RUST_LOG=info
ENV HOST=0.0.0.0

EXPOSE 3000

ENTRYPOINT ["/app/stage0"]