use super::value_entry::{TypeConversionError, ValueEntry, ValueError, ValueType};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

/// The main struct of the Key-Value store
pub struct KeyValueStore {
    /// The data is internally stored as a HashMap mapping String keys to a KeyValueEntry struct
    _data: HashMap<String, ValueEntry>,

    /// The default time to live for each key is set here (globally).
    default_ttl: u64,
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

    fn _insert(&mut self, key: &String, value_entry: &ValueEntry) {
        self._data.insert(key.to_owned(), value_entry.to_owned());
    }

    fn _remove_and_none_if_expired(&mut self, key: &String) -> Option<ValueEntry> {
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

    fn _get_or_none_if_expired(&self, key: &String) -> Option<&ValueEntry> {
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

    fn _get_mut_or_none_if_expired(&mut self, key: &String) -> Option<&mut ValueEntry> {
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

    /// Inserts a Key-Value(in Vec<u8> type) pair in the KeyValueStore
    pub fn set_bytes(&mut self, key: String, value: Vec<u8>, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_bytes(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Inserts a Key-Value(in String type) pair in the KeyValueStore
    pub fn set_string(&mut self, key: String, value: String, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_string(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Inserts a Key-Value(in i64 type) pair in the KeyValueStore
    pub fn set_i64(&mut self, key: String, value: i64, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_i64(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Inserts a Key-Value(in Vec<String> type) pair in the KeyValueStore
    pub fn set_list(&mut self, key: String, value: Vec<String>, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_list(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Inserts a Key-Value(in HashSet<String> type) pair in the KeyValueStore
    pub fn set_hset(&mut self, key: String, value: Vec<String>, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_set(HashSet::from_iter(value), expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (in Vec<u8> type) associated to the Key in the KeyValueStore
    pub fn get_bytes(&self, key: String) -> Option<Result<Vec<u8>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_bytes()),
            _ => None,
        }
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    pub fn get_string(&self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_string()),
            _ => None,
        }
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    pub fn get_i64(&self, key: String) -> Option<Result<i64, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_i64()),
            _ => None,
        }
    }

    /// Gets a Value (converted to Vec<String> type) associated to the Key in the KeyValueStore
    pub fn get_list(&self, key: String) -> Option<Result<Vec<String>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
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

    /// Gets a Value (converted to set<String> type) associated to the Key in the KeyValueStore
    pub fn get_hset(&self, key: String) -> Option<Result<&HashSet<String>, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_hset()),
            _ => None,
        }
    }

    fn _get_mut_hset(&mut self, key: String) -> Option<Result<&mut HashSet<String>, ValueError>> {
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_mut_hset() {
                Ok(hset) => Some(Ok(hset)),
                Err(e) => Some(Err(e)),
            },
            None => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    pub fn remove(&mut self, key: String) {
        self._remove_and_none_if_expired(&key);
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (in Vec<u8> type)
    pub fn pop_bytes(&mut self, key: String) -> Option<Result<Vec<u8>, ValueError>> {
        match self._remove_and_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_bytes()),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to String type)
    pub fn pop_string(&mut self, key: String) -> Option<Result<String, ValueError>> {
        match self._remove_and_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_string()),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to i64 type)
    pub fn pop_i64(&mut self, key: String) -> Option<Result<i64, ValueError>> {
        match self._remove_and_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_i64()),
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

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to Vec<String> type)
    pub fn pop_set(&mut self, key: String) -> Option<Result<Vec<String>, ValueError>> {
        self.pop_list(key)
    }

    /// Clear all Key-Value pairs from the KeyValueStore
    pub fn clear(&mut self) {
        self._data.clear();
    }

    fn _add(&mut self, key: String, value: i64) -> Option<Result<i64, ValueError>> {
        if let Some(value_entry) = self._get_mut_or_none_if_expired(&key) {
            match value_entry.get_value_as_i64() {
                Ok(old_value) => {
                    let updated_integer_value = old_value + value;
                    value_entry.value = ValueType::Integer64(updated_integer_value);
                    Some(Ok(updated_integer_value))
                }
                Err(e) => Some(Err(e)),
            }
        } else {
            None
        }
    }

    /// decrement an existing value associated to key by a certain number.
    pub fn decr(&mut self, key: String, by: Option<u64>) -> Option<Result<i64, ValueError>> {
        match i64::try_from(by.unwrap_or(1)) {
            Ok(value) => self._add(key, -value),
            Err(e) => Some(Err(ValueError::TypeConversionError(
                TypeConversionError::TryFromIntError(e),
            ))),
        }
    }

    /// increment an existing value associated to a key by a certain number.
    pub fn incr(&mut self, key: String, by: Option<u64>) -> Option<Result<i64, ValueError>> {
        match i64::try_from(by.unwrap_or(1)) {
            Ok(value) => self._add(key, value),
            Err(e) => Some(Err(ValueError::TypeConversionError(
                TypeConversionError::TryFromIntError(e),
            ))),
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

    /// Adds an item to the set and returns the cardinality of the set.
    pub fn hset_add(&mut self, key: String, value: String) -> Option<Result<usize, ValueError>> {
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
    pub fn hset_remove(&mut self, key: String, value: String) -> Option<Result<usize, ValueError>> {
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
    pub fn hset_contains(&self, key: String, value: String) -> Option<Result<bool, ValueError>> {
        match self.get_hset(key) {
            Some(Ok(hset)) => Some(Ok(hset.contains(&value))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// Get the intersection between two sets in the data store
    pub fn hset_intersection(
        &self,
        key1: String,
        key2: String,
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
    pub fn hset_union(
        &self,
        key1: String,
        key2: String,
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
            h1.union(&h2).map(|item| item.to_owned()),
        )))
    }

    /// Get the difference between two sets in the data store
    pub fn hset_difference(
        &self,
        key1: String,
        key2: String,
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

    /// size of the list
    pub fn list_size(&self, key: String) -> Option<Result<usize, ValueError>> {
        match self._get_deque(key) {
            Some(Ok(deque)) => Some(Ok(deque.len())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }

    /// cardinality of the set
    pub fn hset_size(&self, key: String) -> Option<Result<usize, ValueError>> {
        match self.get_hset(key) {
            Some(Ok(hset)) => Some(Ok(hset.len())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}
