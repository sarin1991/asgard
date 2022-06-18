use crate::transport::TransportChannel;
use crate::role::{Leader,Follower,Candidate,Role};
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use crate::transport::Address;
use tokio::task;
use tokio::sync::mpsc::{Sender,Receiver};
use std::collections::BinaryHeap;
use std::sync::mpsc::SendError;
use std::time::{Instant,Duration};
use crate::asgard_error::AsgardError;

struct Asgardian {
    term:u64,
    latest_log_index:u64,
    commit_index:u64,
    transport_channel:TransportChannel,
    role: Role,
}

impl Asgardian{
    fn new(transport_channel:TransportChannel)->Self{
        Self {
            term:0,
            latest_log_index:0,
            commit_index:0,
            transport_channel,
            role:Role::new(),
        }
    }
    async fn asgard_message_timer(inbound_message_sender:Sender<(Message,Address)>)->Result<(),tokio::sync::mpsc::error::SendError<(Message,String)>>{
        let mut interval = tokio::time::interval(Duration::from_millis(30));
        let asgard_message_timer = AsgardMessageTimer{};
        let asgardian_message = AsgardianMessage::AsgardMessageTimer(asgard_message_timer);
        let message = Message::AsgardianMessage(asgardian_message);
        loop {
            interval.tick().await;
            inbound_message_sender.send((message.clone(),"local".to_owned())).await?;
        }
    }
    async fn asgard_election_timer(inbound_message_sender:Sender<(Message,Address)>)->Result<(),AsgardError>{
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        let asgard_election_timer = AsgardElectionTimer{};
        let asgardian_message = AsgardianMessage::AsgardElectionTimer(asgard_election_timer);
        let message = Message::AsgardianMessage(asgardian_message);
        loop {
            interval.tick().await;
            inbound_message_sender.send((message.clone(),"local".to_owned())).await?;
        }
    }
    fn handle_asgardian_message(&mut self,asgardian_message:AsgardianMessage,sender:Address){
        panic!("Unimplemented!");
    }
    fn handle_api_message(&mut self,api_message:APIMessage,sender:Address){
        panic!("Unimplemented!");
    } 
    async fn start(&mut self)->Result<(),AsgardError>{
        let task1 = tokio::spawn(Asgardian::asgard_election_timer(self.transport_channel.inbound_message_sender.clone()));
        let task2 = tokio::spawn(Asgardian::asgard_message_timer(self.transport_channel.inbound_message_sender.clone()));
        loop{
            let (message,address) = self.transport_channel.inbound_message_receiver.recv().await.unwrap();
            match message {
                Message::AsgardianMessage(asgardian_message) => self.handle_asgardian_message(asgardian_message, address),
                Message::APIMessage(api_message) => self.handle_api_message(api_message, address),
            }
        }
        task1.await??;
        task2.await??;
    }
}