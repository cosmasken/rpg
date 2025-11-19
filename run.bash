#!/usr/bin/env bash

set -eu

# Clean up any previous wallet state first
rm -rf ~/.config/linera

eval "$(linera net helper)"
linera_spawn linera net up --with-faucet

export LINERA_FAUCET_URL=http://localhost:8080
linera wallet init --faucet="$LINERA_FAUCET_URL"
linera wallet request-chain --faucet="$LINERA_FAUCET_URL"

# Build the RPG backend
cd linera-backend
cargo build --release --target wasm32-unknown-unknown

# Deploy the RPG application to the local network
APP_ID=$(linera publish-and-create \
  target/wasm32-unknown-unknown/release/rpg_game_{contract,service}.wasm \
  --json-argument "\"world1\"" \
  --json-parameters "{ \"world_region\": \"world1\" }")

echo "RPG Application deployed with ID: $APP_ID"

# Export the application ID for the frontend
export LINERA_APPLICATION_ID=$APP_ID

# Install npm dependencies and start Vite server from the root directory
cd /build
npm install

# Always use development mode to avoid build issues
npm run start -- --host 0.0.0.0 &
  
# Keep the container running
wait