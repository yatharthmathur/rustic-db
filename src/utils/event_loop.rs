use log::{error, info};
use std::{sync::Arc, time::Duration};

use tokio::{
    net::TcpListener,
    sync::RwLock,
    time::{timeout, Instant},
};

use crate::managers::manager::RusticManager;
use crate::utils::request_handler::handle_client_request;

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

pub async fn event_loop(
    listener: TcpListener,
    shared_manager: Arc<RwLock<RusticManager>>,
    start_instant: Instant,
) {
    loop {
        // Accept incoming connections for 5 seconds.
        match timeout(Duration::from_secs(5), listener.accept()).await {
            Ok(Ok((socket, _))) => {
                // Spawn a new task to handle the client
                tokio::spawn(handle_client_request(
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
