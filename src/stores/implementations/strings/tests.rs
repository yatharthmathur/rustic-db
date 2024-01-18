use crate::stores::store::KeyValueStore;

#[test]
fn test_set_get_string() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
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
fn test_pop_string() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_string("ABC".to_string(), "HELLO".to_string(), Some(5000));
    assert_eq!(
        store.pop_string("ABC".to_string()).unwrap().unwrap(),
        "HELLO".to_string()
    );
}
