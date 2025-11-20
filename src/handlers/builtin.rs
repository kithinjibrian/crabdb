use base64;
use rand_core::OsRng;
use serde_json::json;
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::{pincer::message::Message, server::tcp::KeyStore};

pub async fn handle_ping(
    mut req: Message,
    peer: std::net::SocketAddr,
    key_store: &KeyStore,
) -> Message {
    req.typ = "response".to_string();
    req.status = Some("ok".to_string());
    req.payload = json!({"msg":"pong"});
    req
}

pub async fn handle_echo(
    mut req: Message,
    peer: std::net::SocketAddr,
    key_store: &KeyStore,
) -> Message {
    req.typ = "response".to_string();
    req.status = Some("ok".to_string());
    req
}

pub async fn handle_dh_init(
    mut req: Message,
    peer: std::net::SocketAddr,
    key_store: &KeyStore,
) -> Message {
    req.typ = "dh_response".to_string();
    req.status = Some("ok".to_string());

    let server_secret = EphemeralSecret::random_from_rng(OsRng);
    let server_public: PublicKey = (&server_secret).into();

    let client_public_base64 = req.payload["A"].as_str().unwrap();

    let client_public_bytes = base64::decode(client_public_base64)
        .expect("Failed to decode client public key from Base64");

    let client_public = PublicKey::from(*arrayref::array_ref![client_public_bytes, 0, 32]);

    let shared_secret = server_secret.diffie_hellman(&client_public);

    let mut store = key_store.lock().await;
    store.insert(peer, shared_secret);

    let server_public_base64 = base64::encode(server_public.as_bytes());

    req.payload = json!({
        "B": server_public_base64
    });

    req
}

pub async fn handle_unknown(
    mut req: Message,
    peer: std::net::SocketAddr,
    key_store: &KeyStore,
) -> Message {
    req.typ = "response".to_string();
    req.status = Some("error".to_string());
    req.payload = json!({"error":"unknown type"});
    req
}
