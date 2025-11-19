/**
 * Model cache and lazy loading system for performance optimization
 */

import {FBXLoader} from '../libs/FBXLoader.js';
import * as THREE from '../libs/three.module.js';

class ModelCache {
  constructor() {
    this.cache = new Map();
    this.loader = new FBXLoader();
    this.loadingPromises = new Map();
  }

  async loadModel(resourceName) {
    // Return cached model if available
    if (this.cache.has(resourceName)) {
      return this.cache.get(resourceName).clone();
    }

    // Return existing loading promise if already loading
    if (this.loadingPromises.has(resourceName)) {
      const model = await this.loadingPromises.get(resourceName);
      return model.clone();
    }

    // Start loading
    const loadPromise = new Promise((resolve, reject) => {
      this.loader.load(
        './resources/monsters/FBX/' + resourceName,
        (fbx) => {
          this.cache.set(resourceName, fbx);
          this.loadingPromises.delete(resourceName);
          resolve(fbx);
        },
        undefined,
        (error) => {
          this.loadingPromises.delete(resourceName);
          reject(error);
        }
      );
    });

    this.loadingPromises.set(resourceName, loadPromise);
    const model = await loadPromise;
    return model.clone();
  }

  preloadModels(modelList) {
    return Promise.all(
      modelList.map(modelName => this.loadModel(modelName))
    );
  }
}

export const modelCache = new ModelCache();
