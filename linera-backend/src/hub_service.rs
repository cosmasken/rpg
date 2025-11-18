// RPG Hub Service
// SPDX-License-Identifier: MIT

#![cfg_attr(target_arch = "wasm32", no_main)]

mod hub_state;
use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{linera_base_types::WithServiceAbi, views::View, Service, ServiceRuntime};

use hub_state::{HubState, PlayerAchievement};

pub struct RpgHubService {
    state: Arc<HubState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(RpgHubService);

impl WithServiceAbi for RpgHubService {
    type Abi = crate::hub_abi::RpgHubAbi;
}

impl Service for RpgHubService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = HubState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        RpgHubService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            HubQueryRoot {
                state: self.state.clone(),
            },
            HubMutationRoot {
                runtime: self.runtime.clone(),
            },
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}

struct HubQueryRoot {
    state: Arc<HubState>,
}

#[Object]
impl HubQueryRoot {
    async fn player_achievements(&self, player_id: String) -> Option<Vec<PlayerAchievement>> {
        self.state.player_achievements.get(&player_id).await
            .expect("Failed to get player achievements")
    }

    async fn world_chains_count(&self) -> u64 {
        *self.state.total_chains.get()
    }

    async fn total_achievements_count(&self) -> u64 {
        *self.state.total_achievements.get()
    }

    async fn world_chain_info(&self, chain_id: String) -> Option<hub_state::WorldChainInfo> {
        use linera_sdk::linera_base_types::ChainId;
        // Parse the chain ID string to ChainId (this would need proper parsing in real implementation)
        // For now, we'll assume it's used differently
        // This is a simplified version - would need proper ChainId deserialization
        None  // Placeholder - would need to implement proper ChainId parsing
    }

    async fn achievement_records(&self, achievement_id: String) -> Option<Vec<hub_state::AchievementRecord>> {
        self.state.all_achievements.get(&achievement_id).await
            .expect("Failed to get achievement records")
    }
}

struct HubMutationRoot {
    runtime: Arc<ServiceRuntime<RpgHubService>>,
}

#[Object]
impl HubMutationRoot {
    async fn submit_achievement(
        &self,
        player_id: String,
        achievement_id: String,
        chain_id_str: String,  // Would need to convert to ChainId
        timestamp: u64,
        metadata: String,
    ) -> [u8; 0] {
        // This would need proper ChainId conversion
        // For the example, we'll use a placeholder
        // In a real implementation, we'd properly parse the chain_id_str
        let operation = crate::hub_abi::HubOperation::SubmitAchievement {
            player_id,
            achievement_id,
            chain_id: self.runtime.chain_id(),  // Using current chain as placeholder
            timestamp,
            metadata,
        };
        self.runtime.schedule_operation(&operation);
        []
    }

    async fn register_world_chain(
        &self,
        chain_id_str: String,  // Would need to convert to ChainId
        world_region: String,
    ) -> [u8; 0] {
        // This would need proper ChainId conversion
        let operation = crate::hub_abi::HubOperation::RegisterWorldChain {
            chain_id: self.runtime.chain_id(),  // Using current chain as placeholder
            world_region,
        };
        self.runtime.schedule_operation(&operation);
        []
    }
}