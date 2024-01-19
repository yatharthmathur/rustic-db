use crate::stores::store::KeyValueStore;

#[test]
fn test_set_get_deque() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_list(
        "ABC",
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
        None,
    );
    assert_eq!(
        store.get_list("ABC").unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );

    store.set_string("DEF", "XYZ".to_string(), None);
    assert_eq!(
        store.get_list("DEF").unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );

    assert_eq!(
        store.pop_list("DEF").unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );
}

#[test]
fn test_deque_methods() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_list("ABC", vec!["X".to_string()], None);
    assert_eq!(
        store.get_list("ABC").unwrap().unwrap(),
        vec!["X".to_string()]
    );
    assert_eq!(store.list_front("ABC").unwrap().unwrap(), "X".to_string());
    assert_eq!(store.list_back("ABC").unwrap().unwrap(), "X".to_string());

    assert_eq!(
        store.list_pushb("ABC", "Y".to_string()).unwrap().unwrap(),
        "Y".to_string()
    );
    assert_eq!(
        store.get_list("ABC").unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string()]
    );

    assert_eq!(
        store.list_pushf("ABC", "A".to_string()).unwrap().unwrap(),
        "A".to_string()
    );
    assert_eq!(
        store.get_list("ABC").unwrap().unwrap(),
        vec!["A".to_string(), "X".to_string(), "Y".to_string()]
    );
    assert_eq!(store.list_size("ABC").unwrap().unwrap(), 3);
    assert_eq!(store.list_front("ABC").unwrap().unwrap(), "A".to_string());
    assert_eq!(store.list_back("ABC").unwrap().unwrap(), "Y".to_string());

    assert_eq!(store.list_popf("ABC").unwrap().unwrap(), "A".to_string());
    assert_eq!(
        store.get_list("ABC").unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string()]
    );
    assert_eq!(store.list_size("ABC").unwrap().unwrap(), 2);
    assert_eq!(store.list_front("ABC").unwrap().unwrap(), "X".to_string());
    assert_eq!(store.list_back("ABC").unwrap().unwrap(), "Y".to_string());

    assert_eq!(store.list_popb("ABC").unwrap().unwrap(), "Y".to_string());
    assert_eq!(
        store.get_list("ABC").unwrap().unwrap(),
        vec!["X".to_string()]
    );
    assert_eq!(store.list_size("ABC").unwrap().unwrap(), 1);

    assert_eq!(store.list_popb("ABC").unwrap().unwrap(), "X".to_string());
    assert_eq!(store.list_size("ABC").unwrap().unwrap(), 0);
}
