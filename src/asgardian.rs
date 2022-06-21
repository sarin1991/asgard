use crate::transport::TransportChannel;
use crate::role::{Leader,Follower,Candidate,RoleData,RoleFlag};
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use crate::transport::Address;
use tokio;
use tokio::sync::mpsc::{Sender,Receiver};
use std::collections::BinaryHeap;
use std::sync::mpsc::SendError;
use std::time::{Instant,Duration};
use crate::asgard_error::AsgardError;

struct AsgardData {
    term:u64,
    latest_log_index:u64,
    commit_index:u64,
    transport_channel:TransportChannel,
}
impl AsgardData {
    fn new(transport_channel:TransportChannel)->Self{
        Self {
            term:0,
            latest_log_index:0,
            commit_index:0,
            transport_channel,
        }
    }
}

struct Asgardian {
    role_flag: RoleFlag,
    asgard_data: AsgardData,
    role_data: RoleData,
}
impl Asgardian{
    fn new(transport_channel:TransportChannel)->Self{
        Self {
            role_flag: RoleFlag::CandidateFlag,
            asgard_data: AsgardData::new(transport_channel),
            role_data: RoleData::new(),
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
    fn handle_asgardian_message(&mut self,asgardian_message:AsgardianMessage,sender:Address)->Result<bool,AsgardError>{
        let break_flag = match self.role {
            Role::Candidate(_) => ;
            Role::Exile(_) => ;
            Role::Follower(_) => ;
            Role::Immigrant(_) => ;
            Role::Leader(_) => ;
        };
        Ok(break_flag)
        panic!("Unimplemented!");
    }
    fn handle_api_message(&mut self,api_message:APIMessage,sender:Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    } 
    async fn start(&mut self)->Result<(),AsgardError>{
        let task1 = tokio::spawn(Asgardian::asgard_election_timer(self.transport_channel.inbound_message_sender.clone()));
        let task2 = tokio::spawn(Asgardian::asgard_message_timer(self.transport_channel.inbound_message_sender.clone()));
        loop{
            let (message,address) = self.transport_channel.inbound_message_receiver.recv().await.unwrap();
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