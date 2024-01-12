use crate::data_store::{
    errors::{TypeConversionError, ValueError},
    store::KeyValueStore,
    types::ValueType,
    value_entry::ValueEntry,
};
use std::time::{Duration, Instant};

impl KeyValueStore {
    /// Inserts a Key-Value(in i64 type) pair in the KeyValueStore
    pub fn set_i64(&mut self, key: String, value: i64, ttl: Option<u64>) {
        let expiration = Instant::now() + Duration::from_millis(ttl.unwrap_or(self.default_ttl));
        let value_entry = ValueEntry::from_i64(value, expiration);
        self._insert(&key, &value_entry);
    }

    /// Gets a Value (converted to String type) associated to the Key in the KeyValueStore
    pub fn get_i64(&self, key: String) -> Option<Result<i64, ValueError>> {
        match self._get_or_none_if_expired(&key) {
            Some(value_entry) => Some(value_entry.get_value_as_i64()),
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
}
