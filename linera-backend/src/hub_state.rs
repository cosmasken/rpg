use linera_sdk::linera_base_types::ChainId;
use linera_views::views::{MapView, RegisterView, RootView, ViewStorageContext};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAchievement {
    pub achievement_id: String,
    pub chain_id: ChainId,
    pub timestamp: u64,
    pub metadata: String,  // JSON string with achievement details
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldChainInfo {
    pub world_region: String,
    pub registration_timestamp: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRecord {
    pub achievement_id: String,
    pub player_id: String,
    pub chain_id: ChainId,
    pub timestamp: u64,
    pub metadata: String,
}

/// The hub application state.
#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct HubState {
    /// Map of player IDs to their achievements
    pub player_achievements: MapView<String, Vec<PlayerAchievement>>,
    /// Map of chain IDs to world chain info
    pub world_chains: MapView<ChainId, WorldChainInfo>,
    /// All achievement records
    pub all_achievements: MapView<String, Vec<AchievementRecord>>,  // achievement_id -> records
    /// Player achievement lookup by achievement_id
    pub player_achievement_lookup: MapView<String, Vec<String>>,  // "player_id:achievement_id" -> [record_ids]
    /// Total registered chains count
    pub total_chains: RegisterView<u64>,
    /// Total achievements count
    pub total_achievements: RegisterView<u64>,
}