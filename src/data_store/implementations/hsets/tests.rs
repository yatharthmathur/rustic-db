use std::collections::HashSet;

use crate::data_store::store::KeyValueStore;

#[test]
fn test_set_get_hset() {
    let mut store = KeyValueStore::new(5000);
    store.set_hset(
        "ABC".to_string(),
        vec![
            "X".to_string(),
            "Y".to_string(),
            "Z".to_string(),
            "Y".to_string(),
            "Z".to_string(),
        ],
        None,
    );

    let mut sorted_list_in_store = store.get_list("ABC".to_string()).unwrap().unwrap();
    sorted_list_in_store.sort();
    assert_eq!(
        sorted_list_in_store,
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );

    store.set_string("DEF".to_string(), "XYZYZYZ".to_string(), None);
    assert_eq!(
        store.get_hset("DEF".to_string()).unwrap().unwrap(),
        HashSet::from_iter(vec!["X".to_string(), "Y".to_string(), "Z".to_string()])
    );
}

#[test]
fn test_add_remove_hset() {
    let mut store = KeyValueStore::new(5000);
    store.set_hset(
        "ABC".to_string(),
        vec![
            "X".to_string(),
            "Y".to_string(),
            "Z".to_string(),
            "Y".to_string(),
            "Z".to_string(),
        ],
        None,
    );

    assert_eq!(store.hset_size("ABC".to_string()).unwrap().unwrap(), 3);

    assert_eq!(
        store
            .hset_add("ABC".to_string(), "Q".to_string())
            .unwrap()
            .unwrap(),
        4
    );
    assert!(store
        .hset_contains("ABC".to_string(), "Q".to_string())
        .unwrap()
        .unwrap());

    assert!(!store
        .hset_contains("ABC".to_string(), "A".to_string())
        .unwrap()
        .unwrap());
    assert_eq!(
        store
            .hset_remove("ABC".to_string(), "A".to_string())
            .unwrap()
            .unwrap(),
        4
    );

    assert_eq!(
        store
            .hset_remove("ABC".to_string(), "X".to_string())
            .unwrap()
            .unwrap(),
        3
    );

    let mut sorted_list_in_store = store.get_list("ABC".to_string()).unwrap().unwrap();
    sorted_list_in_store.sort();
    assert_eq!(
        sorted_list_in_store,
        vec!["Q".to_string(), "Y".to_string(), "Z".to_string()]
    );
}

#[test]
fn test_hset_union_intersection_difference() {
    let mut store = KeyValueStore::new(5000);
    store.set_hset(
        "ABC".to_string(),
        vec![
            "X".to_string(),
            "Y".to_string(),
            "Z".to_string(),
            "A".to_string(),
            "B".to_string(),
            "B".to_string(),
        ],
        None,
    );

    store.set_hset(
        "DEF".to_string(),
        vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ],
        None,
    );

    assert_eq!(store.hset_size("ABC".to_string()).unwrap().unwrap(), 5);
    assert_eq!(store.hset_size("DEF".to_string()).unwrap().unwrap(), 5);

    let mut intersection = store
        .hset_intersection("ABC".to_string(), "DEF".to_string())
        .unwrap()
        .unwrap();
    intersection.sort();

    let mut union = store
        .hset_union("ABC".to_string(), "DEF".to_string())
        .unwrap()
        .unwrap();
    union.sort();

    let mut difference = store
        .hset_difference("ABC".to_string(), "DEF".to_string())
        .unwrap()
        .unwrap();
    difference.sort();

    assert_eq!(intersection, vec!["A", "B"]);
    assert_eq!(union, vec!["A", "B", "C", "D", "E", "X", "Y", "Z"]);
    assert_eq!(difference, vec!["X", "Y", "Z"]);
}
