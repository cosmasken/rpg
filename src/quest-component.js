import {entity} from "./entity.js";


export const quest_component = (() => {

  const _TITLE = 'Welcome Adventurer!';
  const _TEXT = `Welcome to Honeywood adventurer, I see you're the chosen one and also the dragon born and whatever else, you're going to save the world! Also bring the rings back to mordor and defeat the evil dragon, and all the other things. But first, I must test you with some meaningless bullshit tasks that every rpg makes you do to waste time. Go kill like uh 30 ghosts and collect their eyeballs or something. Also go get my drycleaning and pick up my kids from daycare.`;

  class QuestComponent extends entity.Component {
    constructor() {
      super();

      const e = document.getElementById('quest-ui');
      e.style.visibility = 'hidden';
    }

    InitComponent() {
      this._RegisterHandler('input.picked', (m) => this._OnPicked(m));
    }

    async _OnPicked(msg) {
      // HARDCODE A QUEST
      const quest = {
        id: 'foo',
        title: _TITLE,
        text: _TEXT,
        completed: false,
        progress: 0,
      };
      
      // Try to save to blockchain if available
      const blockchainManager = this.GetComponent('BlockchainManager');
      if (blockchainManager && blockchainManager.isConnected) {
        // Try to load existing quests first
        const existingQuests = await blockchainManager.loadQuests(this._parent.Name) || [];
        const updatedQuests = [...existingQuests, quest];
        
        // Save updated quests to blockchain (fire and forget)
        blockchainManager.saveQuests(this._parent.Name, updatedQuests).catch(err => {
          console.error('Failed to save quests to blockchain:', err);
        });
      }
      
      this._AddQuestToJournal(quest);
    }

    _AddQuestToJournal(quest) {
      const ui = this.FindEntity('ui').GetComponent('UIController');
      ui.AddQuest(quest);
    }
    
    // Method to update quest progress
    async UpdateQuestProgress(questId, progress, completed = false) {
      const ui = this.FindEntity('ui').GetComponent('UIController');
      
      // Try to save to blockchain if available
      const blockchainManager = this.GetComponent('BlockchainManager');
      if (blockchainManager && blockchainManager.isConnected) {
        // Load existing quests
        const existingQuests = await blockchainManager.loadQuests(this._parent.Name) || [];
        
        // Update the specific quest
        const updatedQuests = existingQuests.map(quest => {
          if (quest.id === questId) {
            return {
              ...quest,
              progress: progress,
              completed: completed
            };
          }
          return quest;
        });
        
        // Save updated quests to blockchain (fire and forget)
        blockchainManager.saveQuests(this._parent.Name, updatedQuests).catch(err => {
          console.error('Failed to save quests to blockchain:', err);
        });
      }
      
      // Update UI if needed
      ui.UpdateQuestProgress(questId, progress, completed);
    }
  };

  return {
      QuestComponent: QuestComponent,
  };
})();