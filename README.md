# RPG Game with Linera Microchains Integration

## Overview

This project is a complete RPG game with integrated Linera microchains backend. The game combines a 3D WebGL-based frontend with a secure blockchain backend for persistent player data and inventory management.

## Project Structure

```
RPG/
├── index.html              # Main game HTML file
├── base.css                # Game styling
├── src/                    # Frontend source code
│   ├── *.js               # Game engine components
│   └── blockchain-integration.js  # Linera integration
├── resources/              # Game assets
├── linera-backend/         # Linera blockchain backend
│   ├── Cargo.toml          # Backend dependencies
│   ├── src/
│   │   ├── lib.rs         # ABI definitions
│   │   ├── state.rs       # Data structures
│   │   ├── contract.rs    # Blockchain logic
│   │   └── service.rs     # GraphQL API
│   └── tests/             # Unit and integration tests
└── LINERADOCS.md          # Linera documentation reference
```

## Frontend Features

- 3D RPG game built with Three.js
- Character movement and combat system
- Inventory management
- Player stats and progression
- Quest system
- UI elements for health, stats, and inventory

## Backend Features

- Player state persistence on the blockchain
- Inventory tracking with flexible schema
- Quest tracking with progress persistence
- Battle result recording
- Level progression tracking
- Web3 integration for true ownership
- NFT-ready architecture
- Cross-chain compatibility

## Integration Points

The frontend and backend are connected through:

1. **BlockchainManager Component** - Handles connection to Linera
2. **Player State Sync** - Health, stats, XP saved to blockchain
3. **Inventory Sync** - Items and equipment persisted on chain
4. **Event Handling** - Game events trigger blockchain operations

## Building and Running

### Frontend
```bash
# Serve the frontend with proper headers
npx http-party/http-server \
  --header Cross-Origin-Embedder-Policy:require-corp \
  --header Cross-Origin-Opener-Policy:same-origin
```

### Backend
```bash
# Navigate to the backend directory
cd linera-backend

# Build for WebAssembly
cargo build --release --target wasm32-unknown-unknown

# Deploy to local Linera network
linera net up
linera publish-and-create \
  target/wasm32-unknown-unknown/release/rpg_game_{contract,service}.wasm
```

## Architecture Compliance

The backend follows Linera best practices:
- Proper ABI definitions with serialization
- View-based state management
- GraphQL service interface
- Error handling with logging
- Unit and integration tests
- Cross-chain message preparation

## Web3 Features

- Player accounts with blockchain persistence
- True item ownership through blockchain
- Cross-game asset compatibility
- Secure and transparent state management
- Gas-efficient operations

## Development Status

- ✅ Complete frontend game engine
- ✅ Linera backend implementation
- ✅ Blockchain integration
- ✅ Player state persistence
- ✅ Inventory management
- ✅ Quest system persistence
- ✅ Error handling
- ✅ Unit tests
- 🔄 Cross-chain functionality (in progress)

## Future Enhancements

- Cross-chain character transfers
- NFT marketplace integration
- Guild/Party system on blockchain
- On-chain quests and achievements
- Advanced crafting system

## License

This project is licensed under MIT License (see LICENSE file for details).