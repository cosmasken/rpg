# RPG Game Linera Backend

This is the complete backend component for the RPG game, built using Linera microchains technology. It provides blockchain-based storage for player data and inventory with full Web3 integration.

## Architecture

The backend consists of:

1. **ABI (lib.rs)** - Defines the interface contract with serialization methods
2. **State (state.rs)** - Defines the data structures stored on the blockchain
3. **Contract (contract.rs)** - Contains the business logic that runs on the blockchain
4. **Service (service.rs)** - Provides the GraphQL API for frontend interaction
5. **Tests (test.rs, tests/)** - Comprehensive unit and integration tests

## Data Structures

- `PlayerData`: Stores player statistics (health, strength, experience, level, etc.)
- `InventoryData`: Stores player inventory items
- `InventoryItem`: Represents a single item in the inventory with flexible parameters

## Operations

The contract supports the following operations:

- `SavePlayerState`: Saves player statistics to the blockchain
- `SaveInventory`: Saves player inventory to the blockchain

## Queries

The service provides the following queries:

- `playerState(player_id)`: Retrieves player statistics
- `inventory(player_id)`: Retrieves player inventory

## Features

- **Player State Persistence**: Health, stats, XP, and level stored on blockchain
- **Inventory Management**: Items and equipment stored on chain with flexible schema
- **Web3 Integration**: Full blockchain connectivity for RPG game
- **NFT Ready**: Architecture supports NFT-based items
- **Battle State**: Combat results and progression tracked on chain
- **Cross-Chain Ready**: Prepared for multi-chain functionality
- **Error Resilient**: Proper error handling and logging
- **GraphQL API**: Clean interface for frontend communication

## Building

To build the backend:

```bash
# From the linera-backend directory
# Ensure you have the wasm32 target
rustup target add wasm32-unknown-unknown

# Build the contract and service
cargo build --release --target wasm32-unknown-unknown
```

## Deploying

To deploy the application to a local Linera network:

```bash
# Start the local network
linera net up

# Build the WASM modules
cargo build --release --target wasm32-unknown-unknown

# Publish and create the application
linera publish-and-create \
  target/wasm32-unknown-unknown/release/rpg_game_{contract,service}.wasm
```

## Integration with Frontend

The frontend RPG game connects to this backend through the `src/blockchain-integration.js` module in the main RPG directory, which provides a clean interface for saving and loading player data and inventory to/from the blockchain.

The frontend automatically detects if a blockchain connection is available and gracefully degrades to local storage when blockchain is not available.

## Testing

Run the unit tests:

```bash
cargo test
```

The backend includes comprehensive unit tests for:
- Player state storage and retrieval
- Inventory management
- Player progression tracking
- Error handling scenarios

## Production Considerations

- The application is designed with proper error handling using Linera's logging system
- Serialization methods are properly implemented per Linera standards
- Cross-chain message handling is prepared (though not yet fully implemented)
- The architecture supports scaling to multiple chains

## Frontend Connection

The frontend RPG game (in the parent directory) includes a `BlockchainManager` component that:
- Connects to the deployed Linera application
- Updates player stats when they change in-game
- Maintains inventory state on the blockchain
- Shows connection status to the user
- Gracefully handles connection failures