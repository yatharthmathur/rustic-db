use crate::stores::{errors::ValueError, store::KeyValueStore, value_entry::ValueEntry};

impl KeyValueStore {
    /// Inserts a Key-Value(in String type) pair in the KeyValueStore
    pub fn set_string(&mut self, key: &str, value: String, ttl: Option<u64>) {
        let expiration = self._get_expiration_instant(ttl);
        let value_entry = ValueEntry::from_string(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    pub fn get_string(&self, key: &str) -> Option<Result<String, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_string()),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to String type)
    pub fn pop_string(&mut self, key: &str) -> Option<Result<String, ValueError>> {
        match self._remove_and_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_string()),
            _ => None,
        }
    }
}
