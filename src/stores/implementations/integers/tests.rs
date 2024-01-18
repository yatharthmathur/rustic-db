use crate::stores::store::KeyValueStore;

#[test]
fn test_get_set_i64() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
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

    assert_eq!(store.pop_i64("ABC".to_string()).unwrap().unwrap(), 999);
}

#[test]
fn test_incr_decr() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
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
