use crate::stores::{errors::ValueError, store::KeyValueStore, value_entry::ValueEntry};
use std::collections::HashMap;

impl KeyValueStore {
    /// Inserts a Key-Value(in HashMap<(String, String)> type) pair in the KeyValueStore
    pub fn set_hmap(&mut self, key: String, value: Vec<(String, String)>, ttl: Option<u64>) {
        let expiration = self._get_expiration_instant(ttl);
        let value_entry = ValueEntry::from_hashmap(HashMap::from_iter(value), expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (converted to set<String> type) associated to the Key in the KeyValueStore
    pub fn get_hmap(&self, key: String) -> Option<Result<HashMap<String, String>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_hmap()),
            _ => None,
        }
    }

    fn _get_mut_hmap(
        &mut self,
        key: String,
    ) -> Option<Result<&mut HashMap<String, String>, ValueError>> {
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_mut_hmap()),
            None => None,
        }
    }

    /// Set a key-value pair in the hash map associated with a key
    /// Then return the size of the hmap.
    pub fn hmap_insert(
        &mut self,
        key: String,
        kv_pair: (String, String),
    ) -> Option<Result<usize, ValueError>> {
        match self._get_mut_hmap(key) {
            Some(Ok(hmap)) => {
                hmap.insert(kv_pair.0, kv_pair.1);
                Some(Ok(hmap.len()))
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Gets a value for a provided hmap_key from the hash map associated with the storage key
    pub fn hmap_get(&self, key: String, hmap_key: String) -> Option<Result<String, ValueError>> {
        match self.get_hmap(key) {
            Some(Ok(hmap)) => {
                if let Some(value) = hmap.get(&hmap_key) {
                    Some(Ok(value.to_owned()))
                } else {
                    None
                }
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Removes a key-value pair for a provided hmap_key from the hash map associated with the storage key
    /// also returns the pair
    pub fn hmap_remove(
        &mut self,
        key: String,
        hmap_key: String,
    ) -> Option<Result<(String, String), ValueError>> {
        match self._get_mut_hmap(key) {
            Some(Ok(hmap)) => {
                if let Some(value) = hmap.remove_entry(&hmap_key) {
                    Some(Ok(value))
                } else {
                    None
                }
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Checks if a hmap_key is present in the hmap associated with a key
    pub fn hmap_contains_key(
        &self,
        key: String,
        hmap_key: String,
    ) -> Option<Result<bool, ValueError>> {
        match self.get_hmap(key) {
            Some(Ok(hmap)) => Some(Ok(hmap.contains_key(&hmap_key))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Gets the size of the hmap associated with a key
    pub fn hmap_size(&self, key: String) -> Option<Result<usize, ValueError>> {
        match self.get_hmap(key) {
            Some(Ok(hmap)) => Some(Ok(hmap.len())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    // Gets all the hmap_keys of the hmap associated with a key
    pub fn hmap_keys(&self, key: String) -> Option<Result<Vec<String>, ValueError>> {
        match self.get_hmap(key) {
            Some(Ok(hmap)) => Some(Ok(Vec::from_iter(hmap.keys().map(|item| item.to_owned())))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    // Gets all the hmap_values of the hmap associated with a key
    pub fn hmap_values(&self, key: String) -> Option<Result<Vec<String>, ValueError>> {
        match self.get_hmap(key) {
            Some(Ok(hmap)) => Some(Ok(Vec::from_iter(
                hmap.values().map(|item| item.to_owned()),
            ))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    // Gets all the (hmap_keys, hmap_values) pairs of the hmap associated with a key
    pub fn hmap_items(&self, key: String) -> Option<Result<Vec<(String, String)>, ValueError>> {
        match self.get_hmap(key) {
            Some(Ok(hmap)) => Some(Ok(Vec::from_iter(
                hmap.iter().map(|(k, v)| (k.to_owned(), v.to_owned())),
            ))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}
