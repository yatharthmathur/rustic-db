mod data_store;

fn main() {
    println!("Hello, world!");
    let _store = data_store::store::KeyValueStore::new(100);
}
