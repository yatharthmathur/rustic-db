use super::manager::RusticManager;

#[test]
fn test_create_store() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    let store = manager.create_store("store0".to_owned(), None);
    store.set_i64("key".to_owned(), 5, None);
    assert_eq!(store.get_i64("key".to_owned()).unwrap().unwrap(), 5);
}

#[test]
fn test_get_store() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    manager.create_store("store0".to_owned(), None);

    let mut_store = manager.get_store_mut("store0".to_owned()).unwrap();
    mut_store.set_i64("key".to_owned(), 5, None);

    let immut_store = manager.get_store("store0".to_owned()).unwrap();
    assert_eq!(immut_store.get_i64("key".to_owned()).unwrap().unwrap(), 5);
}

#[test]
fn test_remove_store() {
    let mut manager = RusticManager::new();
    manager.create_store("store0".to_owned(), None);

    let store = manager.get_store_mut("store0".to_owned()).unwrap();
    store.set_i64("key".to_owned(), 5, None);
    assert_eq!(store.get_i64("key".to_owned()).unwrap().unwrap(), 5);

    manager.remove_store("store0".to_owned());
    let store = manager.get_store_mut("store0".to_owned());
    assert!(store.is_none());
}

#[test]
#[should_panic]
fn test_duplicate_store_names() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    manager.create_store("store0".to_owned(), None);
    manager.create_store("store0".to_owned(), None);
}

#[test]
fn test_list_store_names() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    manager.create_store("store0".to_owned(), None);
    manager.create_store("store1".to_owned(), None);
    manager.create_store("store2".to_owned(), None);

    let mut expected_keys: Vec<String> = manager
        .list_store_names()
        .iter()
        .map(|key| key.to_string())
        .collect();
    expected_keys.sort();
    assert_eq!(
        expected_keys,
        vec![
            "store0".to_owned(),
            "store1".to_owned(),
            "store2".to_owned()
        ]
    )
}
