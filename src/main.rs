mod managers;
mod stores;
use managers::manager::RusticManager;
use stores::store::KeyValueStore;

fn main() {
    println!("Hello, rustics!");
    let mut manager = RusticManager::new();
    manager.create_store("new_store_0".to_owned(), Some(5000));
    {
        let store = manager.get_store_mut("new_store_0".to_owned()).unwrap();
        store.set_i64("key".to_owned(), 500, None);
        println!(
            "Hello, {} rustics",
            store.get_i64("key".to_owned()).unwrap().unwrap()
        );
    }
    manager.remove_store("new_store_0".to_owned());
}
