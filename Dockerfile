# Multi-stage Dockerfile for SpeedTestPro Backend
# Optimized for Render deployment

# Stage 1: Build Stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY backend/Cargo.toml backend/Cargo.lock ./

# Copy source code
COPY backend/src ./src
COPY backend/.env.example ./.env

# Build with release optimizations
RUN cargo build --release --locked

# Stage 2: Runtime Stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 speedtest

# Create data directory
RUN mkdir -p /app/data && chown speedtest:speedtest /app/data

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/speedtest-pro-backend ./

# Copy environment template
COPY backend/.env.example ./.env

# Set ownership
RUN chown -R speedtest:speedtest /app

# Switch to non-root user
USER speedtest

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Run the binary
CMD ["./speedtest-pro-backend"]
