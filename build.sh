#!/bin/bash

# Build script for wbnova-godot-plugin
# Usage: ./build.sh [release|debug]

set -e  # Exit on any error

# Check if argument is provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 [release|debug]"
    echo "  release - Build in release mode"
    echo "  debug   - Build in debug mode"
    exit 1
fi

BUILD_TYPE="$1"

# Validate build type
if [ "$BUILD_TYPE" != "release" ] && [ "$BUILD_TYPE" != "debug" ]; then
    echo "Error: Build type must be 'release' or 'debug'"
    echo "Usage: $0 [release|debug]"
    exit 1
fi

echo "Building wbnova-godot-plugin in $BUILD_TYPE mode..."

# Step 1: Build the project
if [ "$BUILD_TYPE" = "release" ]; then
    echo "Running: cargo build --release"
    cargo build --release
    SOURCE_BINARY="target/release/libwbnova_integration.so"
    TARGET_BINARY="project/addons/wbnovaintegration/libwbnova_integration.so"
else
    echo "Running: cargo build"
    cargo build
    SOURCE_BINARY="target/debug/libwbnova_integration.so"
    TARGET_BINARY="project/addons/wbnovaintegration/libwbnova_integration_debug.so"
fi

# Step 2: Copy the binary to the addons directory
echo "Copying binary from $SOURCE_BINARY to $TARGET_BINARY"

# Ensure the target directory exists
mkdir -p "$(dirname "$TARGET_BINARY")"

# Copy the binary
cp "$SOURCE_BINARY" "$TARGET_BINARY"

echo "Build completed successfully!"
echo "Binary copied to: $TARGET_BINARY"