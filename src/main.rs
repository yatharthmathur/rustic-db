mod data_store;

use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let _store = data_store::store::KeyValueStore::new(Duration::new(0, 100000000));
}
