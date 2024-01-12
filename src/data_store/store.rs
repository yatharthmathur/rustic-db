use super::value_entry::{TypeConversionError, ValueEntry, ValueError, ValueType};
use std::collections::HashMap;
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
                return None;
            }
        };
        self._data.get(key)
    }

    fn _get_mut_or_none_if_expired(&mut self, key: &String) -> Option<&mut ValueEntry> {
        if let Some(value_entry) = self._data.get_mut(key) {
            if value_entry.is_expired_entry(None) {
                return None;
            }
        };
        self._data.get_mut(key)
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
    /// NOTE: this returns true, even if the key is expired.
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

    /// Clear all Key-Value pairs from the KeyValueStore
    pub fn clear(&mut self) {
        self._data.clear();
    }

    fn _add(&mut self, key: String, value: i64) -> Option<Result<i64, ValueError>> {
        if let Some(value_entry) = self._data.get_mut(&key) {
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
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_mut_deque() {
                Ok(list) => {
                    list.push_back(value.to_owned());
                    Some(Ok(value))
                }
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    /// Append to the front of a list
    pub fn list_pushf(&mut self, key: String, value: String) -> Option<Result<String, ValueError>> {
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_mut_deque() {
                Ok(list) => {
                    list.push_front(value.to_owned());
                    Some(Ok(value))
                }
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    /// pop from the front of a list
    pub fn list_popf(&mut self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_mut_deque() {
                Ok(list) => {
                    let opt_value = list.pop_front();
                    if let Some(value) = opt_value {
                        Some(Ok(value))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    /// pop from the back of a list
    pub fn list_popb(&mut self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_mut_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_mut_deque() {
                Ok(list) => {
                    let opt_value = list.pop_back();
                    if let Some(value) = opt_value {
                        Some(Ok(value))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    /// get front of the list
    pub fn list_front(&self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_deque() {
                Ok(list) => {
                    let opt_value = list.front();
                    if let Some(value) = opt_value {
                        Some(Ok(value.to_owned()))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    /// get back of the list
    pub fn list_back(&self, key: String) -> Option<Result<String, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => match value_entry.get_value_as_deque() {
                Ok(list) => {
                    let opt_value = list.back();
                    if let Some(value) = opt_value {
                        Some(Ok(value.to_owned()))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    /// size of the list
    pub fn list_size(&self, key: String) -> Option<Result<usize, ValueError>> {
        match self._data.get(&key) {
            Some(value_entry) => match value_entry.get_value_as_deque() {
                Ok(list) => Some(Ok(list.len())),
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }
}
