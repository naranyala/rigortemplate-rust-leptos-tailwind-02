#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Custom error handler
function log_error() {
    echo -e "\n\033[0;31m[ERROR]\033[0m $1"
    echo "Check the logs above for more details."
    exit 1
}

# 1. Install dependencies using bun
echo "Installing dependencies with bun..."
bun install || log_error "Failed to install dependencies. Please check your network connection."

# 2. Build Tailwind CSS
echo "Building Tailwind CSS..."
bun run tailwind:build || log_error "Tailwind CSS build failed. Check input.css for syntax errors."

# 3. Port Detection Logic
PORT=$(./scripts/find-port.sh 8080)

echo "Using port: $PORT"

# 4. Run the application using Trunk
echo "Starting Trunk server on port $PORT..."

# Start Tailwind watcher in background
bun run tailwind:watch &
TAILWIND_PID=$!

# Start Trunk with the detected port
trunk serve --port $PORT --open false || {
    kill $TAILWIND_PID
    log_error "Trunk server failed to start. Ensure Rust and WASM target are correctly installed."
}

# Ensure tailwind watcher is killed when trunk is stopped
trap 'kill $TAILWIND_PID' EXIT

echo "Application is running at http://localhost:$PORT"
