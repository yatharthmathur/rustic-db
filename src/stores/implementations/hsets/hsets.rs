use crate::stores::{errors::ValueError, store::KeyValueStore, value_entry::ValueEntry};
use std::collections::HashSet;

impl KeyValueStore {
    /// Inserts a Key-Value(in HashSet<String> type) pair in the KeyValueStore
    pub fn set_hset(&mut self, key: &str, value: Vec<String>, ttl: Option<u64>) {
        let expiration = self._get_expiration_instant(ttl);
        let value_entry = ValueEntry::from_hset(HashSet::from_iter(value), expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (converted to set<String> type) associated to the Key in the KeyValueStore
    pub fn get_hset(&self, key: &str) -> Option<Result<HashSet<String>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_hset()),
            _ => None,
        }
    }

    fn _get_mut_hset(&mut self, key: &str) -> Option<Result<&mut HashSet<String>, ValueError>> {
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_mut_hset()),
            None => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to Vec<String> type)
    pub fn pop_hset(&mut self, key: &str) -> Option<Result<HashSet<String>, ValueError>> {
        match self._remove_and_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_hset()),
            _ => None,
        }
    }

    /// Adds an item to the set and returns the cardinality of the set.
    pub fn hset_add(&mut self, key: &str, value: String) -> Option<Result<usize, ValueError>> {
        match self._get_mut_hset(key) {
            Some(Ok(hset)) => {
                hset.insert(value);
                Some(Ok(hset.len()))
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Removes an item from the set and returns the cardinality of the set.
    pub fn hset_remove(&mut self, key: &str, value: String) -> Option<Result<usize, ValueError>> {
        match self._get_mut_hset(key) {
            Some(Ok(hset)) => {
                hset.remove(&value);
                Some(Ok(hset.len()))
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Checks if the item exists in the given hset
    pub fn hset_contains(&self, key: &str, value: String) -> Option<Result<bool, ValueError>> {
        match self.get_hset(key) {
            Some(Ok(hset)) => Some(Ok(hset.contains(&value))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Get the intersection between two sets in the data store
    pub fn hset_intersection(
        &self,
        key1: &str,
        key2: &str,
    ) -> Option<Result<Vec<String>, ValueError>> {
        let opt_res_h1 = self.get_hset(key1);
        let opt_res_h2 = self.get_hset(key2);

        let h1 = match opt_res_h1 {
            Some(Ok(hset1)) => hset1.to_owned(),
            Some(Err(e)) => {
                return Some(Err(e));
            }
            None => HashSet::new(),
        };

        let h2 = match opt_res_h2 {
            Some(Ok(hset2)) => hset2.to_owned(),
            Some(Err(e)) => {
                return Some(Err(e));
            }
            None => HashSet::new(),
        };

        Some(Ok(Vec::from_iter(
            h1.intersection(&h2).map(|item| item.to_owned()),
        )))
    }

    /// Get the intersection between two sets in the data store
    pub fn hset_union(&self, key1: &str, key2: &str) -> Option<Result<Vec<String>, ValueError>> {
        let opt_res_h1 = self.get_hset(key1);
        let opt_res_h2 = self.get_hset(key2);

        let h1 = match opt_res_h1 {
            Some(Ok(hset1)) => hset1.to_owned(),
            Some(Err(e)) => {
                return Some(Err(e));
            }
            None => HashSet::new(),
        };

        let h2 = match opt_res_h2 {
            Some(Ok(hset2)) => hset2.to_owned(),
            Some(Err(e)) => {
                return Some(Err(e));
            }
            None => HashSet::new(),
        };

        Some(Ok(Vec::from_iter(
            h1.union(&h2).map(|item| item.to_owned()),
        )))
    }

    /// Get the difference between two sets in the data store
    pub fn hset_difference(
        &self,
        key1: &str,
        key2: &str,
    ) -> Option<Result<Vec<String>, ValueError>> {
        let opt_res_h1 = self.get_hset(key1);
        let opt_res_h2 = self.get_hset(key2);

        let h1 = match opt_res_h1 {
            Some(Ok(hset1)) => hset1.to_owned(),
            Some(Err(e)) => {
                return Some(Err(e));
            }
            None => HashSet::new(),
        };

        let h2 = match opt_res_h2 {
            Some(Ok(hset2)) => hset2.to_owned(),
            Some(Err(e)) => {
                return Some(Err(e));
            }
            None => HashSet::new(),
        };

        Some(Ok(Vec::from_iter(
            h1.difference(&h2).map(|item| item.to_owned()),
        )))
    }

    /// cardinality of the set
    pub fn hset_size(&self, key: &str) -> Option<Result<usize, ValueError>> {
        match self.get_hset(key) {
            Some(Ok(hset)) => Some(Ok(hset.len())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

pub mod hsets_tests {}
