pub mod codec;
pub mod message;

pub use codec::read_frame;
pub use codec::write_frame;

use message::Message as Msg;

use crate::handlers::builtin;
use crate::server::tcp::KeyStore;

pub async fn handle_message(msg: Msg, peer: std::net::SocketAddr, key_store: &KeyStore) -> Msg {
    match msg.typ.as_str() {
        "dh_init" => builtin::handle_dh_init(msg, peer, key_store).await,
        "ping" => builtin::handle_ping(msg, peer, key_store).await,
        "echo" => builtin::handle_echo(msg, peer, key_store).await,
        _ => builtin::handle_unknown(msg, peer, key_store).await,
    }
}
