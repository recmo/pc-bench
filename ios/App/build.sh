#!/bin/bash
set -e

echo "PATH"
echo $PATH
echo "PROJECT_DIR"
echo $PROJECT_DIR
# Provide access to Rust utilities
PATH="$PATH:~/.cargo/bin"

# Move to Rust lib project
cd $PROJECT_DIR/../bindings

# Build rust release
cargo +nightly build --target aarch64-apple-ios --lib --release
cbindgen src/lib.rs -l c > target/universal/release/bindings.h

# Copy to Bindings
cp target/aarch64-apple-ios/release/* $PROJECT_DIR/Bindings/
