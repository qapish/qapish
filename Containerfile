# Containerfile for Qapish AI Colocation Platform
# Use with: podman build -t qapish-api:latest .

# Multi-stage build for optimal image size
FROM registry.fedoraproject.org/fedora:39 as builder

# Install build dependencies
RUN dnf install -y \
    rust \
    cargo \
    pkg-config \
    openssl-devel \
    && dnf clean all

# Create app user for build
RUN useradd --create-home --shell /bin/bash app

# Set working directory
WORKDIR /app

# Copy dependency manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY ai/Cargo.toml ./ai/
COPY api/Cargo.toml ./api/
COPY web/Cargo.toml ./web/
COPY infra/Cargo.toml ./infra/
COPY libs/persistence/Cargo.toml ./libs/persistence/

# Install trunk for web builds
RUN cargo install trunk wasm-bindgen-cli

# Copy source code
COPY . .

# Build web frontend
WORKDIR /app/web
RUN trunk build --release

# Build API server (release mode)
WORKDIR /app
RUN cargo build --release --bin api

# Runtime stage
FROM registry.fedoraproject.org/fedora-minimal:39

# Install runtime dependencies
RUN microdnf install -y \
    ca-certificates \
    openssl \
    curl \
    && microdnf clean all

# Create non-root user for runtime
RUN useradd --create-home --shell /bin/bash --uid 1000 qapish

# Set working directory
WORKDIR /app

# Copy built artifacts
COPY --from=builder /app/target/release/api /usr/local/bin/qapish-api
COPY --from=builder /app/web/dist ./web/dist

# Set ownership
RUN chown -R qapish:qapish /app

# Switch to non-root user
USER qapish

# Expose port (high port for rootless)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

# Run the application
CMD ["qapish-api"]
