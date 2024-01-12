use crate::data_store::errors::ValueError;

use super::super::{store::KeyValueStore, value_entry::ValueEntry};
use std::time::{Duration, Instant};

impl KeyValueStore {
    /// Inserts a Key-Value(in Vec<u8> type) pair in the KeyValueStore
    pub fn set_bytes(&mut self, key: String, value: Vec<u8>, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_bytes(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (in Vec<u8> type) associated to the Key in the KeyValueStore
    pub fn get_bytes(&self, key: String) -> Option<Result<Vec<u8>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_bytes()),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (in Vec<u8> type)
    pub fn pop_bytes(&mut self, key: String) -> Option<Result<Vec<u8>, ValueError>> {
        match self._remove_and_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_bytes()),
            _ => None,
        }
    }
}
