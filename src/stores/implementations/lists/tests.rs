use crate::stores::store::KeyValueStore;

#[test]
fn test_set_get_deque() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_list(
        "ABC".to_string(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
        None,
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );

    store.set_string("DEF".to_string(), "XYZ".to_string(), None);
    assert_eq!(
        store.get_list("DEF".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );

    assert_eq!(
        store.pop_list("DEF".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string(), "Z".to_string()]
    );
}

#[test]
fn test_deque_methods() {
    let mut store = KeyValueStore::new("new_store".to_owned(), None);
    store.set_list("ABC".to_string(), vec!["X".to_string()], None);
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string()]
    );
    assert_eq!(
        store.list_front("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );
    assert_eq!(
        store.list_back("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );

    assert_eq!(
        store
            .list_pushb("ABC".to_string(), "Y".to_string())
            .unwrap()
            .unwrap(),
        "Y".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string()]
    );

    assert_eq!(
        store
            .list_pushf("ABC".to_string(), "A".to_string())
            .unwrap()
            .unwrap(),
        "A".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["A".to_string(), "X".to_string(), "Y".to_string()]
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 3);
    assert_eq!(
        store.list_front("ABC".to_string()).unwrap().unwrap(),
        "A".to_string()
    );
    assert_eq!(
        store.list_back("ABC".to_string()).unwrap().unwrap(),
        "Y".to_string()
    );

    assert_eq!(
        store.list_popf("ABC".to_string()).unwrap().unwrap(),
        "A".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string(), "Y".to_string()]
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 2);
    assert_eq!(
        store.list_front("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );
    assert_eq!(
        store.list_back("ABC".to_string()).unwrap().unwrap(),
        "Y".to_string()
    );

    assert_eq!(
        store.list_popb("ABC".to_string()).unwrap().unwrap(),
        "Y".to_string()
    );
    assert_eq!(
        store.get_list("ABC".to_string()).unwrap().unwrap(),
        vec!["X".to_string()]
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 1);

    assert_eq!(
        store.list_popb("ABC".to_string()).unwrap().unwrap(),
        "X".to_string()
    );
    assert_eq!(store.list_size("ABC".to_string()).unwrap().unwrap(), 0);
}
