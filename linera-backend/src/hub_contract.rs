// RPG Hub Contract
// SPDX-License-Identifier: MIT

#[cfg_attr(target_arch = "wasm32", no_main)]
use crate::hub_abi::{HubOperation, HubMessage, RpgHubAbi};
use crate::hub_state::{HubState, PlayerAchievement, WorldChainInfo};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use linera_sdk::log;

pub struct RpgHubContract {
    state: HubState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(RpgHubContract);

impl WithContractAbi for RpgHubContract {
    type Abi = RpgHubAbi;
}

impl Contract for RpgHubContract {
    type Message = HubMessage;
    type InstantiationArgument = u64; // max_achievements
    type Parameters = crate::hub_abi::HubParameters;
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = HubState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load hub state");
        RpgHubContract { state, runtime }
    }

    async fn instantiate(&mut self, max_achievements: u64) {
        // Validate that the application parameters were configured correctly
        let params = self.runtime.application_parameters::<hub_abi::HubParameters>();
        if params.is_err() || params.unwrap().max_achievements != max_achievements {
            linera_sdk::log::warn!("Max achievements parameter mismatch");
        }
        
        // Initialize counters
        self.state.total_chains.set(0);
        self.state.total_achievements.set(0);
    }

    async fn execute_operation(&mut self, operation: HubOperation) -> () {
        match operation {
            HubOperation::SubmitAchievement {
                player_id,
                achievement_id,
                chain_id,
                timestamp,
                metadata,
            } => {
                // Create the achievement record
                let player_achievement = PlayerAchievement {
                    achievement_id: achievement_id.clone(),
                    chain_id,
                    timestamp,
                    metadata: metadata.clone(),
                };
                
                // Add to player's achievements
                if let Some(mut achievements) = self.state.player_achievements.get_mut(&player_id).await.expect("Failed to get player achievements") {
                    achievements.push(player_achievement);
                } else {
                    let mut achievements = Vec::new();
                    achievements.push(player_achievement);
                    if let Err(e) = self.state.player_achievements.insert(&player_id, achievements).await {
                        linera_sdk::log::error!("Failed to save player achievements for player {}: {}", player_id, e);
                        return;
                    }
                }
                
                // Add to all achievements by achievement_id
                let achievement_record = hub_state::AchievementRecord {
                    achievement_id: achievement_id.clone(),
                    player_id: player_id.clone(),
                    chain_id,
                    timestamp,
                    metadata,
                };
                
                if let Some(mut records) = self.state.all_achievements.get_mut(&achievement_id).await.expect("Failed to get achievement records") {
                    records.push(achievement_record);
                } else {
                    let mut records = Vec::new();
                    records.push(achievement_record);
                    if let Err(e) = self.state.all_achievements.insert(&achievement_id, records).await {
                        linera_sdk::log::error!("Failed to save achievement records for achievement {}: {}", achievement_id, e);
                        return;
                    }
                }
                
                // Update counters
                let current_achievements = *self.state.total_achievements.get();
                self.state.total_achievements.set(current_achievements + 1);
                
                linera_sdk::log::info!("Achievement {} submitted for player {} from chain {}", achievement_id, player_id, chain_id);
            }
            HubOperation::RegisterWorldChain {
                chain_id,
                world_region,
            } => {
                // Register the world chain
                let world_chain_info = WorldChainInfo {
                    world_region: world_region.clone(),
                    registration_timestamp: self.runtime.system_time().as_millis(),
                    active: true,
                };
                
                if let Err(e) = self.state.world_chains.insert(&chain_id, world_chain_info).await {
                    linera_sdk::log::error!("Failed to register world chain {}: {}", chain_id, e);
                    return;
                }
                
                // Update counter
                let current_chains = *self.state.total_chains.get();
                self.state.total_chains.set(current_chains + 1);
                
                linera_sdk::log::info!("World chain {} registered with region {}", chain_id, world_region);
            }
        }
    }

    async fn execute_message(&mut self, message: HubMessage) {
        match message {
            HubMessage::AchievementSubmitted {
                player_id,
                achievement_id,
                chain_id,
                timestamp,
                metadata,
            } => {
                // This is essentially the same as the operation, just via cross-chain message
                let player_achievement = PlayerAchievement {
                    achievement_id: achievement_id.clone(),
                    chain_id,
                    timestamp,
                    metadata: metadata.clone(),
                };
                
                // Add to player's achievements
                if let Some(mut achievements) = self.state.player_achievements.get_mut(&player_id).await.expect("Failed to get player achievements") {
                    achievements.push(player_achievement);
                } else {
                    let mut achievements = Vec::new();
                    achievements.push(player_achievement);
                    if let Err(e) = self.state.player_achievements.insert(&player_id, achievements).await {
                        linera_sdk::log::error!("Failed to save player achievements for player {}: {}", player_id, e);
                        return;
                    }
                }
                
                // Add to all achievements by achievement_id
                let achievement_record = hub_state::AchievementRecord {
                    achievement_id: achievement_id.clone(),
                    player_id: player_id.clone(),
                    chain_id,
                    timestamp,
                    metadata,
                };
                
                if let Some(mut records) = self.state.all_achievements.get_mut(&achievement_id).await.expect("Failed to get achievement records") {
                    records.push(achievement_record);
                } else {
                    let mut records = Vec::new();
                    records.push(achievement_record);
                    if let Err(e) = self.state.all_achievements.insert(&achievement_id, records).await {
                        linera_sdk::log::error!("Failed to save achievement records for achievement {}: {}", achievement_id, e);
                        return;
                    }
                }
                
                // Update counters
                let current_achievements = *self.state.total_achievements.get();
                self.state.total_achievements.set(current_achievements + 1);
                
                linera_sdk::log::info!("Achievement {} submitted via message for player {} from chain {}", achievement_id, player_id, chain_id);
            }
            HubMessage::WorldChainRegistered {
                chain_id,
                world_region,
            } => {
                // Register the world chain via cross-chain message
                let world_chain_info = WorldChainInfo {
                    world_region: world_region.clone(),
                    registration_timestamp: self.runtime.system_time().as_millis(),
                    active: true,
                };
                
                if let Err(e) = self.state.world_chains.insert(&chain_id, world_chain_info).await {
                    linera_sdk::log::error!("Failed to register world chain via message {}: {}", chain_id, e);
                    return;
                }
                
                // Update counter
                let current_chains = *self.state.total_chains.get();
                self.state.total_chains.set(current_chains + 1);
                
                linera_sdk::log::info!("World chain {} registered via message with region {}", chain_id, world_region);
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save hub state");
    }
}