// RPG Game Contract
// SPDX-License-Identifier: MIT

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use rpg_game::{RpgGameAbi, RpgGameOperation, RpgGameMessage};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use serde_json;
use state::{InventoryData, PlayerData, RpgGameState, BattleRecord, GuildData, PlayerTransferRequest};

pub struct RpgGameContract {
    state: RpgGameState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(RpgGameContract);

impl WithContractAbi for RpgGameContract {
    type Abi = RpgGameAbi;
}

impl Contract for RpgGameContract {
    type Message = RpgGameMessage;
    type InstantiationArgument = String; // World region name
    type Parameters = rpg_game::Parameters;
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = RpgGameState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        RpgGameContract { state, runtime }
    }

    async fn instantiate(&mut self, world_region: String) {
        // Validate that the application parameters were configured correctly.
        let params = self.runtime.application_parameters();
        if params.world_region != world_region {
            println!("World region parameter mismatch");
        }

        // Set the world region for this chain
        self.state.world_region.set(world_region);
    }

    async fn execute_operation(&mut self, operation: RpgGameOperation) -> () {
        match operation {
            RpgGameOperation::SavePlayerState {
                player_id,
                health,
                max_health,
                strength,
                wisdomness,
                benchpress,
                curl,
                experience,
                level,
            } => {
                let player_data = PlayerData {
                    health,
                    max_health,
                    strength,
                    wisdomness,
                    benchpress,
                    curl,
                    experience,
                    level,
                };

                if let Err(e) = self.state.player_states.insert(&player_id, player_data) {
                    println!("Failed to save player state to blockchain for player {}: {}", player_id, e);
                }
            }
            RpgGameOperation::SaveInventory { player_id, inventory } => {
                // Parse the inventory JSON string into the proper structure
                let inventory_data: Vec<crate::state::InventoryItem> =
                    match serde_json::from_str(&inventory) {
                        Ok(data) => data,
                        Err(e) => {
                            println!("Failed to parse inventory JSON for player {}: {}", player_id, e);
                            return; // Exit the operation early if JSON parsing fails
                        }
                    };

                let inventory_struct = InventoryData {
                    items: inventory_data,
                };

                if let Err(e) = self.state.player_inventories.insert(&player_id, inventory_struct) {
                    println!("Failed to save inventory to blockchain for player {}: {}", player_id, e);
                }
            }
            RpgGameOperation::SaveQuests { player_id, quests } => {
                // Parse the quests JSON string into the proper structure
                let quests_data: Vec<crate::state::QuestData> =
                    match serde_json::from_str(&quests) {
                        Ok(data) => data,
                        Err(e) => {
                            println!("Failed to parse quests JSON for player {}: {}", player_id, e);
                            return; // Exit the operation early if JSON parsing fails
                        }
                    };

                if let Err(e) = self.state.player_quests.insert(&player_id, quests_data) {
                    println!("Failed to save quests to blockchain for player {}: {}", player_id, e);
                }
            }
            RpgGameOperation::TransferPlayer {
                player_id,
                destination_chain,
                player_state,
                inventory,
                quests,
                auth_token,
            } => {
                // Transfer player to another chain by sending a cross-chain message
                let transfer_message = RpgGameMessage::PlayerTransfer {
                    player_id: player_id.clone(),
                    player_state,
                    inventory,
                    quests,
                    auth_token: auth_token.clone(),
                };

                // Send the player data to the destination chain
                self.runtime
                    .prepare_message(transfer_message)
                    .with_authentication()
                    .send_to(destination_chain);

                // Remove the player from the current chain (deleting the old data)
                // We don't delete immediately because the transfer might fail and need to be retried
                let transfer_request = PlayerTransferRequest {
                    source_chain: self.runtime.chain_id(),
                    destination_chain,
                    player_id: player_id.clone(),
                    auth_token,
                    timestamp: self.runtime.system_time().micros(),
                };

                if let Err(e) = self.state.player_transfer_requests.insert(&player_id, transfer_request) {
                    println!("Failed to save player transfer request for player {}: {}", player_id, e);
                }
            }
            RpgGameOperation::JoinGuild {
                player_id,
                guild_id,
                chain_id,
            } => {
                // Send a cross-chain message to join a guild on another chain
                let join_message = RpgGameMessage::GuildJoinRequest {
                    player_id: player_id.clone(),
                    guild_id: guild_id.clone(),
                };

                self.runtime
                    .prepare_message(join_message)
                    .with_authentication()
                    .send_to(chain_id);

                // Add to join request list temporarily
                if let Some(mut requests) = self.state.guild_join_requests.get_mut(&guild_id).await.expect("Failed to get guild join requests") {
                    requests.push(player_id);
                } else {
                    let mut requests = Vec::new();
                    requests.push(player_id);
                    if let Err(e) = self.state.guild_join_requests.insert(&guild_id, requests) {
                        println!("Failed to save guild join request for guild {}: {}", guild_id, e);
                    }
                }
            }
            RpgGameOperation::RecordBattle {
                battle_id,
                player_id,
                opponent,
                player_result,
                damage_dealt,
                damage_taken,
                experience_gained,
            } => {
                // Create a battle record
                let battle_record = BattleRecord {
                    battle_id: battle_id.clone(),
                    player_id: player_id.clone(),
                    opponent,
                    result: player_result,
                    damage_dealt,
                    damage_taken,
                    experience_gained,
                    timestamp: self.runtime.system_time().micros(),
                };

                // Save the battle record
                if let Err(e) = self.state.battle_records.insert(&battle_id, battle_record) {
                    println!("Failed to save battle record for battle {}: {}", battle_id, e);
                }

                // Add to player's battle history
                if let Some(mut battles) = self.state.player_battles.get_mut(&player_id).await.expect("Failed to get player battles") {
                    battles.push(battle_id.clone());
                } else {
                    let mut battles = Vec::new();
                    battles.push(battle_id.clone());
                    if let Err(e) = self.state.player_battles.insert(&player_id, battles) {
                        println!("Failed to save player battle history for player {}: {}", player_id, e);
                    }
                }
            }
        }
    }

