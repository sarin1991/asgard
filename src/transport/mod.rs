pub trait AsgardTransport {
    type Message;
    type Context;
    fn update_context(context:Self::Context);
    fn broadcast_message(msg:Self::Message);
    fn send_message(peer_id:String,msg:Self::Message);
}

pub trait ClientTransport {
    type Message;
    type Context;
    fn update_context(context:Self::Context);
    fn send_message(peer_id:String,msg:Self::Message);
}