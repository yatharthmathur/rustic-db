use super::manager::RusticManager;

#[test]
fn test_create_store() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    let store = manager.create_store("store0", None);
    store.set_i64("key", 5, None);
    assert_eq!(store.get_i64("key").unwrap().unwrap(), 5);
}

#[test]
fn test_get_store() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    manager.create_store("store0", None);

    let mut_store = manager.get_store_mut("store0").unwrap();
    mut_store.set_i64("key", 5, None);

    let immut_store = manager.get_store("store0").unwrap();
    assert_eq!(immut_store.get_i64("key").unwrap().unwrap(), 5);
}

#[test]
fn test_remove_store() {
    let mut manager = RusticManager::new();
    manager.create_store("store0", None);

    let store = manager.get_store_mut("store0").unwrap();
    store.set_i64("key", 5, None);
    assert_eq!(store.get_i64("key").unwrap().unwrap(), 5);

    manager.remove_store("store0");
    let store = manager.get_store_mut("store0");
    assert!(store.is_none());
}

#[test]
#[should_panic]
fn test_duplicate_store_names() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    manager.create_store("store0", None);
    manager.create_store("store0", None);
}

#[test]
fn test_list_store_names() {
    let mut manager = RusticManager::new();
    // create a store with infinite TTL
    manager.create_store("store0", None);
    manager.create_store("store1", None);
    manager.create_store("store2", None);

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
