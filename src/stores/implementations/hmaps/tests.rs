use crate::stores::store::KeyValueStore;

#[test]
fn test_insert_get_hmap() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_hmap("ABC".to_string(), vec![], None);

    assert_eq!(
        store
            .hmap_insert("ABC".to_string(), ("A".to_string(), "B".to_string()))
            .unwrap()
            .unwrap(),
        1
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_insert("ABC".to_string(), ("B".to_string(), "C".to_string()))
            .unwrap()
            .unwrap(),
        2
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 2);

    assert!(store
        .hmap_contains_key("ABC".to_string(), "B".to_string())
        .unwrap()
        .unwrap());
    assert_eq!(
        store
            .hmap_contains_key("ABC".to_string(), "C".to_string())
            .unwrap()
            .unwrap(),
        false
    );
}

#[test]
fn test_hmap_keys_values_items() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_hmap("ABC".to_string(), vec![], None);

    assert_eq!(
        store
            .hmap_insert("ABC".to_string(), ("A".to_string(), "B".to_string()))
            .unwrap()
            .unwrap(),
        1
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_insert("ABC".to_string(), ("B".to_string(), "C".to_string()))
            .unwrap()
            .unwrap(),
        2
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 2);

    assert_eq!(
        store
            .hmap_insert("ABC".to_string(), ("C".to_string(), "A".to_string()))
            .unwrap()
            .unwrap(),
        3
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 3);

    let keys = store.hmap_keys("ABC".to_string()).unwrap().unwrap();
    let mut sorted_keys = keys.to_owned();
    sorted_keys.sort();

    let values = store.hmap_values("ABC".to_string()).unwrap().unwrap();
    let values_in_order_of_keys: Vec<String> = keys
        .iter()
        .map(|item| {
            store
                .hmap_get("ABC".to_string(), item.to_owned())
                .unwrap()
                .unwrap()
        })
        .collect();

    let mut items = store.hmap_items("ABC".to_string()).unwrap().unwrap();
    items.sort();

    assert_eq!(sorted_keys, vec!["A", "B", "C"]);
    assert_eq!(values, values_in_order_of_keys);
    assert_eq!(
        items,
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "A".to_string())
        ]
    );
}

#[test]
fn test_remove_hmap() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_hmap("ABC".to_string(), vec![], None);

    assert_eq!(
        store
            .hmap_insert("ABC".to_string(), ("A".to_string(), "B".to_string()))
            .unwrap()
            .unwrap(),
        1
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_insert("ABC".to_string(), ("B".to_string(), "C".to_string()))
            .unwrap()
            .unwrap(),
        2
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 2);

    assert_eq!(
        store
            .hmap_remove("ABC".to_string(), "A".to_string())
            .unwrap()
            .unwrap(),
        ("A".to_string(), "B".to_string())
    );
    assert_eq!(store.hmap_size("ABC".to_string()).unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_contains_key("ABC".to_string(), "A".to_string())
            .unwrap()
            .unwrap(),
        false
    );

    assert!(store
        .hmap_remove("ABC".to_string(), "C".to_string())
        .is_none());
    assert_eq!(
        store
            .hmap_contains_key("ABC".to_string(), "C".to_string())
            .unwrap()
            .unwrap(),
        false
    );
}
