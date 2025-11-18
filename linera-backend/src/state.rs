// RPG Game State
// SPDX-License-Identifier: MIT

use linera_sdk::views::{MapView, RegisterView, RootView, ViewStorageContext};
use linera_sdk::linera_base_types::ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct PlayerData {
    pub health: u64,
    pub max_health: u64,
    pub strength: u64,
    pub wisdomness: u64,
    pub benchpress: u64,
    pub curl: u64,
    pub experience: u64,
    pub level: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct InventoryData {
    pub items: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct InventoryItem {
    pub slot: String,
    pub item_id: String,
    pub params: serde_json::Value,  // Flexible params structure
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct QuestData {
    pub id: String,
    pub title: String,
    pub text: String,
    pub completed: bool,
    pub progress: u64,  // For quests that track progress (e.g., kill 10 monsters)
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct BattleRecord {
    pub battle_id: String,
    pub player_id: String,
    pub opponent: String,
    pub result: u64,  // 0 for loss, 1 for draw, 2 for win
    pub damage_dealt: u64,
    pub damage_taken: u64,
    pub experience_gained: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct GuildData {
    pub id: String,
    pub name: String,
    pub members: Vec<String>,  // Player IDs
    pub resources: u64,
    pub level: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct PlayerTransferRequest {
    pub source_chain: ChainId,
    pub destination_chain: ChainId,
    pub player_id: String,
    pub auth_token: String,
    pub timestamp: u64,
}

/// The main application state.
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct RpgGameState {
    /// Map of player IDs to their state
    pub player_states: MapView<String, PlayerData>,
    /// Map of player IDs to their inventory
    pub player_inventories: MapView<String, InventoryData>,
    /// Map of player IDs to their quests
    pub player_quests: MapView<String, Vec<QuestData>>,
    /// Battle records for verifiable combat system
    pub battle_records: MapView<String, BattleRecord>,
    /// Guild data for multi-chain guild system
    pub guilds: MapView<String, GuildData>,
    /// Guild membership map (player_id -> guild_id)
    pub player_guilds: MapView<String, String>,
    /// Battle records organized by player
    pub player_battles: MapView<String, Vec<String>>,  // List of battle IDs for each player
    /// Guild membership requests
    pub guild_join_requests: MapView<String, Vec<String>>,  // guild_id -> [player_ids]
    /// Player transfer requests
    pub player_transfer_requests: MapView<String, PlayerTransferRequest>,  // transfer_id -> request
    /// World region identifier for this chain
    pub world_region: RegisterView<String>,
}