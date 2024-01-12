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

    assert_eq!(
        store.get_bytes("ABC".to_string()).unwrap().unwrap(),
        "HELLO".as_bytes().to_vec()
    );

    // Can set bytes and fetch it back as string.
    store.set_bytes("bytes".to_string(), "HELLO".as_bytes().to_vec(), Some(5000));
    assert_eq!(
        store.get_string("bytes".to_string()).unwrap().unwrap(),
        "HELLO".to_string()
    );

    // Can set string and fetch it back as int.
    store.set_string("integer".to_string(), "64".to_string(), Some(5000));
    assert_eq!(store.get_i64("integer".to_string()).unwrap().unwrap(), 64);
}

#[test]
fn test_get_set_i64() {
    let mut store = KeyValueStore::new(0);
    store.set_i64("ABC".to_string(), 999, Some(5000));
    assert_eq!(store.get_i64("ABC".to_string()).unwrap().unwrap(), 999);

    assert_eq!(
        store.get_string("ABC".to_string()).unwrap().unwrap(),
        "999".to_string()
    );

    assert_eq!(
        store.get_bytes("ABC".to_string()).unwrap().unwrap(),
        i64::to_le_bytes(999)
    );
}

#[test]
fn test_pop_string() {
    let mut store = KeyValueStore::new(0);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    assert_eq!(
        store.pop_string("ABC".to_string()).unwrap().unwrap(),
        "HELLO".to_string()
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

#[test]
fn test_incr_decr() {
    let mut store = KeyValueStore::new(5000);
    store.set_string("ABC".to_string(), "68".to_string(), None);
    assert_eq!(store.incr("ABC".to_string(), None).unwrap().unwrap(), 69);
    assert_eq!(store.get_i64("ABC".to_string()).unwrap().unwrap(), 69);

    assert_eq!(store.decr("ABC".to_string(), Some(8)).unwrap().unwrap(), 61);
    assert_eq!(
        store.get_string("ABC".to_string()).unwrap().unwrap(),
        "61".to_string()
    );

    store.set_string("xyz".to_string(), "ab123x".to_string(), None);
    assert!(store.incr("xyz".to_string(), Some(10)).unwrap().is_err())
}

#[test]
fn test_set_get_list() {
    let mut store = KeyValueStore::new(5000);
    store.set_list(
        "ABC".to_string(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
        None,
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );

    store.set_string("DEF".to_string(), "XYZ".to_string(), None);
    assert_eq!(
        store.get_list("DEF".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );
}

#[test]
fn test_list_methods() {
    let mut store = KeyValueStore::new(5000);
    store.set_list("ABC".to_string(), vec!["X".to_string()], None);
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string()]
    );
    assert_eq!(
        store.list_front("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );
    assert_eq!(
        store.list_back("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );

    assert_eq!(
        store
            .list_pushb("ABC".to_string(), "Y".to_string())
            .unwrap()
            .unwrap(),
        "Y".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string()]
    );

    assert_eq!(
        store
            .list_pushf("ABC".to_string(), "A".to_string())
            .unwrap()
            .unwrap(),
        "A".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["A".to_string(), "X".to_string(), "Y".to_string()]
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 3);
    assert_eq!(
        store.list_front("ABC".to_string()).unwrap().unwrap(),
        "A".to_string()
    );
    assert_eq!(
        store.list_back("ABC".to_string()).unwrap().unwrap(),
        "Y".to_string()
    );

    assert_eq!(
        store.list_popf("ABC".to_string()).unwrap().unwrap(),
        "A".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string()]
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 2);
    assert_eq!(
        store.list_front("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );
    assert_eq!(
        store.list_back("ABC".to_string()).unwrap().unwrap(),
        "Y".to_string()
    );

    assert_eq!(
        store.list_popb("ABC".to_string()).unwrap().unwrap(),
        "Y".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string()]
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 1);

    assert_eq!(
        store.list_popb("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 0);
}
