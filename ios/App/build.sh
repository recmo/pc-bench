#!/bin/bash
set -e

# Provide access to Rust utilities
PATH="$PATH:~/.cargo/bin"

# Move to Rust lib project
cd $PROJECT_DIR/../bindings

# Build rust release
cargo lipo --release
cbindgen src/lib.rs -l c > target/universal/release/bindings.h

# Copy to Bindings
cp target/universal/release/* $PROJECT_DIR/Bindings/
