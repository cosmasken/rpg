import * as linera from '@linera/client';
import { entity } from './entity.js';

/**
 * Proper Linera Client Integration following official documentation
 */
export const linera_client = (() => {
  
  class LineraClient extends entity.Component {
    constructor(params) {
      super();
      this._params = params;
      this._client = null;
      this._application = null;
      this._wallet = null;
      this._signer = null;
      this._chainId = null;
      this._owner = null;
      this._isConnected = false;
      this._faucet = null;
    }

    async InitComponent() {
      try {
        await this._initializeLinera();
        this._updateUI();
        this._setupNotifications();
      } catch (error) {
        console.error('Failed to initialize Linera client:', error);
        this._updateUI();
      }
    }

    async _initializeLinera() {
      console.log('Initializing Linera client...');
      
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
      const applicationId = import.meta.env?.LINERA_APPLICATION_ID;
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

    _updateUI() {
      // Update blockchain status
      const statusElement = document.getElementById('blockchain-status-text');
      if (statusElement) {
        statusElement.textContent = this._isConnected ? 'Connected' : 'Not Connected';
        statusElement.style.color = this._isConnected ? '#00FF00' : '#FF0000';
      }

      // Update chain ID
      const chainElement = document.getElementById('chain-id-text');
      if (chainElement) {
        chainElement.textContent = this._chainId || '-';
      }

      // Update owner address (add this to UI if needed)
      const ownerElement = document.getElementById('owner-text');
      if (ownerElement && this._owner) {
        ownerElement.textContent = this._owner.substring(0, 8) + '...';
      }

      // Update world region
      if (this._isConnected && this._application) {
        this._getWorldRegion();
      } else {
        const regionElement = document.getElementById('world-region-text');
        if (regionElement) {
          regionElement.textContent = 'world1'; // Default region
        }
      }
    }

    async _getWorldRegion() {
      try {
        const response = await this._application.query('{ "query": "query { worldRegion }" }');
        const data = JSON.parse(response);
        const worldRegion = data.data?.worldRegion || 'world1';
        
        const regionElement = document.getElementById('world-region-text');
        if (regionElement) {
          regionElement.textContent = worldRegion;
        }
      } catch (error) {
        console.warn('Could not get world region:', error);
        const regionElement = document.getElementById('world-region-text');
        if (regionElement) {
          regionElement.textContent = 'world1';
        }
      }
    }

    _setupNotifications() {
      if (!this._client) return;

      this._client.onNotification(notification => {
        console.log('Received notification:', notification);
        
        // Check for new blocks
        if (notification.reason?.NewBlock) {
          console.log('New block:', notification.reason.NewBlock);
          
          // Broadcast to other components
          this.Broadcast({
            topic: 'blockchain.newBlock',
            block: notification.reason.NewBlock
          });
          
          // Update world region in case it changed
          if (this._application) {
            this._getWorldRegion();
          }
        }
      });
    }

    // Public API methods following GraphQL patterns
    async savePlayerState(playerId, playerData) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot save player state');
        return false;
      }

      try {
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

        const response = await this._application.query(`{ "query": "${mutation}" }`);
        console.log('Player state saved:', response);
        return true;
      } catch (error) {
        console.error('Error saving player state:', error);
        return false;
      }
    }

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

        const response = await this._application.query(`{ "query": "${query}" }`);
        const data = JSON.parse(response);
        return data.data?.playerState;
      } catch (error) {
        console.error('Error loading player state:', error);
        return null;
      }
    }

    async recordBattle(battleData) {
      if (!this._isConnected || !this._application) {
        console.warn('Not connected to blockchain, cannot record battle');
        return false;
      }

      try {
        const mutation = `mutation {
          recordBattle(
            battleId: "${battleData.battleId}",
            playerId: "${battleData.playerId}",
            opponent: "${battleData.opponent}",
            playerResult: ${battleData.result},
            damageDealt: ${battleData.damageDealt},
            damageTaken: ${battleData.damageTaken},
            experienceGained: ${battleData.experienceGained}
          )
        }`;

        const response = await this._application.query(`{ "query": "${mutation}" }`);
        console.log('Battle recorded:', response);
        return true;
      } catch (error) {
        console.error('Error recording battle:', error);
        return false;
      }
    }

    // Getters
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
    LineraClient: LineraClient
  };
})();
