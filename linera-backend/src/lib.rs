// RPG Game ABI
// SPDX-License-Identifier: MIT

use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json;

pub struct RpgGameAbi;

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

impl ContractAbi for RpgGameAbi {
    type Operation = RpgGameOperation;
    type Response = ();

    /// How the `Operation` is deserialized
    fn deserialize_operation(operation: Vec<u8>) -> Result<Self::Operation, String> {
        bcs::from_bytes(&operation)
            .map_err(|e| format!("BCS deserialization error {e:?} for operation {operation:?}"))
    }

    /// How the `Operation` is serialized
    fn serialize_operation(operation: &Self::Operation) -> Result<Vec<u8>, String> {
        bcs::to_bytes(operation)
            .map_err(|e| format!("BCS serialization error {e:?} for operation {operation:?}"))
    }

    /// How the `Response` is deserialized
    fn deserialize_response(response: Vec<u8>) -> Result<Self::Response, String> {
        bcs::from_bytes(&response)
            .map_err(|e| format!("BCS deserialization error {e:?} for response {response:?}"))
    }

    /// How the `Response` is serialized
    fn serialize_response(response: Self::Response) -> Result<Vec<u8>, String> {
        bcs::to_bytes(&response)
            .map_err(|e| format!("BCS serialization error {e:?} for response {response:?}"))
    }
}

impl ServiceAbi for RpgGameAbi {
    type Query = Request;
    type QueryResponse = Response;

    /// How the `Query` is deserialized
    fn deserialize_query(query: Vec<u8>) -> Result<Self::Query, String> {
        serde_json::from_slice(&query)
            .map_err(|e| format!("JSON deserialization error {e:?} for query {query:?}"))
    }

    /// How the `Query` is serialized
    fn serialize_query(query: &Self::Query) -> Result<Vec<u8>, String> {
        serde_json::to_vec(query)
            .map_err(|e| format!("JSON serialization error {e:?} for query {query:?}"))
    }

    /// How the `QueryResponse` is deserialized
    fn deserialize_query_response(response: Vec<u8>) -> Result<Self::QueryResponse, String> {
        serde_json::from_slice(&response)
            .map_err(|e| format!("JSON deserialization error {e:?} for response {response:?}"))
    }

    /// How the `QueryResponse` is serialized
    fn serialize_query_response(response: Self::QueryResponse) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&response)
            .map_err(|e| format!("JSON serialization error {e:?} for response {response:?}"))
    }
}

#[cfg(test)]
mod tests;