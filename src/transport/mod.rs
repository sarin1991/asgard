use tokio::sync::mpsc::{Sender,Receiver};
use tokio::task;
use crate::messages::{APIMessage,AsgardianMessage,Message};

#[derive(Debug)]
pub enum Address{
    IP(String),
    Broadcast,
    Local,
}

pub(crate) struct TransportChannel {
    pub outbound_asgardian_message_sender: Sender<(AsgardianMessage,Address)>,
    pub outbound_client_message_sender: Sender<(APIMessage,Address)>,
    pub inbound_message_receiver: Receiver<(Message,Address)>,
    pub inbound_message_sender: Sender<(Message,Address)>,
}
impl TransportChannel{
    pub fn new(outbound_asgardian_message_sender: Sender<(AsgardianMessage,Address)>,
                outbound_client_message_sender: Sender<(APIMessage,Address)>,
                inbound_message_receiver: Receiver<(Message,Address)>,
                inbound_message_sender: Sender<(Message,Address)>)->Self{
        Self { 
            outbound_asgardian_message_sender, 
            outbound_client_message_sender, 
            inbound_message_receiver,
            inbound_message_sender
        }
    }
}

pub trait AsgardTransport {
    type Context;
    fn new(outbound_asgardian_message_receiver:Receiver<(AsgardianMessage,Address)>,
            inbound_message_sender:Sender<(Message,Address)>)->Self;
    fn update_context(context:Self::Context);
    fn run(self);
}

pub trait ClientTransport {
    type Context;
    fn new(outbound_client_message_receiver:Receiver<(APIMessage,Address)>,
            inbound_message_sender:Sender<(Message,Address)>)->Self;
    fn update_context(context:Self::Context);
    fn run(self);
}

pub(crate) struct Transport<A:AsgardTransport+Send+'static,C:ClientTransport+Send+'static>{
    client_transport:C,
    asgard_transport:A,
    transport_channel: Option<TransportChannel>,
    peers: Vec<Address>,
    clients: Vec<Address>,
}

impl <A:AsgardTransport+Send+'static,C:ClientTransport+Send+'static> Transport<A,C> {
    pub(crate) fn new() -> Self {
        let (inbound_message_sender,inbound_message_receiver) = tokio::sync::mpsc::channel::<(Message,Address)>(1024);
        let (outbound_asgardian_message_sender,outbound_asgardian_message_receiver) = tokio::sync::mpsc::channel::<(AsgardianMessage,Address)>(1024);
        let (outbound_client_message_sender,outbound_client_message_receiver) = tokio::sync::mpsc::channel::<(APIMessage,Address)>(1024);
        let transport_channel = TransportChannel::new(outbound_asgardian_message_sender, 
                                                                        outbound_client_message_sender, 
                                                                        inbound_message_receiver,
                                                                        inbound_message_sender.clone());
        Self { 
            client_transport: C::new(outbound_client_message_receiver,inbound_message_sender.clone()), 
            asgard_transport: A::new(outbound_asgardian_message_receiver,inbound_message_sender.clone()),
            transport_channel: Some(transport_channel),
            peers: vec![],
            clients: vec![],
        }
    }
    pub(crate) fn initialize(&mut self)->TransportChannel{
        self.transport_channel.take().unwrap()
    }
    pub(crate) fn add_peer(&mut self,peer:Address){
        self.peers.push(peer);
    }
    pub(crate) fn add_client(&mut self,client:Address){
        self.clients.push(client);
    }
    pub(crate) async fn run(self){
        let Transport{client_transport,
                      asgard_transport,
                      ..} = self;
        let client_transport_task = task::spawn_blocking({
            move || {client_transport.run();}
        });
        let asgard_transport_task = task::spawn_blocking({
            move || {asgard_transport.run();}
        });
        let _ = client_transport_task.await;
        let _ = asgard_transport_task.await;
    }
}