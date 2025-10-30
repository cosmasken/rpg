#!/bin/bash

# Build script for RPG Game Linera backend
# This script compiles the Rust code to WebAssembly for Linera

set -e  # Exit on any error

echo "Building RPG Game backend for Linera..."

# Ensure we have the wasm32 target
rustup target add wasm32-unknown-unknown

# Build the contract and service
echo "Building contract..."
cargo build --release --target wasm32-unknown-unknown -p rpg-game

echo "Build complete!"
echo "Contract WASM: target/wasm32-unknown-unknown/release/rpg_game_contract.wasm"
echo "Service WASM: target/wasm32-unknown-unknown/release/rpg_game_service.wasm"

echo ""
echo "To deploy the application to Linera:"
echo "linera publish-and-create \\"
echo "  target/wasm32-unknown-unknown/release/rpg_game_contract.wasm \\"
echo "  target/wasm32-unknown-unknown/release/rpg_game_service.wasm"