pub mod tests {
    use crate::KeyValueStore;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[test]
    fn test_contains() {
        let mut store = KeyValueStore::new(0);
        store.set_with_string_value("ABC".to_string(), "HELLO".to_string(), Some(5000));
        assert!(store.contains("ABC".to_string()));
        assert_ne!(store.contains("ABC".to_string()), false);
    }

    #[test]
    fn test_set_get() {
        let mut store = KeyValueStore::new(0);
        store.set_with_string_value("ABC".to_string(), "HELLO".to_string(), Some(5000));
        if let Some(val) = store.get_as_string("ABC".to_string()) {
            assert_eq!(val, "HELLO".to_string());
        }
    }

    #[test]
    fn test_set_get_vec_u8() {
        let mut store = KeyValueStore::new(0);
        store.set("ABC".to_string(), "HELLO".as_bytes().to_vec(), Some(5000));
        if let Some(val) = store.get_as_string("ABC".to_string()) {
            assert_eq!(val, "HELLO".to_string());
        }

        if let Some(vec_val) = store.get("ABC".to_string()) {
            if let Ok(val) = String::from_utf8(vec_val.to_owned()) {
                assert_eq!(val, "HELLO".to_string());
            }
        }
    }

    #[test]
    fn test_pop_vec_u8() {
        let mut store = KeyValueStore::new(0);
        store.set("ABC".to_string(), "HELLO".as_bytes().to_vec(), Some(5000));
        if let Some(val) = store.get_as_string("ABC".to_string()) {
            assert_eq!(val, "HELLO".to_string());
        }

        if let Some(vec_val) = store.pop("ABC".to_string()) {
            if let Ok(val) = String::from_utf8(vec_val.to_owned()) {
                assert_eq!(val, "HELLO".to_string());
            }
        }
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
        let mut store = KeyValueStore::new(0);
        let bin_code = bincode::serialize(&local_struct_instance).unwrap();
        store.set("ABC".to_string(), bin_code, Some(5000));

        if let Some(vec_val) = store.pop("ABC".to_string()) {
            let new_local_struct: LocalStruct = bincode::deserialize(&vec_val).unwrap();
            let struct_to_compare = LocalStruct {
                test1: 3.1415,
                test2: "Hey there".to_string(),
            };
            assert_eq!(new_local_struct.test1, struct_to_compare.test1);
            assert_eq!(new_local_struct.test2, struct_to_compare.test2);
        }
    }

    #[test]
    fn test_pop_key() {
        let mut store = KeyValueStore::new(0);
        store.set_with_string_value("ABC".to_string(), "HELLO".to_string(), Some(5000));
        if let Some(val) = store.get_as_string("ABC".to_string()) {
            assert_eq!(val, "HELLO".to_string());
        };

        if let Some(val) = store.pop_as_string("ABC".to_string()) {
            assert_eq!(val, "HELLO".to_string());
        };

        match store.get_as_string("ABC".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        };
    }

    #[test]
    fn test_remove_key() {
        let mut store = KeyValueStore::new(0);
        store.set_with_string_value("ABC".to_string(), "HELLO".to_string(), Some(5000));
        store.remove("XYZ".to_string());
        if let Some(val) = store.get_as_string("ABC".to_string()) {
            assert_eq!(val, "HELLO".to_string());
        };

        store.remove("ABC".to_string());

        match store.get_as_string("ABC".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        };
    }

    #[test]
    fn test_key_expiry() {
        let mut store = KeyValueStore::new(500);
        store.set_with_string_value("ABC".to_string(), "HELLO".to_string(), Some(500));
        store.set_with_string_value("XYZ".to_string(), "HELLO".to_string(), Some(500));

        std::thread::sleep(Duration::from_millis(250));
        match store.get_as_string("ABC".to_string()) {
            Some(_) => assert!(true),
            None => assert!(false),
        };
        match store.get_as_string("XYZ".to_string()) {
            Some(_) => assert!(true),
            None => assert!(false),
        };

        std::thread::sleep(Duration::from_millis(250));
        match store.get_as_string("ABC".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        };
        match store.get_as_string("XYZ".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        };
    }

    #[test]
    fn test_clear_store() {
        let mut store = KeyValueStore::new(5000);
        store.set_with_string_value("ABC".to_string(), "HELLO".to_string(), None);
        store.set_with_string_value("XYZ".to_string(), "HELLO".to_string(), None);
        store.set_with_string_value("DEF".to_string(), "HELLO".to_string(), None);
        store.clear();

        match store.get_as_string("ABC".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        };

        match store.get_as_string("DEF".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        };

        match store.get_as_string("XYZ".to_string()) {
            None => assert!(true),
            Some(_) => assert!(false),
        };
    }
}
