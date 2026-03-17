# Multi-stage build for optimal image size

# Build stage
FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src
COPY build.rs .

# Build with optimizations
RUN cargo build --release --locked && \
    cp target/release/ethical-hacking-llm /usr/local/bin/

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    tini \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -g 10001 app && \
    useradd -u 10001 -g app -m -s /bin/bash app

# Create directories
RUN mkdir -p /app/{models,data,logs,config} && \
    chown -R app:app /app

# Copy binary
COPY --from=builder /usr/local/bin/ethical-hacking-llm /usr/local/bin/

# Copy configuration
COPY config /app/config
COPY models/README.md /app/models/

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Switch to non-root user
USER app

# Set working directory
WORKDIR /app

# Expose ports
EXPOSE 3000 9091

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Use tini as init
ENTRYPOINT ["tini", "--", "docker-entrypoint.sh"]

# Default command
CMD ["ethical-hacking-llm"]
