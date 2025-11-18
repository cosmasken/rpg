import * as THREE from 'https://cdn.jsdelivr.net/npm/three@0.118.1/build/three.module.js';
import {entity} from './entity.js';

/**
 * Advanced Blockchain integration module for the RPG game.
 * Combines the best features of both implementations:
 * 1. Full wallet/signer setup capability (from linera_client)
 * 2. Clean GraphQL interface (from blockchain_integration)
 * 3. Comprehensive feature support (both)
 */
export const blockchain_integration = (() => {

  class BlockchainManager extends entity.Component {
    constructor(params) {
      super();
      this._params = params;
      this._client = null;
      this._application = null;
      this._wallet = null;
      this._signer = null;
      this._chainId = null;
      this._owner = null;
      this._faucet = null;
      this._isConnected = false;
    }

    async InitComponent() {
      // Attempt to initialize blockchain connection with fallback
      try {
        // First, try to use existing Linera environment
        await this._initializeFromEnvironment();
        if (!this._isConnected) {
          // If that fails, fall back to creating our own client
          await this._initializeBlockchainClient();
        }
        
        console.log('Blockchain integration initialized successfully');
        this._updateBlockchainStatusUI(this._isConnected);

        // Load world region if connected
        if (this._isConnected) {
          const worldRegion = await this.getWorldRegion();
          this._updateWorldRegionUI(worldRegion);
        }
      } catch (error) {
        console.error('Failed to initialize blockchain integration:', error);
        this._updateBlockchainStatusUI(false);
        // Continue without blockchain if initialization fails
      }
    }

    async _initializeFromEnvironment() {
      // Check if Linera client is available in the browser environment
      if (typeof window !== 'undefined' && typeof window.linera !== 'undefined') {
        try {
          // Get the application ID from environment or config
          const appId = this._params.applicationId || import.meta.env?.LINERA_APPLICATION_ID;
          if (!appId) {
            console.warn('No Linera application ID provided, running without blockchain integration');
            return;
          }

          // Initialize the Linera client if it's available
          if (window.linera && typeof window.linera.initialize === 'function') {
            await window.linera.initialize();
          }

          // Create a client instance
          if (window.linera && typeof window.linera.client !== 'undefined') {
            this._application = await window.linera.client.application(appId);
            this._isConnected = true;
            console.log('Connected to Linera application:', appId);
            
            // Try to get chain ID if available
            if (this._application && this._application.chainId) {
              this._chainId = this._application.chainId;
            }
          }
        } catch (error) {
          console.warn('Could not initialize from environment, will try client fallback:', error);
        }
      }
    }

    async _initializeBlockchainClient() {
      console.log('Initializing Linera client fallback...');
      
      // Import Linera client dynamically
      const linera = await import('@linera/client');
      
      // Initialize Linera WebAssembly
      await linera.default();

      // Get faucet URL from environment or use local default
      const faucetUrl = import.meta.env?.LINERA_FAUCET_URL || 'http://localhost:8080';

      // Create faucet connection
      this._faucet = new linera.Faucet(faucetUrl);

      // Create wallet from faucet
      this._wallet = await this._faucet.createWallet();

      // Create random signer (in production, you'd want to persist this)
      this._signer = linera.PrivateKeySigner.createRandom();

      // Get owner address
      this._owner = await this._signer.address();

      // Claim chain from faucet
      this._chainId = await this._faucet.claimChain(this._wallet, this._owner);

      // Create client
      this._client = new linera.Client(this._wallet, this._signer);

      // Connect to application if available
      const applicationId = this._params.applicationId || import.meta.env?.LINERA_APPLICATION_ID;
      if (applicationId) {
        try {
          this._application = await this._client.application(applicationId);
          console.log('Connected to application:', applicationId);
        } catch (error) {
          console.warn('Could not connect to application:', error);
        }
      }

      this._isConnected = true;
      console.log('Linera client initialized successfully');
      console.log('Owner:', this._owner);
      console.log('Chain ID:', this._chainId);
    }

    _updateBlockchainStatusUI(connected) {
      const statusElement = document.getElementById('blockchain-status-text');
      if (statusElement) {
        statusElement.textContent = connected ? 'Connected' : 'Not Connected';
        statusElement.style.color = connected ? '#00FF00' : '#FF0000';
      }

      // Update chain ID if available
      const chainElement = document.getElementById('chain-id-text');
      if (chainElement) {
        chainElement.textContent = this._chainId || 'Unknown';
      }

      // Update owner address if available
      const ownerElement = document.getElementById('owner-text');
      if (ownerElement && this._owner) {
        ownerElement.textContent = this._owner.substring(0, 8) + '...';
      }
    }

    _updateWorldRegionUI(worldRegion) {
      const regionElement = document.getElementById('world-region-text');
      if (regionElement) {
        regionElement.textContent = worldRegion || 'Unknown';
      }
    }

    /**
     * Save player state to the blockchain
     * @param {string} playerId - Unique identifier for the player
     * @param {Object} playerData - Player state to save
     */
    async savePlayerState(playerId, playerData) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot save player state');
        return false;
      }

      try {
        // Prepare the mutation to save player state
        const mutation = `mutation {
          savePlayerState(
            playerId: "${playerId}",
            health: ${playerData.health},
            maxHealth: ${playerData.maxHealth},
            strength: ${playerData.strength},
            wisdomness: ${playerData.wisdomness},
            benchpress: ${playerData.benchpress},
            curl: ${playerData.curl},
            experience: ${playerData.experience},
            level: ${playerData.level}
          )
        }`;

        const response = await this._application.query({ query: mutation });
        console.log('Player state saved to blockchain:', response);
        return true;
      } catch (error) {
        console.error('Error saving player state to blockchain:', error);
        return false;
      }
    }

    /**
     * Load player state from the blockchain
     * @param {string} playerId - Unique identifier for the player
     */
    async loadPlayerState(playerId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot load player state');
        return null;
      }

      try {
        const query = `query {
          playerState(playerId: "${playerId}") {
            health,
            maxHealth,
            strength,
            wisdomness,
            benchpress,
            curl,
            experience,
            level
          }
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.playerState;
        console.log('Player state loaded from blockchain:', data);
        return data;
      } catch (error) {
        console.error('Error loading player state from blockchain:', error);
        return null;
      }
    }

    /**
     * Save inventory to the blockchain
     * @param {string} playerId - Unique identifier for the player
     * @param {Array} inventory - Array of inventory items
     */
    async saveInventory(playerId, inventory) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot save inventory');
        return false;
      }

      try {
        // Convert inventory to a format suitable for GraphQL
        const inventoryJson = JSON.stringify(inventory);
        const mutation = `mutation {
          saveInventory(
            playerId: "${playerId}",
            inventory: ${JSON.stringify(inventoryJson)}
          )
        }`;

        const response = await this._application.query({ query: mutation });
        console.log('Inventory saved to blockchain:', response);
        return true;
      } catch (error) {
        console.error('Error saving inventory to blockchain:', error);
        return false;
      }
    }

    /**
     * Load inventory from the blockchain
     * @param {string} playerId - Unique identifier for the player
     */
    async loadInventory(playerId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot load inventory');
        return null;
      }

      try {
        const query = `query {
          inventory(playerId: "${playerId}")
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.inventory;
        console.log('Inventory loaded from blockchain:', data);
        return data;
      } catch (error) {
        console.error('Error loading inventory from blockchain:', error);
        return null;
      }
    }

    /**
     * Save quests to the blockchain
     * @param {string} playerId - Unique identifier for the player
     * @param {Array} quests - Array of quest objects
     */
    async saveQuests(playerId, quests) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot save quests');
        return false;
      }

      try {
        // Convert quests to a format suitable for GraphQL
        const questsJson = JSON.stringify(quests);
        const mutation = `mutation {
          saveQuests(
            playerId: "${playerId}",
            quests: ${JSON.stringify(questsJson)}
          )
        }`;

        const response = await this._application.query({ query: mutation });
        console.log('Quests saved to blockchain:', response);
        return true;
      } catch (error) {
        console.error('Error saving quests to blockchain:', error);
        return false;
      }
    }

    /**
     * Load quests from the blockchain
     * @param {string} playerId - Unique identifier for the player
     */
    async loadQuests(playerId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot load quests');
        return null;
      }

      try {
        const query = `query {
          quests(playerId: "${playerId}")
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.quests;
        console.log('Quests loaded from blockchain:', data);
        return data;
      } catch (error) {
        console.error('Error loading quests from blockchain:', error);
        return null;
      }
    }

    /**
     * Transfer player to another chain with complete state
     * @param {string} playerId - Unique identifier for the player
     * @param {string} destinationChain - The destination chain ID
     * @param {Object} playerData - Player state to transfer
     * @param {Array} inventory - Array of inventory items
     * @param {Array} quests - Array of quest objects
     * @param {string} authToken - Authentication token for the transfer
     */
    async transferPlayer(playerId, destinationChain, playerData, inventory, quests, authToken) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot transfer player');
        return false;
      }

      try {
        const inventoryJson = JSON.stringify(inventory);
        const questsJson = JSON.stringify(quests);

        const mutation = `mutation {
          transferPlayer(
            playerId: "${playerId}",
            destinationChain: "${destinationChain}",
            health: ${playerData.health},
            maxHealth: ${playerData.maxHealth},
            strength: ${playerData.strength},
            wisdomness: ${playerData.wisdomness},
            benchpress: ${playerData.benchpress},
            curl: ${playerData.curl},
            experience: ${playerData.experience},
            level: ${playerData.level},
            inventory: ${JSON.stringify(inventoryJson)},
            quests: ${JSON.stringify(questsJson)},
            authToken: "${authToken}"
          )
        }`;

        const response = await this._application.query({ query: mutation });
        console.log('Player transfer initiated:', response);
        return true;
      } catch (error) {
        console.error('Error initiating player transfer:', error);
        return false;
      }
    }

    /**
     * Join a guild on another chain
     * @param {string} playerId - Unique identifier for the player
     * @param {string} guildId - The guild ID to join
     * @param {string} chainId - The chain ID where the guild exists
     */
    async joinGuild(playerId, guildId, chainId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot join guild');
        return false;
      }

      try {
        const mutation = `mutation {
          joinGuild(
            playerId: "${playerId}",
            guildId: "${guildId}",
            chainId: "${chainId}"
          )
        }`;

        const response = await this._application.query({ query: mutation });
        console.log('Guild join request sent:', response);
        return true;
      } catch (error) {
        console.error('Error joining guild:', error);
        return false;
      }
    }

    /**
     * Record a battle result on the blockchain
     * @param {string} battleId - Unique identifier for the battle
     * @param {string} playerId - The player ID
     * @param {string} opponent - The opponent name/ID
     * @param {number} playerResult - 0 for loss, 1 for draw, 2 for win
     * @param {number} damageDealt - Amount of damage dealt
     * @param {number} damageTaken - Amount of damage taken
     * @param {number} experienceGained - Amount of experience gained
     */
    async recordBattle(battleId, playerId, opponent, playerResult, damageDealt, damageTaken, experienceGained) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot record battle');
        return false;
      }

      try {
        const mutation = `mutation {
          recordBattle(
            battleId: "${battleId}",
            playerId: "${playerId}",
            opponent: "${opponent}",
            playerResult: ${playerResult},
            damageDealt: ${damageDealt},
            damageTaken: ${damageTaken},
            experienceGained: ${experienceGained}
          )
        }`;

        const response = await this._application.query({ query: mutation });
        console.log('Battle recorded on blockchain:', response);
        return true;
      } catch (error) {
        console.error('Error recording battle:', error);
        return false;
      }
    }

    /**
     * Get a specific battle record
     * @param {string} battleId - The battle ID to retrieve
     */
    async getBattleRecord(battleId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot get battle record');
        return null;
      }

      try {
        const query = `query {
          battleRecord(battleId: "${battleId}") {
            battleId,
            playerId,
            opponent,
            result,
            damageDealt,
            damageTaken,
            experienceGained,
            timestamp
          }
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.battleRecord;
        console.log('Battle record retrieved:', data);
        return data;
      } catch (error) {
        console.error('Error getting battle record:', error);
        return null;
      }
    }

    /**
     * Get player's battle history
     * @param {string} playerId - The player ID
     */
    async getPlayerBattles(playerId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot get player battles');
        return null;
      }

      try {
        const query = `query {
          playerBattles(playerId: "${playerId}")
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.playerBattles;
        console.log('Player battles retrieved:', data);
        return data;
      } catch (error) {
        console.error('Error getting player battles:', error);
        return null;
      }
    }

    /**
     * Get guild information
     * @param {string} guildId - The guild ID
     */
    async getGuild(guildId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot get guild');
        return null;
      }

      try {
        const query = `query {
          guild(guildId: "${guildId}") {
            id,
            name,
            members,
            resources,
            level
          }
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.guild;
        console.log('Guild retrieved:', data);
        return data;
      } catch (error) {
        console.error('Error getting guild:', error);
        return null;
      }
    }

    /**
     * Get player's current guild
     * @param {string} playerId - The player ID
     */
    async getPlayerGuild(playerId) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot get player guild');
        return null;
      }

      try {
        const query = `query {
          playerGuild(playerId: "${playerId}")
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.playerGuild;
        console.log('Player guild retrieved:', data);
        return data;
      } catch (error) {
        console.error('Error getting player guild:', error);
        return null;
      }
    }

    /**
     * Get the world region of the current chain
     */
    async getWorldRegion() {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot get world region');
        return null;
      }

      try {
        const query = `query {
          worldRegion
        }`;

        const response = await this._application.query({ query });
        if (response.errors) {
          console.error('GraphQL errors:', response.errors);
          return null;
        }
        const data = response.data.worldRegion;
        console.log('World region retrieved:', data);
        return data;
      } catch (error) {
        console.error('Error getting world region:', error);
        return null;
      }
    }

    /**
     * Submit an achievement to the central hub
     * @param {string} playerId - The player ID
     * @param {string} achievementId - The achievement ID
     * @param {string} achievementName - The achievement name
     * @param {string} achievementDescription - The achievement description
     * @param {string} hubAppId - The hub application ID
     * @param {number} timestamp - The timestamp of the achievement
     * @param {string} metadata - Additional metadata as JSON string
     */
    async submitAchievement(playerId, achievementId, achievementName, achievementDescription, hubAppId, timestamp, metadata) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot submit achievement');
        return false;
      }

      try {
        const mutation = `mutation {
          submitAchievement(
            playerId: "${playerId}",
            achievementId: "${achievementId}",
            achievementName: "${achievementName}",
            achievementDescription: "${achievementDescription}",
            hubAppId: "${hubAppId}",
            timestamp: ${timestamp},
            metadata: ${JSON.stringify(metadata)}
          )
        }`;

        const response = await this._application.query({ query: mutation });
        console.log('Achievement submitted to hub:', response);
        return true;
      } catch (error) {
        console.error('Error submitting achievement:', error);
        return false;
      }
    }

    get isConnected() {
      return this._isConnected;
    }
    
    get chainId() {
      return this._chainId;
    }
    
    get owner() {
      return this._owner;
    }
    
    get client() {
      return this._client;
    }
    
    get application() {
      return this._application;
    }
  }

  return {
    BlockchainManager: BlockchainManager
  };
})();