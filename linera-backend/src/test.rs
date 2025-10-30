// Tests for RPG Game backend
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod tests {
    use futures::FutureExt as _;
    use linera_sdk::{util::BlockingWait, views::View, Contract, Service};
    use serde_json;

    use crate::{
        contract::RpgGameContract, 
        service::RpgGameService, 
        state::{PlayerData, RpgGameState, InventoryItem, InventoryData}, 
        RpgGameOperation
    };

    #[test]
    fn test_player_state_storage() {
        let mut runtime = linera_sdk::ContractRuntime::new();
        runtime.set_application_parameters(());
        
        let mut contract = RpgGameContract {
            state: RpgGameState::load(runtime.root_view_storage_context())
                .blocking_wait()
                .expect("Failed to read from mock key value store"),
            runtime,
        };

        // Instantiate the contract
        contract
            .instantiate(())
            .now_or_never()
            .expect("Instantiation should not await anything");

        // Create a test operation to save player state
        let player_id = "test_player_1".to_string();
        let operation = RpgGameOperation::SavePlayerState {
            player_id: player_id.clone(),
            health: 100,
            max_health: 100,
            strength: 50,
            wisdomness: 5,
            benchpress: 20,
            curl: 100,
            experience: 0,
            level: 1,
        };

        // Execute the operation
        contract
            .execute_operation(operation)
            .now_or_never()
            .expect("Operation execution should not await anything");

        // Verify the state was saved by checking state directly
        let player_data = contract
            .state
            .player_states
            .get(&player_id)
            .now_or_never()
            .expect("Get should not await anything")
            .expect("Player data should exist");

        assert_eq!(player_data.health, 100);
        assert_eq!(player_data.strength, 50);
        assert_eq!(player_data.level, 1);
    }

    #[test]
    fn test_inventory_storage() {
        let mut runtime = linera_sdk::ContractRuntime::new();
        runtime.set_application_parameters(());
        
        let mut contract = RpgGameContract {
            state: RpgGameState::load(runtime.root_view_storage_context())
                .blocking_wait()
                .expect("Failed to read from mock key value store"),
            runtime,
        };

        // Instantiate the contract
        contract
            .instantiate(())
            .now_or_never()
            .expect("Instantiation should not await anything");

        // Create a test inventory
        let player_id = "test_player_2".to_string();
        let inventory_items = vec![
            crate::state::InventoryItem {
                slot: "inventory-1".to_string(),
                item_id: "sword-001".to_string(),
                params: serde_json::json!({"name": "Iron Sword", "damage": 15}),
            }
        ];
        let inventory_json = serde_json::to_string(&inventory_items).unwrap();

        let operation = RpgGameOperation::SaveInventory {
            player_id: player_id.clone(),
            inventory: inventory_json,
        };

        // Execute the operation
        contract
            .execute_operation(operation)
            .now_or_never()
            .expect("Inventory operation execution should not await anything");

        // Verify the state was saved
        let inventory_data = contract
            .state
            .player_inventories
            .get(&player_id)
            .now_or_never()
            .expect("Get should not await anything")
            .expect("Inventory data should exist");

        assert_eq!(inventory_data.items.len(), 1);
        assert_eq!(inventory_data.items[0].item_id, "sword-001");
    }

    #[test]
    fn test_player_leveling() {
        let mut runtime = linera_sdk::ContractRuntime::new();
        runtime.set_application_parameters(());
        
        let mut contract = RpgGameContract {
            state: RpgGameState::load(runtime.root_view_storage_context())
                .blocking_wait()
                .expect("Failed to read from mock key value store"),
            runtime,
        };

        // Instantiate the contract
        contract
            .instantiate(())
            .now_or_never()
            .expect("Instantiation should not await anything");

        // Test player progression
        let player_id = "leveling_test_player".to_string();
        
        // Initial state
        let initial_operation = RpgGameOperation::SavePlayerState {
            player_id: player_id.clone(),
            health: 50,
            max_health: 100,
            strength: 10,
            wisdomness: 5,
            benchpress: 8,
            curl: 12,
            experience: 0,
            level: 1,
        };

        contract
            .execute_operation(initial_operation)
            .now_or_never()
            .expect("Operation execution should not await anything");

        // Level up
        let leveled_operation = RpgGameOperation::SavePlayerState {
            player_id: player_id.clone(),
            health: 90,
            max_health: 120,
            strength: 15,
            wisdomness: 7,
            benchpress: 10,
            curl: 15,
            experience: 100,
            level: 2,
        };

        contract
            .execute_operation(leveled_operation)
            .now_or_never()
            .expect("Operation execution should not await anything");

        // Verify the updated state
        let player_data = contract
            .state
            .player_states
            .get(&player_id)
            .now_or_never()
            .expect("Get should not await anything")
            .expect("Player data should exist");

        assert_eq!(player_data.level, 2);
        assert_eq!(player_data.strength, 15);
        assert_eq!(player_data.max_health, 120);
    }
}