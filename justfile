# Development workflow commands - your equivalent to `pnpm dev`

# 🚀 Main dev command: Run both API and web frontend
dev:
    @echo "🚀 Starting development servers..."
    @echo "  - API server: http://localhost:8081"
    @echo "  - Web frontend: http://localhost:8080"
    @echo "  - Use Ctrl+C to stop both servers"
    just dev-api & just dev-web & wait

# Run API server in development mode
dev-api:
    @echo "🔧 Starting API server on :8081..."
    PORT=8081 cargo run --bin api

# Run web frontend with hot reload
dev-web:
    @echo "🌐 Starting web frontend with hot reload on :8080..."
    cd web && trunk serve --port 8080

# Build everything for production
build:
    @echo "📦 Building for production..."
    cargo build --release
    cd web && trunk build --release

# Build only the web frontend
build-web:
    cd web && trunk build --release

# Run production server (builds web first)
prod: build-web
    @echo "🚀 Starting production server on :8080..."
    PORT=8080 cargo run --release --bin api

# Clean all build artifacts
clean:
    cargo clean
    cd web && rm -rf dist

# Run tests
test:
    cargo test

# Check code formatting and linting
check:
    cargo fmt --check
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Fix common issues
fix:
    cargo fix --allow-dirty
    cargo fmt

# Install development dependencies
install-deps:
    @echo "📥 Installing trunk for web development..."
    cargo install trunk wasm-bindgen-cli

# Show all available commands
help:
    @just --list
