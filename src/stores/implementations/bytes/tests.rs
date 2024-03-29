use serde::{Deserialize, Serialize};

use crate::stores::store::KeyValueStore;

#[test]
fn test_set_get_bytes() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_bytes("ABC", "HELLO".as_bytes().to_vec(), Some(5000));
    assert_eq!(
        store.get_bytes("ABC").unwrap().unwrap(),
        "HELLO".as_bytes().to_vec()
    );
}

#[test]
fn test_pop_bytes() {
    let mut store = KeyValueStore::new("new_store", None);
    store.set_bytes("ABC", "HELLO".as_bytes().to_vec(), Some(5000));
    assert_eq!(
        store.pop_bytes("ABC").unwrap().unwrap(),
        "HELLO".as_bytes().to_vec()
    );
}

#[test]
fn test_get_set_binary_data() {
    #[derive(Serialize, Deserialize)]
    struct LocalStruct {
        test1: f64,
        test2: String,
    }

    let local_struct_instance: LocalStruct = LocalStruct {
        test1: 3.1415,
        test2: "Hey there".to_string(),
    };
    let mut store = KeyValueStore::new("new_store", None);
    let bin_code = bincode::serialize(&local_struct_instance).unwrap();
    store.set_bytes("ABC", bin_code, Some(5000));

    if let Some(vec_val) = store.pop_bytes("ABC") {
        let new_local_struct: LocalStruct = bincode::deserialize(&vec_val.unwrap()).unwrap();
        let struct_to_compare = LocalStruct {
            test1: 3.1415,
            test2: "Hey there".to_string(),
        };
        assert_eq!(new_local_struct.test1, struct_to_compare.test1);
        assert_eq!(new_local_struct.test2, struct_to_compare.test2);
    }
}
