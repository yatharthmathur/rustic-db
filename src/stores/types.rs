use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone)]
pub enum ValueType {
    Integer64(i64),
    Bytes(Vec<u8>),
    String(String),
    Deque(VecDeque<String>),
    Set(HashSet<String>),
    HashMap(HashMap<String, String>),
}
