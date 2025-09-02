#!/bin/bash

# Development script for Qapish - equivalent to `pnpm dev`
# Usage: ./dev.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

# Function to cleanup background processes
cleanup() {
    print_info "Shutting down development servers..."
    jobs -p | xargs -r kill 2>/dev/null || true
    wait 2>/dev/null || true
    print_success "Development servers stopped"
    exit 0
}

# Trap Ctrl+C to cleanup
trap cleanup SIGINT SIGTERM

# Check if required tools are installed
check_deps() {
    if ! command -v cargo &> /dev/null; then
        print_error "cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi

    if ! command -v trunk &> /dev/null; then
        print_warning "trunk not found. Installing..."
        cargo install trunk wasm-bindgen-cli
    fi
}

# Start API server
start_api() {
    print_info "Starting API server on port 8081..."
    PORT=8081 cargo run --bin api &
    API_PID=$!
    sleep 2
    if kill -0 $API_PID 2>/dev/null; then
        print_success "API server started (PID: $API_PID)"
    else
        print_error "Failed to start API server"
        exit 1
    fi
}

# Start web frontend
start_web() {
    print_info "Starting web frontend on port 8080..."
    cd web
    trunk serve --port 8080 &
    WEB_PID=$!
    cd ..
    sleep 2
    if kill -0 $WEB_PID 2>/dev/null; then
        print_success "Web frontend started (PID: $WEB_PID)"
    else
        print_error "Failed to start web frontend"
        exit 1
    fi
}

# Main function
main() {
    echo -e "${GREEN}🚀 Qapish Development Server${NC}"
    echo "=================================="

    check_deps

    print_info "Building project first..."
    if ! cargo build; then
        print_error "Build failed"
        exit 1
    fi

    start_api
    start_web

    echo ""
    print_success "Development servers running:"
    echo "  • API server: http://localhost:8081"
    echo "  • Web frontend: http://localhost:8080"
    echo ""
    print_info "Press Ctrl+C to stop both servers"

    # Wait for background processes
    wait
}

# Run main function
main "$@"
