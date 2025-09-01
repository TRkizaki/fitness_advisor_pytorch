#!/bin/bash
# Build script for Leptos fitness dashboard

echo "Building Leptos fitness dashboard..."

# Install trunk if not available
if ! command -v trunk &> /dev/null; then
    echo "Installing trunk..."
    cargo install trunk
fi

# Build the application
trunk build --release

echo "Build complete! Open dist/index.html in your browser."