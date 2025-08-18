# Multi-stage Dockerfile for AMOS deployment

# Stage 1: Build Rust backend
FROM rust:1.83 AS backend-builder
WORKDIR /build

# Disable sccache for Docker builds
ENV CARGO_INCREMENTAL=0
ENV RUSTC_WRAPPER=""

# Copy AMOS workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY backend ./backend
COPY demos ./demos

# Build the backend
WORKDIR /build/backend
RUN cargo build --release

# Stage 2: Build React frontend
FROM node:20-alpine AS frontend-builder
WORKDIR /app

# Copy frontend files
COPY frontend/package*.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1001 amos

# Copy built artifacts
COPY --from=backend-builder /build/target/release/amos-deploy-server /app/
COPY --from=frontend-builder /app/dist /app/static

# Set ownership
RUN chown -R amos:amos /app

# Switch to non-root user
USER amos

# Environment
ENV AMOS_STATIC_DIR=/app/static
ENV AMOS_PORT=8080

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

CMD ["/app/amos-deploy-server"]