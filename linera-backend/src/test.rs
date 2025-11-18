#[cfg(test)]
mod tests {
    use super::*;
    use linera_sdk::{test::TestBuilder, BaseLayer, ChainId};
    use serde_json::json;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn test_rpg_game_lifecycle() {
        // Create a test environment with two chains
        let (mut builder, _committee) = TestBuilder::new()
            .with_base_layer(BaseLayer::Simulator)
            .with_nb_chains(2)
            .build();
        
        let chain1 = ChainId::root(0);
        let chain2 = ChainId::root(1);

        // Deploy the application to both chains
        let app = builder.publish_and_create::<rpg_game::RpgGameAbi, String, String, _>(
            chain1,
            "world1".to_string(),
            "world1".to_string(),
            &(),
            &mut BTreeMap::default(),
        ).await.unwrap();

        // Test saving player state
        let player_id = "player1".to_string();
        let player_data = rpg_game::PlayerState {
            health: 100,
            max_health: 100,
            strength: 10,
            wisdomness: 8,
            benchpress: 5,
            curl: 3,
            experience: 0,
            level: 1,
        };
        
        let inventory = json!([
            {"slot": "weapon", "item_id": "sword1", "params": {}},
            {"slot": "armor", "item_id": "shield1", "params": {}},
        ]).to_string();
        
        let quests = json!([
            {"id": "quest1", "title": "First Quest", "text": "Complete the first quest", "completed": false, "progress": 0},
        ]).to_string();

        // Save player data
        builder.call_application(chain1, app, &rpg_game::RpgGameOperation::SavePlayerState {
            player_id: player_id.clone(),
            health: player_data.health,
            max_health: player_data.max_health,
            strength: player_data.strength,
            wisdomness: player_data.wisdomness,
            benchpress: player_data.benchpress,
            curl: player_data.curl,
            experience: player_data.experience,
            level: player_data.level,
        }).await.unwrap();

        // Save inventory
        builder.call_application(chain1, app, &rpg_game::RpgGameOperation::SaveInventory {
            player_id: player_id.clone(),
            inventory: inventory.clone(),
        }).await.unwrap();

        // Save quests
        builder.call_application(chain1, app, &rpg_game::RpgGameOperation::SaveQuests {
            player_id: player_id.clone(),
            quests: quests.clone(),
        }).await.unwrap();

        // Verify data was saved correctly
        let state = builder.view(chain1, app).await.unwrap();
        assert_eq!(state.player_states.get(&player_id).await.unwrap().unwrap().health, 100);
        assert_eq!(state.player_inventories.get(&player_id).await.unwrap().unwrap().items.len(), 2);
        assert_eq!(state.player_quests.get(&player_id).await.unwrap().unwrap().len(), 1);

        // Record a battle
        builder.call_application(chain1, app, &rpg_game::RpgGameOperation::RecordBattle {
            battle_id: "battle1".to_string(),
            player_id: player_id.clone(),
            opponent: "goblin".to_string(),
            player_result: 2, // win
            damage_dealt: 50,
            damage_taken: 10,
            experience_gained: 100,
        }).await.unwrap();

        // Verify battle was recorded
        let state = builder.view(chain1, app).await.unwrap();
        assert_eq!(state.battle_records.get(&"battle1".to_string()).await.unwrap().unwrap().result, 2);
    }

    #[tokio::test]
    async fn test_cross_chain_player_transfer() {
        // Test cross-chain player transfer functionality
        let (mut builder, _committee) = TestBuilder::new()
            .with_base_layer(BaseLayer::Simulator)
            .with_nb_chains(2)
            .build();
        
        let chain1 = ChainId::root(0);
        let chain2 = ChainId::root(1);

        // Deploy the application to both chains with different world regions
        let app1 = builder.publish_and_create::<rpg_game::RpgGameAbi, String, String, _>(
            chain1,
            "world1".to_string(),
            "world1".to_string(),
            &(),
            &mut BTreeMap::default(),
        ).await.unwrap();

        let app2 = builder.publish_and_create::<rpg_game::RpgGameAbi, String, String, _>(
            chain2,
            "world2".to_string(),
            "world2".to_string(),
            &(),
            &mut BTreeMap::default(),
        ).await.unwrap();

        let player_id = "transferring_player".to_string();
        
        // Set up initial player state on chain1
        let player_data = rpg_game::PlayerState {
            health: 80,
            max_health: 100,
            strength: 15,
            wisdomness: 12,
            benchpress: 7,
            curl: 5,
            experience: 150,
            level: 2,
        };
        
        let inventory = json!([
            {"slot": "weapon", "item_id": "magic_sword", "params": {}},
            {"slot": "armor", "item_id": "magic_armor", "params": {}},
        ]).to_string();
        
        let quests = json!([
            {"id": "quest1", "title": "Ongoing Quest", "text": "Continue this quest", "completed": false, "progress": 5},
        ]).to_string();

        // Save player data on chain1
        builder.call_application(chain1, app1, &rpg_game::RpgGameOperation::SavePlayerState {
            player_id: player_id.clone(),
            health: player_data.health,
            max_health: player_data.max_health,
            strength: player_data.strength,
            wisdomness: player_data.wisdomness,
            benchpress: player_data.benchpress,
            curl: player_data.curl,
            experience: player_data.experience,
            level: player_data.level,
        }).await.unwrap();

        builder.call_application(chain1, app1, &rpg_game::RpgGameOperation::SaveInventory {
            player_id: player_id.clone(),
            inventory: inventory.clone(),
        }).await.unwrap();

        builder.call_application(chain1, app1, &rpg_game::RpgGameOperation::SaveQuests {
            player_id: player_id.clone(),
            quests: quests.clone(),
        }).await.unwrap();

        // Initiate the transfer to chain2
        let auth_token = "auth_token_123".to_string();
        
        builder.call_application(chain1, app1, &rpg_game::RpgGameOperation::TransferPlayer {
            player_id: player_id.clone(),
            destination_chain: chain2,
            player_state: player_data.clone(),
            inventory: inventory.clone(),
            quests: quests.clone(),
            auth_token: auth_token.clone(),
        }).await.unwrap();

        // Process the cross-chain message on chain2
        builder.process_inbox(chain2).await.unwrap();

        // Verify player was transferred to chain2
        let state_chain2 = builder.view(chain2, app2).await.unwrap();
        let transferred_player = state_chain2.player_states.get(&player_id).await.unwrap().unwrap();
        assert_eq!(transferred_player.health, 80);
        assert_eq!(transferred_player.strength, 15);

        let transferred_inventory = state_chain2.player_inventories.get(&player_id).await.unwrap().unwrap();
        assert_eq!(transferred_inventory.items.len(), 2);

        let transferred_quests = state_chain2.player_quests.get(&player_id).await.unwrap().unwrap();
        assert_eq!(transferred_quests.len(), 1);
    }

