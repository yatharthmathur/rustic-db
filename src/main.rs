mod data_store;
use data_store::store::KeyValueStore;

fn main() {
    println!("Hello, world!");
    let _store = KeyValueStore::new(100);
}
