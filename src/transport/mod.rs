use tokio::sync::mpsc::{Sender,Receiver};
use message_io::network::{Transport, ToRemoteAddr};

pub trait Transport {
    type Message;
    type Context;
    async fn update_context(context:Context);
    async fn broadcast_message(Message);
    async fn send_message(peer_id:i64,Message);
    async fn receive_message()->(i64,Message);
}