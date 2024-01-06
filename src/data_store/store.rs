use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Each entry of the Key-Value pair in the Data store is this struct.
#[derive(Clone)]
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

    /// Extract value and convert it to String from KeyValueEntry
    fn extract_string_value_from_kv_entry(
        kv_entry_option: Option<KeyValueEntry>,
    ) -> Option<String> {
        if kv_entry_option.is_none() {
            return None;
        }

        // kv_entry_option is definitely Some.
        let kv_entry = kv_entry_option.unwrap();
        match String::from_utf8(kv_entry.value) {
            Ok(string) => Some(string),
            // This case will not happen as all values that
            // were initially stored in the DB were valid.
            _ => None,
        }
    }
}

/// The main struct of the Key-Value store
pub struct KeyValueStore {
    /// The data is internally stored as a HashMap mapping String keys to a KeyValueEntry struct
    _data: HashMap<String, KeyValueEntry>,

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

    fn _insert(&mut self, key: String, kv_entry: KeyValueEntry) {
        self._data.insert(key.clone(), kv_entry);
    }

    fn _remove(&mut self, key: String) -> Option<KeyValueEntry> {
        self._data.remove(&key)
    }

    fn _get_or_remove_if_expired(&mut self, key: String) -> Option<KeyValueEntry> {
        let now = Instant::now();
        if let Some(kv_entry) = self._data.get(&key) {
            if now >= kv_entry.expiration {
                self._data.remove(&key);
            }
        };
        self._data.get(&key).cloned()
    }

    pub fn contains(&mut self, key: String) -> bool {
        match self._get_or_remove_if_expired(key) {
            Some(_) => true,
            _ => false,
        }
    }

    /// Inserts a Key-Value(in Vec<u8> type) pair in the KeyValueStore
    pub fn set(&mut self, key: String, value: Vec<u8>, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let kv_entry = KeyValueEntry::new(value, expiration);
        self._insert(key, kv_entry);
    }

    /// Inserts a Key-Value(in String type) pair in the KeyValueStore
    /// Note: it will always be stored as Vec<u8> internally.
    pub fn set_with_string_value(&mut self, key: String, value: String, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let kv_entry = KeyValueEntry::from_string(value, expiration);
        self._insert(key, kv_entry);
    }

    /// Gets a Value (in Vec<u8> type) associated to the Key in the KeyValueStore
    pub fn get(&mut self, key: String) -> Option<Vec<u8>> {
        match self._get_or_remove_if_expired(key) {
            Some(kv_entry) => Some(kv_entry.value),
            _ => None,
        }
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    pub fn get_as_string(&mut self, key: String) -> Option<String> {
        let kv_entry_option = self._get_or_remove_if_expired(key);
        KeyValueEntry::extract_string_value_from_kv_entry(kv_entry_option)
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    pub fn remove(&mut self, key: String) {
        self._remove(key);
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (in Vec<u8> type)
    pub fn pop(&mut self, key: String) -> Option<Vec<u8>> {
        match self._remove(key) {
            Some(kv_entry) => Some(kv_entry.value),
            _ => None,
        }
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to String type)
    pub fn pop_as_string(&mut self, key: String) -> Option<String> {
        let kv_entry_option = self._remove(key);
        KeyValueEntry::extract_string_value_from_kv_entry(kv_entry_option)
    }

    /// Clear all Key-Value pairs from the KeyValueStore
    pub fn clear(&mut self) {
        self._data.clear();
    }
}
