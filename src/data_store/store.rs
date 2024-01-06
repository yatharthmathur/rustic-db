use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Each entry of the Key-Value pair in the Data store is this struct.
struct KeyValueEntry {
    /// Internally all values are stored as a Vector of Bytes
    value: Vec<u8>,

    /// Expiration datetime of the given key is stored here.
    expiration: Instant,
    // todo: Add more fields depending on type maybe.
}

impl KeyValueEntry {
    pub fn new(value: Vec<u8>, expiration: Instant) -> Self {
        KeyValueEntry { value, expiration }
    }

    pub fn from_string(value_string: String, expiration: Instant) -> Self {
        let value = value_string.as_bytes().to_vec();
        KeyValueEntry { value, expiration }
    }
}

/// The main struct of the Key-Value store
pub struct KeyValueStore {
    /// The data is internally stored as a HashMap mapping String keys to a KeyValueEntry struct
    _data: HashMap<String, KeyValueEntry>,

    /// The default time to live for each key is set here (globally).
    default_ttl: Duration,
}

impl KeyValueStore {
    /// Returns a new KeyValue store
    /// Arguments:
    /// * `default_ttl` - duration in milliseconds for  which every key by default lives in the cache.
    pub fn new(default_ttl_millis: u64) -> Self {
        KeyValueStore {
            _data: HashMap::new(),
            default_ttl: Duration::from_millis(default_ttl_millis),
        }
    }

    fn _insert(&mut self, key: String, kv_entry: KeyValueEntry) {
        self._data.insert(key.clone(), kv_entry);
    }

    fn _remove_if_expired(&mut self, key: &String) {
        let now = Instant::now();
        if let Some(kv_entry) = self._data.get(key) {
            if now >= kv_entry.expiration {
                self._data.remove(key);
            }
        }
    }

    fn _get(&mut self, key: &String) -> Option<&KeyValueEntry> {
        self._remove_if_expired(key);
        self._get(key)
    }

    /// Inserts a Key-Value(in Vec<u8> type) pair in the KeyValueStore
    pub fn set(&mut self, key: String, value: Vec<u8>, ttl: Option<Duration>) {
        let expiration = Instant::now() + ttl.unwrap_or(self.default_ttl);
        let kv_entry = KeyValueEntry::new(value, expiration);
        self._insert(key, kv_entry);
    }

    /// Inserts a Key-Value(in String type) pair in the KeyValueStore
    /// Note: it will always be stored as Vec<u8> internally.
    pub fn set_with_string_value(&mut self, key: String, value: String, ttl: Option<Duration>) {
        let expiration = Instant::now() + ttl.unwrap_or(self.default_ttl);
        let kv_entry = KeyValueEntry::from_string(value, expiration);
        self._insert(key, kv_entry);
    }

    /// Gets a Value (in Vec<u8> type) associated to the Key in the KeyValueStore
    pub fn get(&mut self, key: &String) -> Option<&Vec<u8>> {
        match self._get(key) {
            Some(kv_entry) => Some(&kv_entry.value),
            _ => None,
        }
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    pub fn get_as_string(&mut self, key: &String) -> Option<String> {
        let kv_entry_option = self._get(key);
        if kv_entry_option.is_none() {
            return None;
        }

        let kv_entry = kv_entry_option.unwrap();
        match String::from_utf8(kv_entry.value.clone()) {
            Ok(string) => Some(string),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    pub fn remove(&mut self, key: String) {
        todo!();
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (in Vec<u8> type)
    pub fn pop(&mut self, key: String) -> Vec<u8> {
        todo!();
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to String type)
    pub fn pop_as_string(&mut self, key: String) -> Vec<u8> {
        todo!();
    }

    /// Clear all Key-Value pairs from the KeyValueStore
    pub fn clear(&mut self) {
        todo!();
    }
}
