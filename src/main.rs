mod data_store;
use data_store::store::KeyValueStore;

fn main() {
    println!("Hello, rustics!");
    let _store = KeyValueStore::new(0);
}

#[cfg(test)]
mod tests;
