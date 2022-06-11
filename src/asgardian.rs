use crate::transport::TransportChannel;
use crate::role::{Leader,Follower,Candidate,Role};
use crate::messages::{APIMessage,AsgardianMessage,Message};
use crate::transport::Address;
use tokio::task;
use tokio::sync::mpsc::{Sender,Receiver};
use std::collections::BinaryHeap;
use std::time::{Instant,Duration};

struct Timer{
    inbound_message_sender:Sender<(Message,Address)>,
    timer_message_receiver:Receiver<(Message,Duration,Duration)>,
    timer_queue:BinaryHeap<(Message,Instant)>,
}
impl Timer{
    fn set(&mut self,msg:Message,start_duration:Duration,end_duration:Duration){
        
    }
}

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
    async fn timer(inbound_message_sender:Sender<(Message,Address)>){

    }
    fn handle_asgardian_message(&mut self,asgardian_message:AsgardianMessage,sender:Address){
        panic!("Unimplemented!");
    }
    fn handle_api_message(&mut self,api_message:APIMessage,sender:Address){
        panic!("Unimplemented!");
    } 
    async fn start(&mut self){
        let timer_task = task::spawn(Asgardian::timer(self.transport_channel.inbound_message_sender.clone()));
        loop{
            let (message,address) = self.transport_channel.inbound_message_receiver.recv().await.unwrap();
            match message {
                Message::AsgardianMessage(asgardian_message) => self.handle_asgardian_message(asgardian_message, address),
                Message::APIMessage(api_message) => self.handle_api_message(api_message, address),
            }
        }
        timer_task.await;
    }
}