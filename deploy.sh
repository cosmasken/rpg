#!/bin/bash

# Build script for RPG Game Linera backend
# This script compiles the Rust code to WebAssembly for Linera

set -e  # Exit on any error

echo "Building RPG Game backend for Linera..."

# Ensure we have the wasm32 target
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Navigate to the backend directory
cd /home/groot/Code/akindo/polygon/RPG/linera-backend

# Build the contract and service
echo "Building contract..."
cargo build --release --target wasm32-unknown-unknown

echo "Build complete!"
echo "Contract WASM: target/wasm32-unknown-unknown/release/rpg_game_contract.wasm"
echo "Service WASM: target/wasm32-unknown-unknown/release/rpg_game_service.wasm"

echo ""
echo "To deploy the application to a local Linera network:"
echo "1. Start the local network: linera net up"
echo "2. Publish and create: linera publish-and-create \\"
echo "     target/wasm32-unknown-unknown/release/rpg_game_contract.wasm \\"
echo "     target/wasm32-unknown-unknown/release/rpg_game_service.wasm"