    #[tokio::test]
    async fn test_cross_chain_guild_join() {
        // Test cross-chain guild join functionality
        let (mut builder, _committee) = TestBuilder::new()
            .with_base_layer(BaseLayer::Simulator)
            .with_nb_chains(2)
            .build();
        
        let chain1 = ChainId::root(0);
        let chain2 = ChainId::root(1);

        // Deploy the application to both chains
        let app1 = builder.publish_and_create::<rpg_game::RpgGameAbi, String, String, _>(
            chain1,
            "world1".to_string(),
            "world1".to_string(),
            &(),
            &mut BTreeMap::default(),
        ).await.unwrap();

        let app2 = builder.publish_and_create::<rpg_game::RpgGameAbi, String, String, _>(
            chain2,
            "world2".to_string(),
            "world2".to_string(),
            &(),
            &mut BTreeMap::default(),
        ).await.unwrap();

        let player_id = "guild_member".to_string();
        let guild_id = "adventurers_guild".to_string();

        // Create a guild on chain2 (by joining it from chain1)
        builder.call_application(chain1, app1, &rpg_game::RpgGameOperation::JoinGuild {
            player_id: player_id.clone(),
            guild_id: guild_id.clone(),
            chain_id: chain2,
        }).await.unwrap();

        // Process the cross-chain message on chain2
        builder.process_inbox(chain2).await.unwrap();

        // Verify guild was created on chain2 and player is a member
        let state_chain2 = builder.view(chain2, app2).await.unwrap();
        let guild = state_chain2.guilds.get(&guild_id).await.unwrap().unwrap();
        assert!(guild.members.contains(&player_id));

        // Verify player's guild mapping was updated
        let player_guild = state_chain2.player_guilds.get(&player_id).await.unwrap().unwrap();
        assert_eq!(player_guild, guild_id);
    }

    #[tokio::test]
    async fn test_battle_recording() {
        // Test verifiable battle system
        let (mut builder, _committee) = TestBuilder::new()
            .with_base_layer(BaseLayer::Simulator)
            .with_nb_chains(1)
            .build();
        
        let chain1 = ChainId::root(0);

        let app = builder.publish_and_create::<rpg_game::RpgGameAbi, String, String, _>(
            chain1,
            "world1".to_string(),
            "world1".to_string(),
            &(),
            &mut BTreeMap::default(),
        ).await.unwrap();

        let player_id = "battle_tester".to_string();

        // Record multiple battles
        builder.call_application(chain1, app, &rpg_game::RpgGameOperation::RecordBattle {
            battle_id: "battle1".to_string(),
            player_id: player_id.clone(),
            opponent: "orc".to_string(),
            player_result: 2, // win
            damage_dealt: 45,
            damage_taken: 20,
            experience_gained: 50,
        }).await.unwrap();

        builder.call_application(chain1, app, &rpg_game::RpgGameOperation::RecordBattle {
            battle_id: "battle2".to_string(),
            player_id: player_id.clone(),
            opponent: "troll".to_string(),
            player_result: 0, // loss
            damage_dealt: 10,
            damage_taken: 60,
            experience_gained: 10,
        }).await.unwrap();

        // Verify battles were recorded
        let state = builder.view(chain1, app).await.unwrap();
        
        let battle1 = state.battle_records.get(&"battle1".to_string()).await.unwrap().unwrap();
        assert_eq!(battle1.opponent, "orc");
        assert_eq!(battle1.result, 2); // win

        let battle2 = state.battle_records.get(&"battle2".to_string()).await.unwrap().unwrap();
        assert_eq!(battle2.opponent, "troll");
        assert_eq!(battle2.result, 0); // loss

        // Verify player's battle history was updated
        let player_battles = state.player_battles.get(&player_id).await.unwrap().unwrap();
        assert_eq!(player_battles.len(), 2);
        assert!(player_battles.contains(&"battle1".to_string()));
        assert!(player_battles.contains(&"battle2".to_string()));
    }
}