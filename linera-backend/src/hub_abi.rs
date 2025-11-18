use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{ContractAbi, ServiceAbi, ChainId, ApplicationId};
use serde::{Serialize, Deserialize};

pub struct RpgHubAbi;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubParameters {
    pub max_achievements: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HubOperation {
    /// Submit an achievement from a world chain to the hub
    SubmitAchievement {
        player_id: String,
        achievement_id: String,
        chain_id: ChainId,
        timestamp: u64,
        metadata: String,  // JSON string with achievement details
    },
    /// Register a world chain with the hub
    RegisterWorldChain {
        chain_id: ChainId,
        world_region: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HubMessage {
    /// Achievement submission from a world chain
    AchievementSubmitted {
        player_id: String,
        achievement_id: String,
        chain_id: ChainId,
        timestamp: u64,
        metadata: String,
    },
    /// World chain registration
    WorldChainRegistered {
        chain_id: ChainId,
        world_region: String,
    },
}

impl ContractAbi for RpgHubAbi {
    type Operation = HubOperation;
    type Response = ();
}

impl ServiceAbi for RpgHubAbi {
    type Query = Request;
    type QueryResponse = Response;
}