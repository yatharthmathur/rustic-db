use crate::stores::store::KeyValueStore;
use log::info;
use std::collections::HashMap;

pub struct RusticManager {
    _stores_map: HashMap<String, KeyValueStore>,
}

impl RusticManager {
    pub fn new() -> RusticManager {
        return RusticManager {
            _stores_map: HashMap::new(),
        };
    }

    /// Creates a data store and returns a mutable reference to it.
    pub fn create_store(&mut self, name: String, default_ttl: Option<u64>) -> &mut KeyValueStore {
        if self._stores_map.contains_key(&name) {
            panic!("Store with this name already exists.");
        }
        let new_kv_store: KeyValueStore = KeyValueStore::new(name.to_owned(), default_ttl);
        self._stores_map.insert(name.to_owned(), new_kv_store);
        self._stores_map.get_mut(&name).unwrap()
    }

    /// Removes the data store and returns True or False based on if it existed.
    pub fn remove_store(&mut self, name: String) -> bool {
        if self._stores_map.contains_key(&name) {
            self._stores_map.remove(&name);
            true
        } else {
            false
        }
    }

    /// Get optional mutable reference to data store
    pub fn get_store_mut(&mut self, name: String) -> Option<&mut KeyValueStore> {
        self._stores_map.get_mut(&name)
    }

    /// Get optional immutable reference to data store
    pub fn get_store(&self, name: String) -> Option<&KeyValueStore> {
        self._stores_map.get(&name)
    }

    /// List names of all the stores
    pub fn list_store_names(&self) -> Vec<&String> {
        self._stores_map.keys().collect()
    }

    /// Clear expired keys periodically
    pub fn clear_expired_keys_in_all_stores(&mut self) {
        // Remove expired keys
        for (_, store) in self._stores_map.iter_mut() {
            info!(
                "Expiring the key for KeyValueStore({:?}).",
                store.get_name()
            );
            store.clear_all_expired_keys();
        }
    }
}
