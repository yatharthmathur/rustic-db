use std::{
    collections::{vec_deque::VecDeque, HashMap, HashSet},
    time::Instant,
};

use super::{
    errors::{TypeConversionError, ValueError},
    types::ValueType,
};

/// Each entry of the Key-Value pair in the Data store is this struct.
#[derive(Clone)]
pub struct ValueEntry {
    /// Internally all values are stored as a Vector of Bytes
    pub value: ValueType,

    /// Expiration datetime of the given key is stored here.
    pub expiration: Instant,
    // todo: Add more fields depending on type maybe.
}

impl ValueEntry {
    pub fn from_i64(value: i64, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::Integer64(value),
            expiration,
        }
    }

    pub fn from_bytes(value: Vec<u8>, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::Bytes(value),
            expiration,
        }
    }

    pub fn from_string(value: String, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::String(value),
            expiration,
        }
    }

    pub fn from_list(value: Vec<String>, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::Deque(VecDeque::from(value)),
            expiration,
        }
    }

    pub fn from_deque(value: VecDeque<String>, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::Deque(value),
            expiration,
        }
    }

    pub fn from_hset(value: HashSet<String>, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::Set(value),
            expiration,
        }
    }

    pub fn from_hashmap(value: HashMap<String, String>, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::HashMap(value),
            expiration,
        }
    }

    pub fn get_value_as_i64(&self) -> Result<i64, ValueError> {
        match &self.value {
            ValueType::Integer64(integer) => Ok(integer.to_owned()),
            ValueType::String(string) => match string.parse::<i64>() {
                Ok(integer) => Ok(integer),
                Err(e) => Err(ValueError::TypeConversionError(
                    TypeConversionError::ParseIntError(e),
                )),
            },
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_bytes(&self) -> Result<Vec<u8>, ValueError> {
        match &self.value {
            ValueType::Bytes(bytes) => Ok(bytes.to_owned()),
            ValueType::String(string) => Ok(string.to_owned().into_bytes()),
            ValueType::Integer64(integer) => Ok(integer.to_le_bytes().to_vec()),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_string(&self) -> Result<String, ValueError> {
        match &self.value {
            ValueType::String(string) => Ok(string.to_owned()),
            ValueType::Bytes(bytes) => match String::from_utf8(bytes.to_owned()) {
                Err(e) => Err(ValueError::TypeConversionError(
                    TypeConversionError::FromUtf8Error(e),
                )),
                Ok(string) => Ok(string),
            },
            ValueType::Integer64(integer) => Ok(integer.to_string()),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_list(&self) -> Result<Vec<String>, ValueError> {
        match &self.value {
            ValueType::Deque(list) => Ok(Vec::from(list.to_owned())),
            ValueType::String(string) => Ok(string.chars().map(|ch| ch.to_string()).collect()),
            ValueType::Bytes(bytes) => Ok(bytes.iter().map(|byte| byte.to_string()).collect()),
            ValueType::Set(hash_set) => Ok(hash_set.iter().map(|val| val.to_string()).collect()),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_deque(&self) -> Result<&VecDeque<String>, ValueError> {
        match &self.value {
            ValueType::Deque(list) => Ok(list),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_mut_deque(&mut self) -> Result<&mut VecDeque<String>, ValueError> {
        match &mut self.value {
            ValueType::Deque(list) => Ok(list),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_hset(&self) -> Result<HashSet<String>, ValueError> {
        match &self.value {
            ValueType::Set(hash_set) => Ok(hash_set.to_owned()),
            ValueType::String(string) => Ok(HashSet::from_iter(
                string.chars().map(|item| item.to_string()),
            )),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_mut_hset(&mut self) -> Result<&mut HashSet<String>, ValueError> {
        match &mut self.value {
            ValueType::Set(hash_set) => Ok(hash_set),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_hmap(&self) -> Result<HashMap<String, String>, ValueError> {
        match &self.value {
            ValueType::HashMap(hmap) => Ok(hmap.to_owned()),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    pub fn get_value_as_mut_hmap(&mut self) -> Result<&mut HashMap<String, String>, ValueError> {
        match &mut self.value {
            ValueType::HashMap(hmap) => Ok(hmap),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    /// Check if this entry is expired.
    pub fn is_expired_entry(&self, option_now: Option<Instant>) -> bool {
        let now = option_now.unwrap_or(Instant::now());
        now >= self.expiration
    }
}
