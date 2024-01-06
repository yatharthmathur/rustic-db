#![crate_name = "doc"]
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

/// The main struct of the Key-Value store
struct KeyValueStore {
    /// The data is internally stored as a HashMap mapping String keys to a KeyValueEntry struct
    data: HashMap<String, KeyValueEntry>,

    /// The default time to live for each key is set here (globally).
    default_ttl: Duration,
}

impl KeyValueStore {
    /// Returns a new KeyValue store
    /// Arguments:
    /// * `default_ttl` - `Duration` which is the duration for which every key by default lives in the cache.
    fn new(default_ttl: Duration) -> Self {
        KeyValueStore {
            data: HashMap::new(),
            default_ttl,
        }
    }

    /// Inserts a Key-Value(in Vec<u8> type) pair in the KeyValueStore
    fn set(&mut self, key: String, value: Vec<u8>, timeout: Option<Duration>) {
        todo!();
    }

    /// Inserts a Key-Value(in String type) pair in the KeyValueStore
    /// Note: it will always be stored as Vec<u8> internally.
    fn set_with_string_value(&mut self, key: String, value: String, timeout: Option<Duration>) {
        todo!();
    }

    /// Gets a Value (in Vec<u8> type) associated to the Key in the KeyValueStore
    fn get(&mut self, key: String) -> Vec<u8> {
        todo!();
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    fn get_as_string(&mut self, key: String) -> String {
        todo!();
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    fn remove(&mut self, key: String) {
        todo!();
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (in Vec<u8> type)
    fn pop(&mut self, key: String) -> Vec<u8> {
        todo!();
    }

    /// Removes the Key-Value pair for the given Key in the KeyValueStore
    /// and returns the Value (converted to String type)
    fn pop_as_string(&mut self, key: String) -> Vec<u8> {
        todo!();
    }

    /// Clear all Key-Value pairs from the KeyValueStore
    fn clear(&mut self) {
        todo!();
    }
}
