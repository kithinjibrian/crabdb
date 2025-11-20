use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use crate::pincer::{handle_message, read_frame, write_frame};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use x25519_dalek::SharedSecret;

pub type KeyStore = Arc<Mutex<HashMap<SocketAddr, SharedSecret>>>;

pub async fn run(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let key_store: KeyStore = Arc::new(Mutex::new(HashMap::new()));

    println!("Listening on {}", addr);

    loop {
        let (mut socket, peer) = listener.accept().await?;
        println!("Accepted connection from {}", peer);

        let key_store_clone = key_store.clone();

        tokio::spawn(async move {
            loop {
                match read_frame(&mut socket).await {
                    Ok(message) => {
                        let response = handle_message(message, peer, &key_store_clone).await;

                        if let Err(e) = write_frame(&mut socket, &response).await {
                            eprintln!("Write error to {}: {:?}", peer, e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Connection {} closed/error: {:?}", peer, e);
                        return;
                    }
                }
            }
        });
    }
}
