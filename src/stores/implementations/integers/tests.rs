use crate::stores::store::KeyValueStore;

#[test]
fn test_get_set_i64() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_i64("ABC", 999, Some(5000));
    assert_eq!(store.get_i64("ABC").unwrap().unwrap(), 999);

    assert_eq!(store.get_string("ABC").unwrap().unwrap(), "999".to_string());

    assert_eq!(
        store.get_bytes("ABC").unwrap().unwrap(),
        i64::to_le_bytes(999)
    );

    assert_eq!(store.pop_i64("ABC").unwrap().unwrap(), 999);
}

#[test]
fn test_incr_decr() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_string("ABC", "68".to_string(), None);
    assert_eq!(store.incr("ABC", None).unwrap().unwrap(), 69);
    assert_eq!(store.get_i64("ABC").unwrap().unwrap(), 69);

    assert_eq!(store.decr("ABC", Some(8)).unwrap().unwrap(), 61);
    assert_eq!(store.get_string("ABC").unwrap().unwrap(), "61".to_string());

    store.set_string("xyz", "ab123x".to_string(), None);
    assert!(store.incr("xyz", Some(10)).unwrap().is_err())
}
