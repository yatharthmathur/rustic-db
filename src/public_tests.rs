use crate::KeyValueStore;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[test]
fn test_contains() {
    let mut store = KeyValueStore::new(0);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    assert!(store.contains_key("ABC".to_string()));
    assert_ne!(store.contains_key("ABC".to_string()), false);
}

#[test]
fn test_clear_all_expired_keys() {
    let mut store = KeyValueStore::new(5000);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(250));
    store.set_string("XYZ".to_string(), "HELLO".to_string(), Some(250));
    store.set_string("DEF".to_string(), "HELLO".to_string(), Some(250));

    store.clear_all_expired_keys();

    assert!(store.contains_key("ABC".to_string()));

    std::thread::sleep(Duration::from_millis(250));
    assert!(store.contains_key("ABC".to_string()));
    assert!(store.contains_key("DEF".to_string()));
    assert!(store.contains_key("XYZ".to_string()));

    assert!(store.is_expired("ABC".to_string()).unwrap());

    store.clear_all_expired_keys();
    assert_eq!(store.contains_key("ABC".to_string()), false);
    assert_eq!(store.contains_key("DEF".to_string()), false);
    assert_eq!(store.contains_key("XYZ".to_string()), false);

    assert_eq!(store.is_expired("ABC".to_string()), None);
}

#[test]
fn test_set_get_string() {
    let mut store = KeyValueStore::new(0);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    assert_eq!(
        store.get_string("ABC".to_string()).unwrap().unwrap(),
        "HELLO".to_string()
    );

    // Can set string convertible bytes and fetch it back as string.
    store.set_bytes("XYZ".to_string(), "HELLO".as_bytes().to_vec(), Some(5000));
    assert_eq!(
        store.get_string("ABC".to_string()).unwrap().unwrap(),
        "HELLO".to_string()
    );
}

#[test]
fn test_pop_string() {
    let mut store = KeyValueStore::new(0);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    assert_eq!(
        store.pop_string("ABC".to_string()).unwrap().unwrap(),
        "HELLO"
    );
}

#[test]
fn test_set_get_bytes() {
    let mut store = KeyValueStore::new(0);
    store.set_bytes("ABC".to_string(), "HELLO".as_bytes().to_vec(), Some(5000));
    assert_eq!(
        store.get_bytes("ABC".to_string()).unwrap().unwrap(),
        "HELLO".as_bytes().to_vec()
    );
    assert_eq!(
        store.get_string("ABC".to_string()).unwrap().unwrap(),
        "HELLO".to_string()
    );
}

#[test]
fn test_pop_bytes() {
    let mut store = KeyValueStore::new(0);
    store.set_bytes("ABC".to_string(), "HELLO".as_bytes().to_vec(), Some(5000));
    assert_eq!(
        store.pop_bytes("ABC".to_string()).unwrap().unwrap(),
        "HELLO".as_bytes().to_vec()
    );
}

#[test]
fn test_get_set_binary_data() {
    #[derive(Serialize, Deserialize)]
    struct LocalStruct {
        test1: f64,
        test2: String,
    }

    let local_struct_instance: LocalStruct = LocalStruct {
        test1: 3.1415,
        test2: "Hey there".to_string(),
    };
    let mut store = KeyValueStore::new(0);
    let bin_code = bincode::serialize(&local_struct_instance).unwrap();
    store.set_bytes("ABC".to_string(), bin_code, Some(5000));

    if let Some(vec_val) = store.pop_bytes("ABC".to_string()) {
        let new_local_struct: LocalStruct = bincode::deserialize(&vec_val.unwrap()).unwrap();
        let struct_to_compare = LocalStruct {
            test1: 3.1415,
            test2: "Hey there".to_string(),
        };
        assert_eq!(new_local_struct.test1, struct_to_compare.test1);
        assert_eq!(new_local_struct.test2, struct_to_compare.test2);
    }
}

#[test]
fn test_remove_key() {
    let mut store = KeyValueStore::new(0);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    store.remove("ABC".to_string());
    assert!(store.get_string("ABC".to_string()).is_none());
    assert_eq!(store.contains_key("ABC".to_string()), false);
}

#[test]
fn test_key_expiry() {
    let mut store = KeyValueStore::new(500);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(500));
    store.set_string("XYZ".to_string(), "HELLO".to_string(), Some(500));

    std::thread::sleep(Duration::from_millis(250));
    match store.get_string("ABC".to_string()) {
        Some(_) => assert!(true),
        None => assert!(false),
    };
    match store.get_string("XYZ".to_string()) {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    std::thread::sleep(Duration::from_millis(250));
    match store.get_string("ABC".to_string()) {
        None => assert!(true),
        Some(_) => assert!(false),
    };
    match store.get_string("XYZ".to_string()) {
        None => assert!(true),
        Some(_) => assert!(false),
    };
}

#[test]
fn test_clear_store() {
    let mut store = KeyValueStore::new(5000);
    store.set_string("ABC".to_string(), "HELLO".to_string(), None);
    store.set_string("XYZ".to_string(), "HELLO".to_string(), None);
    store.set_string("DEF".to_string(), "HELLO".to_string(), None);
    store.clear();

    match store.get_string("ABC".to_string()) {
        None => assert!(true),
        Some(_) => assert!(false),
    };

    match store.get_string("DEF".to_string()) {
        None => assert!(true),
        Some(_) => assert!(false),
    };

    match store.get_string("XYZ".to_string()) {
        None => assert!(true),
        Some(_) => assert!(false),
    };
}
