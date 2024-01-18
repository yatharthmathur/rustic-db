use crate::KeyValueStore;
use std::time::Duration;

#[test]
fn test_contains() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    assert!(store.contains_key("ABC".to_string()));
    assert_ne!(store.contains_key("ABC".to_string()), false);
}

#[test]
fn test_clear_all_expired_keys() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
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
fn test_remove_key() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    store.remove("ABC".to_string());
    assert!(store.get_string("ABC".to_string()).is_none());
    assert_eq!(store.contains_key("ABC".to_string()), false);
}

#[test]
fn test_key_expiry() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(500));
    store.set_string("XYZ".to_string(), "HELLO".to_string(), Some(500));

    // Infinitely stored value.
    store.set_string("Inf".to_string(), "HELLO".to_string(), None);

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
    match store.get_string("Inf".to_string()) {
        None => assert!(false),
        Some(_) => assert!(true),
    };
}

#[test]
fn test_clear_store() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
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
