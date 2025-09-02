# Development workflow commands - your equivalent to `pnpm dev`

# Setup database and run development servers
dev: db-setup
    @echo "🚀 Starting development servers..."
    @echo "  - API server: http://localhost:8081"
    @echo "  - Web frontend: http://localhost:8080"
    @echo "  - Use Ctrl+C to stop both servers"
    just dev-api & just dev-web & wait

# 🎬 Demo mode - showcase without database
demo:
    @echo "🎬 Starting demo mode..."
    ./script/demo.sh

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

# Setup PostgreSQL database
db-setup:
    @echo "🗄️ Setting up PostgreSQL database..."
    @if ! command -v psql >/dev/null 2>&1; then \
        echo "❌ PostgreSQL not found. Please install PostgreSQL first."; \
        echo "   Fedora: sudo dnf install postgresql postgresql-server postgresql-contrib"; \
        echo "   RHEL/CentOS Stream: sudo dnf install postgresql postgresql-server postgresql-contrib"; \
        echo "   macOS: brew install postgresql"; \
        echo "   Or use Podman: just db-podman"; \
        exit 1; \
    fi
    @if ! sudo -u postgres psql -lqt | cut -d \| -f 1 | grep -qw qapish; then \
        echo "📊 Creating database 'qapish'..."; \
        sudo -u postgres createdb qapish; \
    else \
        echo "✅ Database 'qapish' already exists"; \
    fi
    @export DATABASE_URL="postgresql://postgres:password@localhost:5432/qapish" && \
    echo "DATABASE_URL=$$DATABASE_URL" > .env

# Run PostgreSQL in Podman (alternative to local install)
db-podman:
    @echo "🐳 Starting PostgreSQL with Podman..."
    ./script/podman-db.sh

# Stop Podman PostgreSQL
db-stop:
    podman stop qapish-postgres || true
    podman rm qapish-postgres || true

# Install development dependencies
install-deps:
    @echo "📥 Installing development dependencies..."
    cargo install trunk wasm-bindgen-cli sqlx-cli

# Show all available commands
help:
    @echo "🚀 Qapish AI Colocation Platform - Available Commands"
    @echo ""
    @echo "🎬 Demo & Development:"
    @echo "  just demo        # Quick demo (no database)"
    @echo "  just dev         # Full development (API + Web)"
    @echo "  just dev-api     # API server only"
    @echo "  just dev-web     # Web frontend only"
    @echo ""
    @echo "🗄️ Database Setup:"
    @echo "  just db-setup    # Local PostgreSQL setup"
    @echo "  just db-podman   # Podman PostgreSQL container"
    @echo "  just db-stop     # Stop Podman database"
    @echo ""
    @echo "📦 Building:"
    @echo "  just build       # Production build"
    @echo "  just build-web   # Web frontend build"
    @echo "  just prod        # Run production server"
    @echo ""
    @echo "🧪 Quality & Testing:"
    @echo "  just test        # Run tests"
    @echo "  just check       # Format & lint check"
    @echo "  just fmt         # Format code"
    @echo "  just fix         # Fix common issues"
    @echo ""
    @echo "📚 More info:"
    @echo "  README.md                           # Main documentation"
    @echo "  .docs/quadlet-deployment.md        # Container deployment"
    @echo "  .docs/marketing-implementation.md  # Marketing features"
    @just --list

# 🎬 Quick demo (no database required)
demo-quick:
    @echo "🎬 Quick demo - no database setup required"
    @echo "📦 This will showcase the marketing website and packages"
    @echo "🐳 For full database demo, use: just db-podman && just dev"
    just demo
