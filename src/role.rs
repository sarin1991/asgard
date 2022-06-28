use std::collections::VecDeque;

use crate::asgard_data::AsgardData;
use crate::asgard_error::AsgardError;
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use crate::protobuf_messages::asgard_messages::AsgardLogMessage;
use crate::transport::{TransportChannel,Address};

pub(crate)  struct Rebel{

}

pub(crate) struct Leader{

}

impl Leader {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

struct LeaderMessageQueue{
    messages:VecDeque<Option<AsgardLogMessage>>,
}
impl LeaderMessageQueue{
    fn new()->Self{
        Self{
            messages:VecDeque::<Option<AsgardLogMessage>>::new(),
        }
    }
}
pub(crate) struct Follower{
    initialization_flag: bool,
    rebel: Rebel,
    leader_message_queue: LeaderMessageQueue,
}
impl Follower {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate) struct Candidate{
    rebel: Rebel,
}
impl Candidate {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate)  struct Exile{

}
impl Exile {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate)  struct Immigrant{

}

impl Immigrant {
    pub(crate) fn new() -> Self {
        Self{}
    }
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate) enum Role{
    Leader(Leader),
    Follower(Follower),
    Candidate(Candidate),
    Immigrant(Immigrant),
    Exile(Exile),
}

impl Role {
    pub(crate) fn new() -> Self {
        let immigrant = Immigrant::new();
        Role::Immigrant(immigrant)
    }
}