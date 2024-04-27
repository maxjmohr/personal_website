#!/bin/bash

# Set option to exit on any error
set -e

# Get Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the cargo env
source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Get cargo binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Get trunk
cargo binstall trunk -y

# Get wasm-opt
cargo binstall wasm-opt -y

# Install tailwindcss and other dependencies
npm install -D tailwindcss
npm install tailwind-typewriter

# Clean the project
trunk clean
cargo clean

# Get current directory
ROOT_DIR=$(pwd)

# Build the tailwind css file
npx tailwindcss -i ./styles/input.css -o ./styles/main.css

# Build the main project
cd $ROOT_DIR
trunk build --release

# Find all .wasm files in the current directory and subdirectories
find dist/ -name "*.wasm" | while read wasm_file; do
    # Execute wasm-opt command on each file
    wasm-opt -Oz -o "$wasm_file" "$wasm_file"
done