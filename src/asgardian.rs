use crate::transport::{TransportChannel,Address};
use crate::role::{Leader,Follower,Candidate,Immigrant,Exile,Role};
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use tokio;
use tokio::sync::mpsc::{Sender,Receiver};
use std::collections::BinaryHeap;
use std::sync::mpsc::SendError;
use std::time::{Instant,Duration};
use crate::asgard_error::AsgardError;
use crate::asgard_data::AsgardData;

struct Asgardian {
    asgard_data: AsgardData,
    role: Role,
}
impl Asgardian{
    fn new(transport_channel:TransportChannel)->Self{
        Self {
            asgard_data: AsgardData::new(transport_channel),
            role: Role::new(),
        }
    }
    async fn asgard_message_timer(inbound_message_sender:Sender<(Message,Address)>)->Result<(),AsgardError>{
        let mut interval = tokio::time::interval(Duration::from_millis(30));
        let asgard_message_timer = AsgardMessageTimer{};
        let asgardian_message = AsgardianMessage::AsgardMessageTimer(asgard_message_timer);
        let message = Message::AsgardianMessage(asgardian_message);
        loop {
            interval.tick().await;
            inbound_message_sender.send((message.clone(),Address::Local)).await?;
        }
    }
    async fn asgard_election_timer(inbound_message_sender:Sender<(Message,Address)>)->Result<(),AsgardError>{
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        let asgard_election_timer = AsgardElectionTimer{};
        let asgardian_message = AsgardianMessage::AsgardElectionTimer(asgard_election_timer);
        let message = Message::AsgardianMessage(asgardian_message);
        loop {
            interval.tick().await;
            inbound_message_sender.send((message.clone(),Address::Local)).await?;
        }
    }
    fn increment_term(&mut self){
        panic!("unimplemented!");
    }
    fn handle_asgardian_message(&mut self,asgardian_message:AsgardianMessage,sender:Address)->Result<bool,AsgardError>{
        if asgardian_message.is_lower_term(self.asgard_data.term){
            return Ok(false);
        }
        if asgardian_message.is_higher_term(self.asgard_data.term){
            self.increment_term();
        }
        let break_flag = Role::handle_asgardian_message(&mut self.role,&mut self.asgard_data,asgardian_message,sender)?;
        Ok(break_flag)
    }
    fn handle_api_message(&mut self,api_message:APIMessage,sender:Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    } 
    async fn start(&mut self)->Result<(),AsgardError>{
        let task1 = tokio::spawn(Asgardian::asgard_election_timer(self.asgard_data.transport_channel.inbound_message_sender.clone()));
        let task2 = tokio::spawn(Asgardian::asgard_message_timer(self.asgard_data.transport_channel.inbound_message_sender.clone()));
        loop{
            let (message,address) = self.asgard_data.transport_channel.inbound_message_receiver.recv().await.unwrap();
            let break_flag = match message {
                Message::AsgardianMessage(asgardian_message) => self.handle_asgardian_message(asgardian_message, address)?,
                Message::APIMessage(api_message) => self.handle_api_message(api_message, address)?,
            };
            if !break_flag {
                break;
            }
        }
        task1.await??;
        task2.await??;
        Ok(())
    }
}