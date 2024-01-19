mod managers;
mod stores;
use env_logger;
use log::{error, info};
use managers::manager::RusticManager;
use std::sync::Arc;
use stores::store::KeyValueStore;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
    time::{sleep, timeout, Duration, Instant},
};

async fn handle_client(
    mut tcp_stream: TcpStream,
    shared_manager: Arc<RwLock<RusticManager>>,
    start_instant: Instant,
) {
    // Your client handling logic goes here
    // TODO: Add all logic to handle client requests here.

    // This is how we can share the manager and store for the case where we just want to read.
    // NOTE: write_manager should only invoked if we want to write to the store.
    let mut write_manager = shared_manager.write().await;
    let store = write_manager
        .get_store_mut("default_store".to_owned())
        .unwrap();

    // // This is how we can share the manager and store for the case where we just want to read.
    // // We can only use one of these managers in a single thread or it will lead to a deadlock.
    // let manager = shared_manager.read().await;
    // let store = manager.get_store("default_store".to_owned()).unwrap();

    info!(
        "Handling client from: {:?}",
        tcp_stream.peer_addr().unwrap()
    );
    let buf: &mut Vec<u8> = &mut vec![];
    info!(
        "Message received: {:?}",
        tcp_stream.read_buf(buf).await.unwrap()
    );
    let string_buff: String = String::from_utf8_lossy(buf).to_string();
    info!("Message body: {:?}", string_buff);

    let value = store.get_string("ABC".to_owned());
    let return_buff: &[u8];

    if value.is_some() {
        let cache_value = value.unwrap().unwrap();
        info!("Cache value is present {:?}", cache_value.to_string());
        return_buff = cache_value.as_bytes();
        let res = tcp_stream.write(return_buff).await;
        info!("Response sent: {:?}", res.unwrap());
    } else {
        info!("Cache value is not present");
        store.set_string(
            "ABC".to_owned(),
            start_instant.elapsed().as_secs().to_string() + " " + string_buff.as_str(),
            Some(5000),
        );

        return_buff = "Cache is set".as_bytes();
        let res = tcp_stream.write(return_buff).await;
        info!("Response sent: {:?}", res.unwrap());
    }
}

async fn execute_periodic_tasks(
    shared_manager: Arc<RwLock<RusticManager>>,
    start_instant: Instant,
) {
    let elapsed = start_instant.elapsed().as_secs();
    if elapsed % 60 == 0 {
        // Every 60 seconds
        shared_manager
            .write()
            .await
            .clear_expired_keys_in_all_stores();
    }
}

async fn event_loop(
    listener: TcpListener,
    shared_manager: Arc<RwLock<RusticManager>>,
    start_instant: Instant,
) {
    loop {
        // Accept incoming connections for 5 seconds.
        match timeout(Duration::from_secs(5), listener.accept()).await {
            Ok(Ok((socket, _))) => {
                // Spawn a new task to handle the client
                tokio::spawn(handle_client(
                    socket,
                    Arc::clone(&shared_manager),
                    start_instant,
                ));
            }
            Ok(Err(err)) => error!("Error accepting connection: {:?}", err),
            Err(_) => {
                info!("No connections found, moving on to other tasks.")
            }
        }

        tokio::spawn(execute_periodic_tasks(
            Arc::clone(&shared_manager),
            start_instant,
        ));
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let start_instant = Instant::now();
    info!("Hello, rustics!");

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:29998").await.unwrap();

    info!("Listening on {}", "127.0.0.1:29998");

    // Using RwLock as it allows us to spawn n number of threads to read data, but only 1 to write it.
    let shared_manager = Arc::new(RwLock::new(RusticManager::new()));
    shared_manager
        .write()
        .await
        .create_store("default_store".to_owned(), None);

    // Start the event loop
    event_loop(listener, shared_manager, start_instant).await;
}
