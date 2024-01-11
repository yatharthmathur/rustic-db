use clokwerk::{AsyncScheduler, Job, TimeUnits};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Each entry of the Key-Value pair in the Data store is this struct.
#[derive(Clone)]
struct ValueEntry {
    /// Internally all values are stored as a Vector of Bytes
    value: Vec<u8>,

    /// Expiration datetime of the given key is stored here.
    expiration: Instant,
    // todo: Add more fields depending on type maybe.
}

impl ValueEntry {
    pub fn new(value: Vec<u8>, expiration: Instant) -> Self {
        ValueEntry { value, expiration }
    }

    pub fn from_string(value_string: String, expiration: Instant) -> Self {
        let value = value_string.as_bytes().to_vec();
        ValueEntry { value, expiration }
    }

    /// Extract value and convert it to String from KeyValueEntry
    fn extract_string_value_from_value_entry_ref(
        value_entry_option: Option<&ValueEntry>,
    ) -> Option<String> {
        if value_entry_option.is_none() {
            return None;
        }

        // value_entry_option is definitely Some.
        let value_entry = value_entry_option.unwrap();
        match String::from_utf8(value_entry.value.to_owned()) {
            Ok(string) => Some(string),
            // This case will not happen as all values that
            // were initially stored in the DB were valid.
            _ => None,
        }
    }

    fn extract_string_value_from_value_entry(
        value_entry_option: Option<ValueEntry>,
    ) -> Option<String> {
        if value_entry_option.is_none() {
            return None;
        }

        // value_entry_option is definitely Some.
        let value_entry = value_entry_option.unwrap();
        match String::from_utf8(value_entry.value) {
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

    async fn _clear_all_expired_keys(&mut self) {
        let now = Instant::now();
        let expired_keys: Vec<String> = self
            ._data
            .iter()
            .filter(|(_, value_entry)| now >= value_entry.expiration)
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            self._data.remove(&key);
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
        let value_entry_option = self._get_or_none_if_expired(&key);
        ValueEntry::extract_string_value_from_value_entry_ref(value_entry_option)
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
        let value_entry_option = self._remove(&key);
        ValueEntry::extract_string_value_from_value_entry(value_entry_option)
    }

    /// Clear all Key-Value pairs from the KeyValueStore
    pub fn clear(&mut self) {
        self._data.clear();
    }
}
