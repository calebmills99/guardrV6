# Multi-stage build for Rust backend
FROM rust:1.83-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./
# Cargo.lock might not exist, that's ok - cargo will generate it

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build release binary
RUN cargo build --release --bin guardr-api

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1001 guardr

# Copy binary from builder
COPY --from=builder /app/target/release/guardr-api /usr/local/bin/guardr-api

# Copy config directory
COPY --chown=guardr:guardr config /app/config

# Set working directory
WORKDIR /app

# Switch to non-root user
USER guardr

# Expose port
EXPOSE 5000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:5000/health || exit 1

# Run the API server
CMD ["guardr-api"]
