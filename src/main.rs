mod managers;
mod stores;
mod utils;

use env_logger;
use log::info;
use managers::manager::RusticManager;
use std::sync::Arc;
use stores::store::KeyValueStore;
use tokio::{net::TcpListener, sync::RwLock, time::Instant};
use utils::{
    constants::{INTERNAL_STORE_NAME, IP_PORT_BINDING},
    event_loop::event_loop,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let start_instant = Instant::now();
    info!("Hello, rustics!");

    // Bind the listener to the address
    let listener = TcpListener::bind(IP_PORT_BINDING).await.unwrap();

    info!("Listening on {}", IP_PORT_BINDING);

    // Using RwLock as it allows us to spawn n number of threads to read data, but only 1 to write it.
    let shared_manager = Arc::new(RwLock::new(RusticManager::new()));
    shared_manager
        .write()
        .await
        .create_store(INTERNAL_STORE_NAME, None);

    // Start the event loop
    event_loop(listener, shared_manager, start_instant).await;
}
