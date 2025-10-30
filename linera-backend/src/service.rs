// RPG Game Service
// SPDX-License-Identifier: MIT

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{Context, EmptySubscription, Object, Request, Response, Schema};
use rpg_game::RpgGameOperation;
use linera_sdk::{linera_base_types::WithServiceAbi, views::View, Service, ServiceRuntime};

use self::state::{PlayerData, RpgGameState};

pub struct RpgGameService {
    state: Arc<RpgGameState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(RpgGameService);

impl WithServiceAbi for RpgGameService {
    type Abi = rpg_game::RpgGameAbi;
}

impl Service for RpgGameService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = RpgGameState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        RpgGameService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            MutationRoot {
                runtime: self.runtime.clone(),
            },
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}

struct QueryRoot {
    state: Arc<RpgGameState>,
}

#[Object]
impl QueryRoot {
    async fn player_state(&self, player_id: String) -> Option<PlayerData> {
        self.state.player_states.get(&player_id).await
            .expect("Failed to get player state")
    }

    async fn inventory(&self, player_id: String) -> Option<String> {
        let inventory = self.state.player_inventories.get(&player_id).await
            .expect("Failed to get inventory");
        
        match inventory {
            Some(inv) => {
                // Convert inventory to JSON string
                match serde_json::to_string(&inv.items) {
                    Ok(json_str) => Some(json_str),
                    Err(_) => None,
                }
            },
            None => None,
        }
    }
}

struct MutationRoot {
    runtime: Arc<ServiceRuntime<RpgGameService>>,
}

#[Object]
impl MutationRoot {
    async fn save_player_state(
        &self,
        player_id: String,
        health: u64,
        max_health: u64,
        strength: u64,
        wisdomness: u64,
        benchpress: u64,
        curl: u64,
        experience: u64,
        level: u64,
    ) -> [u8; 0] {
        let operation = RpgGameOperation::SavePlayerState {
            player_id,
            health,
            max_health,
            strength,
            wisdomness,
            benchpress,
            curl,
            experience,
            level,
        };
        self.runtime.schedule_operation(&operation);
        []
    }

    async fn save_inventory(
        &self,
        player_id: String,
        inventory: String,  // JSON string
    ) -> [u8; 0] {
        let operation = RpgGameOperation::SaveInventory { player_id, inventory };
        self.runtime.schedule_operation(&operation);
        []
    }
}