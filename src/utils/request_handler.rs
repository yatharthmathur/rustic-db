use crate::managers::manager::RusticManager;
use log::info;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::RwLock,
    time::Instant,
};

pub(super) async fn handle_client_request(
    mut tcp_stream: TcpStream,
    shared_manager: Arc<RwLock<RusticManager>>,
    start_instant: Instant,
) {
    // Your client handling logic goes here
    // TODO: Add all logic to handle client requests here.
    // This is only temporary to test the Sockets and demonstrate the logic of handling requests.

    // This is how we can share the manager and store for the case where we just want to read.
    // NOTE: write_manager should only invoked if we want to write to the store.
    let mut write_manager = shared_manager.write().await;
    let store = write_manager.get_store_mut("default_store").unwrap();

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

    let value = store.get_string("ABC");
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
            "ABC",
            start_instant.elapsed().as_secs().to_string() + " " + string_buff.as_str(),
            Some(5000),
        );

        return_buff = "Cache is set".as_bytes();
        let res = tcp_stream.write(return_buff).await;
        info!("Response sent: {:?}", res.unwrap());
    }
}
