use std::time::Instant;

/// Each entry of the Key-Value pair in the Data store is this struct.
#[derive(Clone)]
pub struct ValueEntry {
    /// Internally all values are stored as a Vector of Bytes
    pub value: Vec<u8>,

    /// Expiration datetime of the given key is stored here.
    pub expiration: Instant,
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
    pub fn extract_string_value_from_value_entry(value_entry: &ValueEntry) -> Option<String> {
        match String::from_utf8(value_entry.value.to_owned()) {
            Ok(string) => Some(string),
            // This case will not happen as all values that
            // were initially stored in the DB were valid.
            _ => None,
        }
    }
}
