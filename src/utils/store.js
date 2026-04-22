import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Wrapper that mimics the Tauri v2 Store API but uses backend config_get/config_set
// This ensures useConfig hook and plugins work without any changes
export let store;

export async function initStore() {
    // The backend already loaded config.json in init_config()
    // We just create a wrapper object with the same API as Store
    store = {
        async get(key) {
            try {
                const value = await invoke('config_get', { key });
                console.log(`store.get('${key}') =`, JSON.stringify(value)?.substring(0, 200));
                return value ?? null;
            } catch (e) {
                console.error(`store.get('${key}') failed:`, e);
                return null;
            }
        },

        async set(key, value) {
            try {
                await invoke('config_set', { key, value });
            } catch (e) {
                console.error(`store.set('${key}') failed:`, e);
            }
        },

        async has(key) {
            const value = await this.get(key);
            return value !== null;
        },

        async delete(key) {
            await this.set(key, null);
        },

        async save() {
            // Backend auto-saves on every set(), so this is no-op
        },

        async reload() {
            try {
                await invoke('reload_store');
            } catch (e) {
                console.warn('reload_store failed:', e);
            }
        },

        is_empty() {
            // Synchronous check - not critical, return false as fallback
            return false;
        }
    };

    console.log('Store ready (using backend config via invoke)');
}
