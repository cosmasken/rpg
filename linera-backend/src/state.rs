// RPG Game State
// SPDX-License-Identifier: MIT

use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryData {
    pub items: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub slot: String,
    pub item_id: String,
    pub params: serde_json::Value,  // Flexible params structure
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestData {
    pub id: String,
    pub title: String,
    pub text: String,
    pub completed: bool,
    pub progress: u64,  // For quests that track progress (e.g., kill 10 monsters)
}

/// The main application state.
#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct RpgGameState {
    /// Map of player IDs to their state
    pub player_states: MapView<String, PlayerData>,
    /// Map of player IDs to their inventory
    pub player_inventories: MapView<String, InventoryData>,
    /// Map of player IDs to their quests
    pub player_quests: MapView<String, Vec<QuestData>>,
}