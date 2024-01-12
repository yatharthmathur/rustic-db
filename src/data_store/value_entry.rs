use std::{
    collections::{HashMap, HashSet},
    num::{ParseIntError, TryFromIntError},
    string::FromUtf8Error,
    time::Instant,
};

#[derive(Debug)]
pub enum TypeConversionError {
    ParseIntError(ParseIntError),
    FromUtf8Error(FromUtf8Error),
    TryFromIntError(TryFromIntError),
    // Add other type cast error variants as needed
}

#[derive(Debug)]
pub enum ValueError {
    TypeConversionImpossible,
    TypeConversionError(TypeConversionError),
}

#[derive(Clone)]
pub enum ValueType {
    Integer64(i64),
    Bytes(Vec<u8>),
    String(String),
    List(Vec<String>),
    HashSet(HashSet<String>),
    HashMap(HashMap<String, String>),
}

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
            value: ValueType::List(value),
            expiration,
        }
    }

    pub fn from_hashset(value: HashSet<String>, expiration: Instant) -> Self {
        ValueEntry {
            value: ValueType::HashSet(value),
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
                Ok(integer_value) => Ok(integer_value),
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
            ValueType::String(value_string) => Ok(value_string.to_owned()),
            ValueType::Bytes(value_bytes) => match String::from_utf8(value_bytes.to_owned()) {
                Err(e) => Err(ValueError::TypeConversionError(
                    TypeConversionError::FromUtf8Error(e),
                )),
                Ok(string_value) => Ok(string_value),
            },
            ValueType::Integer64(value_integer) => Ok(value_integer.to_string()),
            _ => Err(ValueError::TypeConversionImpossible),
        }
    }

    /// Check if this entry is expired.
    pub fn is_expired_entry(&self, option_now: Option<Instant>) -> bool {
        let now = option_now.unwrap_or(Instant::now());
        now >= self.expiration
    }
}
