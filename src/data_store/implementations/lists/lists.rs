use crate::data_store::{errors::ValueError, store::KeyValueStore, value_entry::ValueEntry};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

impl KeyValueStore {
    /// Inserts a Key-Value(in Vec<String> type) pair in the KeyValueStore
    pub fn set_list(&mut self, key: String, value: Vec<String>, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_list(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (converted to Vec<String> type) associated to the Key in the KeyValueStore
    pub fn get_list(&self, key: String) -> Option<Result<Vec<String>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_list()),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to Vec<String> type)
    pub fn pop_list(&mut self, key: String) -> Option<Result<Vec<String>, ValueError>> {
        match self._remove_and_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_list()),
            _ => None,
        }
    }

    fn _get_deque(&self, key: String) -> Option<Result<&VecDeque<String>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_deque() {
                Ok(deque) => Some(Ok(deque)),
                Err(e) => Some(Err(e)),
            },
            None => None,
        }
    }

    fn _get_mut_deque(&mut self, key: String) -> Option<Result<&mut VecDeque<String>, ValueError>> {
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_mut_deque() {
                Ok(deque) => Some(Ok(deque)),
                Err(e) => Some(Err(e)),
            },
            None => None,
        }
    }

    /// Append to the back of a list
    pub fn list_pushb(&mut self, key: String, value: String) -> Option<Result<String, ValueError>> {
        match self._get_mut_deque(key) {
            Some(Ok(deque)) => {
                deque.push_back(value.to_owned());
                Some(Ok(value))
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Append to the front of a list
    pub fn list_pushf(&mut self, key: String, value: String) -> Option<Result<String, ValueError>> {
        match self._get_mut_deque(key) {
            Some(Ok(deque)) => {
                deque.push_front(value.to_owned());
                Some(Ok(value))
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// pop from the front of a list
    pub fn list_popf(&mut self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_mut_deque(key) {
            Some(Ok(deque)) => {
                let opt_value = deque.pop_front();
                if let Some(value) = opt_value {
                    Some(Ok(value))
                } else {
                    None
                }
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// pop from the back of a list
    pub fn list_popb(&mut self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_mut_deque(key) {
            Some(Ok(deque)) => {
                let opt_value = deque.pop_back();
                if let Some(value) = opt_value {
                    Some(Ok(value))
                } else {
                    None
                }
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// get front of the list
    pub fn list_front(&self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_deque(key) {
            Some(Ok(deque)) => {
                let opt_value = deque.front();
                if let Some(value) = opt_value {
                    Some(Ok(value.to_owned()))
                } else {
                    None
                }
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// get back of the list
    pub fn list_back(&self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_deque(key) {
            Some(Ok(deque)) => {
                let opt_value = deque.back();
                if let Some(value) = opt_value {
                    Some(Ok(value.to_owned()))
                } else {
                    None
                }
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
    /// size of the list
    pub fn list_size(&self, key: String) -> Option<Result<usize, ValueError>> {
        match self._get_deque(key) {
            Some(Ok(deque)) => Some(Ok(deque.len())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}