    async fn execute_message(&mut self, message: RpgGameMessage) {
        match message {
            RpgGameMessage::PlayerTransfer {
                player_id,
                player_state,
                inventory,
                quests,
                auth_token,
            } => {
                // Verify auth token if needed, then add the player to this chain
                // For now, we'll accept all transfers (in a real implementation, we'd verify the auth token)

                // Convert PlayerState to PlayerData
                let player_data = PlayerData {
                    health: player_state.health,
                    max_health: player_state.max_health,
                    strength: player_state.strength,
                    wisdomness: player_state.wisdomness,
                    benchpress: player_state.benchpress,
                    curl: player_state.curl,
                    experience: player_state.experience,
                    level: player_state.level,
                };

                // Save the player state
                if let Err(e) = self.state.player_states.insert(&player_id, player_data) {
                    println!("Failed to save transferred player state for player {}: {}", player_id, e);
                    return;
                }

                // Save the inventory
                let inventory_data: Vec<crate::state::InventoryItem> =
                    match serde_json::from_str(&inventory) {
                        Ok(data) => data,
                        Err(e) => {
                            println!("Failed to parse transferred inventory JSON for player {}: {}", player_id, e);
                            return;
                        }
                    };

                let inventory_struct = InventoryData {
                    items: inventory_data,
                };

                if let Err(e) = self.state.player_inventories.insert(&player_id, inventory_struct) {
                    println!("Failed to save transferred inventory for player {}: {}", player_id, e);
                    return;
                }

                // Save the quests
                let quests_data: Vec<crate::state::QuestData> =
                    match serde_json::from_str(&quests) {
                        Ok(data) => data,
                        Err(e) => {
                            println!("Failed to parse transferred quests JSON for player {}: {}", player_id, e);
                            return;
                        }
                    };

                if let Err(e) = self.state.player_quests.insert(&player_id, quests_data) {
                    println!("Failed to save transferred quests for player {}: {}", player_id, e);
                    return;
                }

                println!("Player {} successfully transferred to this chain", player_id);
            }
            RpgGameMessage::GuildJoinRequest {
                player_id,
                guild_id,
            } => {
                // Add the player to the guild if it exists
                if let Some(mut guild) = self.state.guilds.get_mut(&guild_id).await.expect("Failed to get guild") {
                    // Check if player is already in the guild
                    if !guild.members.contains(&player_id) {
                        guild.members.push(player_id.clone());

                        // Also update the player's guild mapping
                        if let Err(e) = self.state.player_guilds.insert(&player_id, guild_id.clone()) {
                            println!("Failed to update player guild mapping for player {}: {}", player_id, e);
                        }

                        println!("Player {} joined guild {} on this chain", player_id, guild_id);
                    } else {
                        println!("Player {} is already in guild {}", player_id, guild_id);
                    }
                } else {
                    // Guild doesn't exist on this chain, create a new one with the player
                    let mut new_guild = GuildData {
                        id: guild_id.clone(),
                        name: format!("Guild_{}", guild_id),  // In a real impl, we'd fetch the name from the source
                        members: vec![player_id.clone()],
                        resources: 0,
                        level: 1,
                    };

                    if let Err(e) = self.state.guilds.insert(&guild_id, new_guild) {
                        println!("Failed to create new guild {}: {}", guild_id, e);
                        return;
                    }

                    if let Err(e) = self.state.player_guilds.insert(&player_id, guild_id.clone()) {
                        println!("Failed to update player guild mapping for player {}: {}", player_id, e);
                    }

                    println!("Player {} joined new guild {} on this chain", player_id, guild_id);
                }
            }
            RpgGameMessage::BattleResult {
                battle_id,
                player_id,
                opponent,
                result,
                damage_dealt,
                damage_taken,
                experience_gained,
            } => {
                // Record the battle result
                let battle_record = BattleRecord {
                    battle_id: battle_id.clone(),
                    player_id: player_id.clone(),
                    opponent,
                    result,
                    damage_dealt,
                    damage_taken,
                    experience_gained,
                    timestamp: self.runtime.system_time().micros(),
                };

                if let Err(e) = self.state.battle_records.insert(&battle_id, battle_record) {
                    println!("Failed to save battle result for battle {}: {}", battle_id, e);
                    return;
                }

                if let Some(mut battles) = self.state.player_battles.get_mut(&player_id).await.expect("Failed to get player battles") {
                    battles.push(battle_id.clone());
                } else {
                    let mut battles = Vec::new();
                    battles.push(battle_id.clone());
                    if let Err(e) = self.state.player_battles.insert(&player_id, battles) {
                        println!("Failed to save player battle history for player {}: {}", player_id, e);
                    }
                }

                println!("Battle {} result recorded for player {}", battle_id, player_id);
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}