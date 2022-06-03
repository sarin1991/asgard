use tokio::sync::mpsc::{Sender,Receiver};
use tokio::task;
use crate::messages::{APIMessage,AsgardianMessage,Message};

type Address=String;

pub struct TransportChannel {
    client_message_sender: Sender<(APIMessage,Address)>,
    asgardian_message_sender: Sender<(AsgardianMessage,Address)>,
    message_receiver: Receiver<Message>,
}

pub trait AsgardTransport {
    type Context;
    fn new(inbound_sender:Sender<Message>)->Self;
    fn initialize(&mut self)->Sender<(AsgardianMessage,Address)>;
    fn update_context(context:Self::Context);
    fn broadcast_message(msg:AsgardianMessage);
    fn send_message(address:Address,msg:AsgardianMessage);
    fn run(self);
}

pub trait ClientTransport {
    type Context;
    fn new(inbound_sender:Sender<Message>)->Self;
    fn initialize(&mut self)->Sender<(APIMessage,Address)>;
    fn update_context(context:Self::Context);
    fn send_message(address:Address,msg:APIMessage);
    fn run(self);
}

pub struct Transport<A:AsgardTransport+Send+'static,C:ClientTransport+Send+'static>{
    client_transport:C,
    asgard_transport:A,
    outbound_sender: Option<Sender<Message>>,
    inbound_receiver: Receiver<Message>,
    peers: Vec<Address>,
    clients: Vec<Address>,
}

impl <A:AsgardTransport+Send,C:ClientTransport+Send> Transport<A,C> {
    pub fn new() -> Self {
        let (tx,rx) = tokio::sync::mpsc::channel::<Message>(1024);
        Self { 
            client_transport: C::new(tx.clone()), 
            asgard_transport: A::new(tx.clone()),
            outbound_sender: None,
            inbound_receiver: rx,
            peers: vec![],
            clients: vec![],
        }
    }
    pub fn initialize(&mut self)->TransportChannel{
        let (tx,rx) = tokio::sync::mpsc::channel::<Message>(1024);
        self.outbound_sender = Some(tx);
        let client_sender:Sender<(APIMessage, String)> =self.client_transport.initialize();
        let asgardian_sender: Sender<(AsgardianMessage, String)> = self.asgard_transport.initialize();
        TransportChannel { 
            client_message_sender: client_sender, 
            asgardian_message_sender: asgardian_sender, 
            message_receiver: rx 
        }
    }
    pub fn addPeer(&mut self,peer:Address){
        self.peers.push(peer);
    }
    pub fn addClient(&mut self,client:Address){
        self.clients.push(client);
    }
    pub async fn run(self){
        let Transport{client_transport,
                      asgard_transport,
                      ..} = self;
        let client_transport_task = task::spawn_blocking({
            move || {client_transport.run();}
        });
        let asgard_transport_task = task::spawn_blocking({
            move || {asgard_transport.run();}
        });
        client_transport_task.await;
        asgard_transport_task.await;
    }
}