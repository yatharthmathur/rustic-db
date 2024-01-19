use crate::KeyValueStore;
use std::time::Duration;

#[test]
fn test_contains() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_string("ABC", "HELLO".to_string(), Some(5000));
    assert!(store.contains_key("ABC"));
    assert_ne!(store.contains_key("ABC"), false);
}

#[test]
fn test_clear_all_expired_keys() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_string("ABC", "HELLO".to_string(), Some(250));
    store.set_string("XYZ", "HELLO".to_string(), Some(250));
    store.set_string("DEF", "HELLO".to_string(), Some(250));

    store.clear_all_expired_keys();

    assert!(store.contains_key("ABC"));

    std::thread::sleep(Duration::from_millis(250));
    assert!(store.contains_key("ABC"));
    assert!(store.contains_key("DEF"));
    assert!(store.contains_key("XYZ"));

    assert!(store.is_expired("ABC").unwrap());

    store.clear_all_expired_keys();
    assert_eq!(store.contains_key("ABC"), false);
    assert_eq!(store.contains_key("DEF"), false);
    assert_eq!(store.contains_key("XYZ"), false);

    assert_eq!(store.is_expired("ABC"), None);
}

#[test]
fn test_remove_key() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_string("ABC", "HELLO".to_string(), Some(5000));
    store.remove("ABC");
    assert!(store.get_string("ABC").is_none());
    assert_eq!(store.contains_key("ABC"), false);
}

#[test]
fn test_key_expiry() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_string("ABC", "HELLO".to_string(), Some(500));
    store.set_string("XYZ", "HELLO".to_string(), Some(500));

    // Infinitely stored value.
    store.set_string("Inf", "HELLO".to_string(), None);

    std::thread::sleep(Duration::from_millis(250));
    match store.get_string("ABC") {
        Some(_) => assert!(true),
        None => assert!(false),
    };
    match store.get_string("XYZ") {
        Some(_) => assert!(true),
        None => assert!(false),
    };

    std::thread::sleep(Duration::from_millis(250));
    match store.get_string("ABC") {
        None => assert!(true),
        Some(_) => assert!(false),
    };
    match store.get_string("XYZ") {
        None => assert!(true),
        Some(_) => assert!(false),
    };
    match store.get_string("Inf") {
        None => assert!(false),
        Some(_) => assert!(true),
    };
}

#[test]
fn test_clear_store() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_string("ABC", "HELLO".to_string(), None);
    store.set_string("XYZ", "HELLO".to_string(), None);
    store.set_string("DEF", "HELLO".to_string(), None);
    store.clear();

    match store.get_string("ABC") {
        None => assert!(true),
        Some(_) => assert!(false),
    };

    match store.get_string("DEF") {
        None => assert!(true),
        Some(_) => assert!(false),
    };

    match store.get_string("XYZ") {
        None => assert!(true),
        Some(_) => assert!(false),
    };
}
