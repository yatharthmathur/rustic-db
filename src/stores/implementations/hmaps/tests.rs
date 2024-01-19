use crate::stores::store::KeyValueStore;

#[test]
fn test_insert_get_hmap() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_hmap("ABC", vec![], None);

    assert_eq!(
        store
            .hmap_insert("ABC", ("A".to_owned(), "B".to_owned()))
            .unwrap()
            .unwrap(),
        1
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_insert("ABC", ("B".to_owned(), "C".to_owned()))
            .unwrap()
            .unwrap(),
        2
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 2);

    assert!(store
        .hmap_contains_key("ABC", "B".to_owned())
        .unwrap()
        .unwrap());
    assert_eq!(
        store
            .hmap_contains_key("ABC", "C".to_owned())
            .unwrap()
            .unwrap(),
        false
    );
}

#[test]
fn test_hmap_keys_values_items() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_hmap("ABC", vec![], None);

    assert_eq!(
        store
            .hmap_insert("ABC", ("A".to_owned(), "B".to_owned()))
            .unwrap()
            .unwrap(),
        1
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_insert("ABC", ("B".to_owned(), "C".to_owned()))
            .unwrap()
            .unwrap(),
        2
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 2);

    assert_eq!(
        store
            .hmap_insert("ABC", ("C".to_owned(), "A".to_owned()))
            .unwrap()
            .unwrap(),
        3
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 3);

    let keys = store.hmap_keys("ABC").unwrap().unwrap();
    let mut sorted_keys = keys.to_owned();
    sorted_keys.sort();

    let values = store.hmap_values("ABC").unwrap().unwrap();
    let values_in_order_of_keys: Vec<String> = keys
        .iter()
        .map(|item| store.hmap_get("ABC", item.to_owned()).unwrap().unwrap())
        .collect();

    let mut items = store.hmap_items("ABC").unwrap().unwrap();
    items.sort();

    assert_eq!(sorted_keys, vec!["A", "B", "C"]);
    assert_eq!(values, values_in_order_of_keys);
    assert_eq!(
        items,
        vec![
            ("A".to_owned(), "B".to_owned()),
            ("B".to_owned(), "C".to_owned()),
            ("C".to_owned(), "A".to_owned()),
        ]
    );
}

#[test]
fn test_remove_hmap() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_hmap("ABC", vec![], None);

    assert_eq!(
        store
            .hmap_insert("ABC", ("A".to_owned(), "B".to_owned()))
            .unwrap()
            .unwrap(),
        1
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_insert("ABC", ("B".to_owned(), "C".to_owned()))
            .unwrap()
            .unwrap(),
        2
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 2);

    assert_eq!(
        store.hmap_remove("ABC", "A".to_owned()).unwrap().unwrap(),
        ("A".to_owned(), "B".to_owned())
    );
    assert_eq!(store.hmap_size("ABC").unwrap().unwrap(), 1);

    assert_eq!(
        store
            .hmap_contains_key("ABC", "A".to_owned())
            .unwrap()
            .unwrap(),
        false
    );

    assert!(store.hmap_remove("ABC", "C".to_owned()).is_none());
    assert_eq!(
        store
            .hmap_contains_key("ABC", "C".to_owned())
            .unwrap()
            .unwrap(),
        false
    );
}
