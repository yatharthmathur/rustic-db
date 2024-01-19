mod managers;
mod stores;

use managers::manager::RusticManager;
use std::sync::Arc;
use stores::store::KeyValueStore;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
    time::Instant,
};

async fn handle_client(
    mut tcp_stream: TcpStream,
    write_manager: Arc<RwLock<RusticManager>>,
    start_instant: Instant,
) {
    // Your client handling logic goes here
    let mut locked_writer = write_manager.write().await;
    let store = locked_writer
        .get_store_mut("default_store".to_owned())
        .unwrap();

    println!(
        "Handling client from: {:?}",
        tcp_stream.peer_addr().unwrap()
    );
    let buf: &mut Vec<u8> = &mut vec![];
    println!(
        "Message received: {:?}",
        tcp_stream.read_buf(buf).await.unwrap()
    );
    let string_buff: String = String::from_utf8_lossy(buf).to_string();
    println!("Message body: {:?}", string_buff);

    let value = store.get_string("ABC".to_owned());
    if value.is_some() {
        println!("Cache value is present {:?}", value.unwrap().unwrap());
    } else {
        store.set_string(
            "ABC".to_owned(),
            start_instant.elapsed().as_secs().to_string() + " " + string_buff.as_str(),
            Some(5000),
        );
    }

    let return_buff: &mut &[u8] = &mut "Hello, rustics! 😜".as_bytes();
    let res = tcp_stream.write_buf(return_buff).await;
    println!("Response sent: {:?}", res.unwrap());
}

async fn event_loop() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:29998").await.unwrap();
    let shared_write_manager = Arc::new(RwLock::new(RusticManager::new()));
    shared_write_manager
        .write()
        .await
        .create_store("default_store".to_owned(), None);
    let start_instant = Instant::now();
    loop {
        // Accept incoming connections
        match listener.accept().await {
            Ok((socket, _)) => {
                // Spawn a new task to handle the client
                tokio::spawn(handle_client(
                    socket,
                    Arc::clone(&shared_write_manager),
                    start_instant,
                ));
            }
            Err(err) => eprintln!("Error accepting connection: {:?}", err),
        }
    }
}

#[tokio::main]
async fn main() {
    // Start the event loop
    println!("Hello, rustics!");
    event_loop().await;
}
