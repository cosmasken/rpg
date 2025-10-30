// RPG Game Contract
// SPDX-License-Identifier: MIT

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use rpg_game::{RpgGameAbi, RpgGameOperation};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use serde_json;
use state::{InventoryData, PlayerData, RpgGameState};

pub struct RpgGameContract {
    state: RpgGameState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(RpgGameContract);

impl WithContractAbi for RpgGameContract {
    type Abi = RpgGameAbi;
}

impl Contract for RpgGameContract {
    type Message = ();
    type InstantiationArgument = ();
    type Parameters = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = RpgGameState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        RpgGameContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: ()) {
        // Validate that the application parameters were configured correctly.
        self.runtime.application_parameters();
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
                
                if let Err(e) = self.state.player_states.insert(&player_id, player_data).await {
                    linera_sdk::log::error!("Failed to save player state to blockchain for player {}: {}", player_id, e);
                }
            }
            RpgGameOperation::SaveInventory { player_id, inventory } => {
                // Parse the inventory JSON string into the proper structure
                let inventory_data: Vec<crate::state::InventoryItem> = 
                    match serde_json::from_str(&inventory) {
                        Ok(data) => data,
                        Err(e) => {
                            linera_sdk::log::error!("Failed to parse inventory JSON for player {}: {}", player_id, e);
                            return; // Exit the operation early if JSON parsing fails
                        }
                    };
                
                let inventory_struct = InventoryData {
                    items: inventory_data,
                };
                
                if let Err(e) = self.state.player_inventories.insert(&player_id, inventory_struct).await {
                    linera_sdk::log::error!("Failed to save inventory to blockchain for player {}: {}", player_id, e);
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: ()) {
        // For now, we don't support cross-chain messages, but we could extend
        // this to support player transfers between chains, item trading, etc.
        // For now, we just log a warning instead of panicking
        linera_sdk::log::warn!("RPG game application received a cross-chain message, but these are not supported yet");
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}