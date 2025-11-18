# The Legend of SimonDev - RPG with Linera Integration

## Overview

This is an advanced RPG game with distributed world regions across multiple Linera chains, featuring cross-chain player transfers, verifiable battle system, and multi-chain guild system with shared resources.

## Features Implemented

### Cross-chain Player Transfers
- Complete state migration including health, stats, inventory, and quests
- Authentication tokens to secure transfers
- Seamless movement between world regions

### Distributed World Regions
- Each chain represents a different region of the game world
- Players can explore different regions with unique content
- World region identification and tracking

### Verifiable Battle System
- On-chain battle records with detailed statistics
- Damage dealt/taken tracking
- Experience gain recording
- Battle history for each player

### Multi-chain Guild System
- Join guilds across different chains
- Shared resources and guild progression
- Cross-chain guild management

## Architecture

The application consists of:
- **Frontend**: 3D RPG game built with Three.js
- **Backend**: Linera microchain application with contract and service components
- **Blockchain Integration**: Secure player state persistence and cross-chain communication

## Build and Run Instructions

This project can be run using Docker Compose with the provided configuration:

1. The container automatically builds the Linera backend
2. Deploys the application to a local Linera network
3. Serves the frontend on port 5173

Access the game at `http://localhost:5173`

## Technical Details

### Backend Components
- **Contract**: Handles state changes and cross-chain messages
- **Service**: Provides GraphQL API for frontend interaction
- **State**: Manages player data, inventory, quests, battles, and guilds

### Cross-chain Features
- Player transfer operations with complete state
- Guild membership across chains
- Battle result synchronization
- World region management

## Game Features

- 3D RPG gameplay with character movement and combat
- Inventory management system
- Quest progression system
- Player stats (strength, wisdomness, benchpress, curl)
- Health and level progression
- Guild and social features

## Linera Integration

All player data is securely stored on the blockchain:
- Player state (health, stats, XP)
- Inventory items
- Quest progress
- Battle records
- Guild membership

The game demonstrates advanced multi-chain capabilities of the Linera protocol while providing engaging RPG gameplay.