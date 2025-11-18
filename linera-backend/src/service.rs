// RPG Game Service
// SPDX-License-Identifier: MIT

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{Context, EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::linera_base_types::ChainId;
use rpg_game::{RpgGameOperation, PlayerState};
use linera_sdk::{linera_base_types::WithServiceAbi, views::View, Service, ServiceRuntime};

use self::state::{PlayerData, RpgGameState, BattleRecord, GuildData};

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

    async fn quests(&self, player_id: String) -> Option<String> {
        let quests = self.state.player_quests.get(&player_id).await
            .expect("Failed to get quests");

        match quests {
            Some(quest_list) => {
                // Convert quests to JSON string
                match serde_json::to_string(&quest_list) {
                    Ok(json_str) => Some(json_str),
                    Err(_) => None,
                }
            },
            None => None,
        }
    }

    async fn battle_record(&self, battle_id: String) -> Option<BattleRecord> {
        self.state.battle_records.get(&battle_id).await
            .expect("Failed to get battle record")
    }

    async fn player_battles(&self, player_id: String) -> Option<Vec<String>> {
        self.state.player_battles.get(&player_id).await
            .expect("Failed to get player battles")
    }

    async fn guild(&self, guild_id: String) -> Option<GuildData> {
        self.state.guilds.get(&guild_id).await
            .expect("Failed to get guild")
    }

    async fn player_guild(&self, player_id: String) -> Option<String> {
        self.state.player_guilds.get(&player_id).await
            .expect("Failed to get player guild")
    }

    async fn world_region(&self) -> String {
        self.state.world_region.get().clone()
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

    async fn save_quests(
        &self,
        player_id: String,
        quests: String,  // JSON string
    ) -> [u8; 0] {
        let operation = RpgGameOperation::SaveQuests { player_id, quests };
        self.runtime.schedule_operation(&operation);
        []
    }

    async fn transfer_player(
        &self,
        player_id: String,
        destination_chain: ChainId,
        health: u64,
        max_health: u64,
        strength: u64,
        wisdomness: u64,
        benchpress: u64,
        curl: u64,
        experience: u64,
        level: u64,
        inventory: String,
        quests: String,
        auth_token: String,
    ) -> [u8; 0] {
        let player_state = PlayerState {
            health,
            max_health,
            strength,
            wisdomness,
            benchpress,
            curl,
            experience,
            level,
        };

        let operation = RpgGameOperation::TransferPlayer {
            player_id,
            destination_chain,
            player_state,
            inventory,
            quests,
            auth_token,
        };
        self.runtime.schedule_operation(&operation);
        []
    }

    async fn join_guild(
        &self,
        player_id: String,
        guild_id: String,
        chain_id: ChainId,
    ) -> [u8; 0] {
        let operation = RpgGameOperation::JoinGuild {
            player_id,
            guild_id,
            chain_id,
        };
        self.runtime.schedule_operation(&operation);
        []
    }

    async fn record_battle(
        &self,
        battle_id: String,
        player_id: String,
        opponent: String,
        player_result: u64,
        damage_dealt: u64,
        damage_taken: u64,
        experience_gained: u64,
    ) -> [u8; 0] {
        let operation = RpgGameOperation::RecordBattle {
            battle_id,
            player_id,
            opponent,
            player_result,
            damage_dealt,
            damage_taken,
            experience_gained,
        };
        self.runtime.schedule_operation(&operation);
        []
    }
}