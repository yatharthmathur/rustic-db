use std::collections::HashMap;
use std::time::Instant;

use super::value_entry::ValueEntry;

/// The main struct of the Key-Value store
pub struct KeyValueStore {
    /// The data is internally stored as a HashMap mapping String keys to a KeyValueEntry struct
    pub(super) _data: HashMap<String, ValueEntry>,

    /// The default time to live for each key is set here (globally).
    pub(super) default_ttl: u64,
}

impl KeyValueStore {
    /// Returns a new KeyValue store
    /// Arguments:
    /// * `default_ttl` - duration in milliseconds for  which every key by default lives in the cache.
    pub fn new(default_ttl_millis: u64) -> Self {
        KeyValueStore {
            _data: HashMap::new(),
            default_ttl: default_ttl_millis,
        }
    }

    pub(super) fn _insert(&mut self, key: &String, value_entry: &ValueEntry) {
        self._data.insert(key.to_owned(), value_entry.to_owned());
    }

    pub(super) fn _remove_and_none_if_expired(&mut self, key: &String) -> Option<ValueEntry> {
        if let Some(value_entry) = self._data.remove(key) {
            if value_entry.is_expired_entry(None) {
                None
            } else {
                Some(value_entry)
            }
        } else {
            None
        }
    }

    pub(super) fn _get_or_none_if_expired(&self, key: &String) -> Option<&ValueEntry> {
        if let Some(value_entry) = self._data.get(key) {
            if value_entry.is_expired_entry(None) {
                None
            } else {
                Some(value_entry)
            }
        } else {
            None
        }
    }

    pub(super) fn _get_mut_or_none_if_expired(&mut self, key: &String) -> Option<&mut ValueEntry> {
        if let Some(value_entry) = self._data.get_mut(key) {
            if value_entry.is_expired_entry(None) {
                None
            } else {
                Some(value_entry)
            }
        } else {
            None
        }
    }

    pub fn clear_all_expired_keys(&mut self) {
        let now = Instant::now();
        let expired_keys: Vec<String> = self
            ._data
            .iter()
            .filter(|(_, value_entry)| value_entry.is_expired_entry(Some(now)))
            .map(|(key, _)| key.to_owned())
            .collect();

        for key in expired_keys {
            self.remove(key);
        }
    }

    /// Check whether the key exists in the store.
    /// NOTE: this may return true, even if the key is expired.
    pub fn contains_key(&self, key: String) -> bool {
        self._data.contains_key(&key)
    }

    /// Check whether a key is expired or not.
    pub fn is_expired(&self, key: String) -> Option<bool> {
        if let Some(value_entry) = self._data.get(&key) {
            Some(value_entry.is_expired_entry(None))
        } else {
            None
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    pub fn remove(&mut self, key: String) {
        self._remove_and_none_if_expired(&key);
    }

    /// Clear all Key-Value pairs from the KeyValueStore
    pub fn clear(&mut self) {
        self._data.clear();
    }
}
