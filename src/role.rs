use crate::asgard_data::AsgardData;
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use crate::transport::{TransportChannel,Address};

pub(crate)  struct Rebel{

}

pub(crate) struct Leader{

}

impl Leader {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->bool{
        panic!("Unimplemented!");
    }
}

pub(crate) struct Follower{
    rebel: Rebel,
}
impl Follower {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->bool{
        panic!("Unimplemented!");
    }
}

pub(crate) struct Candidate{
    rebel: Rebel,
}
impl Candidate {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->bool{
        panic!("Unimplemented!");
    }
}

pub(crate)  struct Exile{

}
impl Exile {
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->bool{
        panic!("Unimplemented!");
    }
}

pub(crate)  struct Immigrant{

}

impl Immigrant {
    pub(crate) fn new() -> Self {
        Self{}
    }
    pub(crate) fn handle_asgardian_message(&mut self,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->bool{
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