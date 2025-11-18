// RPG Game ABI
// SPDX-License-Identifier: MIT

use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{ContractAbi, ServiceAbi, ChainId, ApplicationId};
use serde::{Deserialize, Serialize};

pub struct RpgGameAbi;

/// Application parameters: Hub application ID and chain ID for multi-chain features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    pub world_region: String,  // The world region this chain represents
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RpgGameOperation {
    /// Save player state to the blockchain
    SavePlayerState {
        player_id: String,
        health: u64,
        max_health: u64,
        strength: u64,
        wisdomness: u64,
        benchpress: u64,
        curl: u64,
        experience: u64,
        level: u64,
    },
    /// Save inventory to the blockchain
    SaveInventory {
        player_id: String,
        inventory: String,  // JSON string of inventory
    },
    /// Save quests to the blockchain
    SaveQuests {
        player_id: String,
        quests: String,  // JSON string of quests
    },
    /// Transfer player to another chain (cross-chain transfer)
    TransferPlayer {
        player_id: String,
        destination_chain: ChainId,
        /// Include complete state for transfer
        player_state: PlayerState,
        inventory: String,
        quests: String,
        /// Authentication token to prevent unauthorized transfers
        auth_token: String,
    },
    /// Join a guild on another chain
    JoinGuild {
        player_id: String,
        guild_id: String,
        chain_id: ChainId,
    },
    /// Initiate a battle and record the result
    RecordBattle {
        battle_id: String,
        player_id: String,
        opponent: String,
        player_result: u64,  // 0 for loss, 1 for draw, 2 for win
        damage_dealt: u64,
        damage_taken: u64,
        experience_gained: u64,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerState {
    pub health: u64,
    pub max_health: u64,
    pub strength: u64,
    pub wisdomness: u64,
    pub benchpress: u64,
    pub curl: u64,
    pub experience: u64,
    pub level: u64,
}

/// Cross-chain message payloads for player transfers and other multi-chain features
#[derive(Debug, Deserialize, Serialize)]
pub enum RpgGameMessage {
    /// Player state transfer to another chain
    PlayerTransfer {
        player_id: String,
        player_state: PlayerState,
        inventory: String,
        quests: String,
        auth_token: String,
    },
    /// Join a guild request from another chain
    GuildJoinRequest {
        player_id: String,
        guild_id: String,
    },
    /// Battle result to be recorded on another chain
    BattleResult {
        battle_id: String,
        player_id: String,
        opponent: String,
        result: u64,  // 0 for loss, 1 for draw, 2 for win
        damage_dealt: u64,
        damage_taken: u64,
        experience_gained: u64,
    },
}

impl ContractAbi for RpgGameAbi {
    type Operation = RpgGameOperation;
    type Response = ();
}

impl ServiceAbi for RpgGameAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[cfg(test)]
mod tests;