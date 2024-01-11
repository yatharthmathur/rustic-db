use super::value_entry::ValueEntry;
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

    fn _remove(&mut self, key: &String) -> Option<ValueEntry> {
        self._data.remove(key)
    }

    fn _get_or_none_if_expired(&mut self, key: &String) -> Option<&ValueEntry> {
        let now = Instant::now();
        if let Some(value_entry) = self._data.get(key) {
            if now >= value_entry.expiration {
                return None;
            }
        };
        self._data.get(key)
    }

    fn _persist_db(&mut self) {
        /// open file
        /// encrypt
        /// write in binary
        /// close file
        todo!();
    }

    pub fn clear_all_expired_keys(&mut self) {
        let now = Instant::now();
        let expired_keys: Vec<String> = self
            ._data
            .iter()
            .filter(|(_, value_entry)| now >= value_entry.expiration)
            .map(|(key, _)| key.to_owned())
            .collect();

        for key in expired_keys {
            self.remove(key);
        }
    }

    pub fn contains(&mut self, key: String) -> bool {
        match self._get_or_none_if_expired(&key) {
            Some(_) => true,
            _ => false,
        }
    }

    /// Inserts a Key-Value(in Vec<u8> type) pair in the KeyValueStore
    pub fn set(&mut self, key: String, value: Vec<u8>, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::new(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Inserts a Key-Value(in String type) pair in the KeyValueStore
    /// Note: it will always be stored as Vec<u8> internally.
    pub fn set_with_string_value(&mut self, key: String, value: String, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_string(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (in Vec<u8> type) associated to the Key in the KeyValueStore
    pub fn get(&mut self, key: String) -> Option<&Vec<u8>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.value.as_ref()),
            _ => None,
        }
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    pub fn get_as_string(&mut self, key: String) -> Option<String> {
        if let Some(value_entry) = self._get_or_none_if_expired(&key) {
            ValueEntry::extract_string_value_from_value_entry(value_entry)
        } else {
            None
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    pub fn remove(&mut self, key: String) {
        self._remove(&key);
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (in Vec<u8> type)
    pub fn pop(&mut self, key: String) -> Option<Vec<u8>> {
        match self._remove(&key) {
            Some(value_entry) => Some(value_entry.value),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to String type)
    pub fn pop_as_string(&mut self, key: String) -> Option<String> {
        if let Some(value_entry) = self._remove(&key) {
            ValueEntry::extract_string_value_from_value_entry(&value_entry)
        } else {
            None
        }
    }

    /// Clear all Key-Value pairs from the KeyValueStore
    pub fn clear(&mut self) {
        self._data.clear();
    }
}

mod private_tests {
    use super::KeyValueStore;
    use std::time::Duration;

    #[test]
    fn test_clear_all_expired_keys() {
        let mut store = KeyValueStore::new(5000);
        store.set_with_string_value("ABC".to_string(), "HELLO".to_string(), Some(250));
        store.set_with_string_value("XYZ".to_string(), "HELLO".to_string(), Some(250));
        store.set_with_string_value("DEF".to_string(), "HELLO".to_string(), Some(250));

        store.clear_all_expired_keys();

        assert!(store._data.contains_key("ABC"));

        std::thread::sleep(Duration::from_millis(250));
        assert!(store._data.contains_key("ABC"));
        assert!(store._data.contains_key("DEF"));
        assert!(store._data.contains_key("XYZ"));

        store.clear_all_expired_keys();
        assert_eq!(store._data.contains_key("ABC"), false);
        assert_eq!(store._data.contains_key("DEF"), false);
        assert_eq!(store._data.contains_key("XYZ"), false);
    }
}
