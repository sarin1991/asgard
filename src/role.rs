use std::collections::VecDeque;

use crate::asgard_data::AsgardData;
use crate::asgard_error::{AsgardError,InconsistentRoleError};
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use crate::protobuf_messages::asgard_messages::AsgardLogMessage;
use crate::protobuf_messages::asgard_messages::{LeaderSync,LeaderHeartbeat,
    VoteResponse,VoteRequest,RebellionResponse,RebellionRequest,FollowerUpdate,AddEntry};
use crate::transport::{TransportChannel,Address};

pub(crate)  struct Rebel{

}
impl Rebel {
    fn new() -> Self {
        Self {  
        }
    }
}

pub(crate) struct Leader{

}

impl Leader {
    fn new() -> Self {
        Self {  

        }
    }
    pub(crate) fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
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
    leader: Option<Address>,
    voted_for: Address,
    initialization_flag: bool,
    rebel: Rebel,
    leader_message_queue: LeaderMessageQueue,
}
impl Follower {
    fn new(leader: Option<Address>,voted_for: Address) -> Self {
        //TODO:Add follower initialization error when both leader and voted for is None. It should be candidate in that case
        Self {
            leader,
            voted_for,
            initialization_flag:false,
            rebel:Rebel::new(),
            leader_message_queue: LeaderMessageQueue::new(),
        }
    }
    pub(crate) fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate) struct Candidate{
    voted_for: Option<Address>,
    rebel: Rebel,
}
impl Candidate {
    pub(crate) fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        let break_flag = match asgardian_message {
            AsgardianMessage::LeaderSync(leader_sync) => Candidate::handle_leader_sync(role,asgard_data,leader_sync,sender)?,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => Candidate::handle_leader_heartbeat(role,asgard_data,leader_heartbeat,sender)?,
            AsgardianMessage::VoteResponse(vote_response) => Candidate::handle_vote_response(role,asgard_data,vote_response,sender)?,
            AsgardianMessage::VoteRequest(vote_request) => Candidate::handle_vote_request(role,asgard_data,vote_request,sender)?,
            AsgardianMessage::RebellionResponse(rebellion_response) => Candidate::handle_rebellion_response(role,asgard_data,rebellion_response,sender)?,
            AsgardianMessage::RebellionRequest(rebellion_request) => Candidate::handle_rebellion_request(role,asgard_data,rebellion_request,sender)?,
            AsgardianMessage::FollowerUpdate(follower_update) => Candidate::handle_follower_update(role,asgard_data,follower_update,sender)?,
            AsgardianMessage::AddEntry(add_entry) => Candidate::handle_add_entry(role,asgard_data,add_entry,sender)?,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => Candidate::handle_asgard_message_timer(role,asgard_data,asgard_message_timer,sender)?,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => Candidate::handle_asgard_election_timer(role,asgard_data,asgard_election_timer,sender)?,
        };
        Ok(break_flag)
    }
    fn handle_leader_sync(role: &mut Role,asgard_data: &mut AsgardData,leader_sync: LeaderSync,sender: Address)->Result<bool,AsgardError>{
        let candidate_voted_for = match role {
            Role::Candidate(candidate) => candidate.voted_for.clone(),
            _ => Err(InconsistentRoleError::new("Candidate".to_owned(),role.get_role_name()))?,
        };
        let voted_for = match candidate_voted_for {
            Some(previous_voted) => previous_voted,
            None => sender.clone(),
        };
        role.to_follower(Some(sender),voted_for)?;
        Ok(false)
    }
    fn handle_leader_heartbeat(role: &mut Role,asgard_data: &mut AsgardData,leader_heartbeat: LeaderHeartbeat,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_vote_response(role: &mut Role,asgard_data: &mut AsgardData,vote_response: VoteResponse,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_vote_request(role: &mut Role,asgard_data: &mut AsgardData,vote_request: VoteRequest,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_rebellion_response(role: &mut Role,asgard_data: &mut AsgardData,rebellion_response: RebellionResponse,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_rebellion_request(role: &mut Role,asgard_data: &mut AsgardData,rebellion_request: RebellionRequest,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_follower_update(role: &mut Role,asgard_data: &mut AsgardData,follower_update: FollowerUpdate,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_add_entry(role: &mut Role,asgard_data: &mut AsgardData,add_entry: AddEntry,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_asgard_message_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_message_timer: AsgardMessageTimer,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    fn handle_asgard_election_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_election_timer: AsgardElectionTimer,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate)  struct Exile{

}
impl Exile {
    pub(crate) fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate)  struct Immigrant{

}

impl Immigrant {
    pub(crate) fn new() -> Self {
        Self{}
    }
    pub(crate) fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
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
    fn get_role_name(&self)->String{
        match self {
            Role::Leader(_) => "Leader".to_owned(),
            Role::Follower(_) => "Follower".to_owned(),
            Role::Candidate(_) => "Candidate".to_owned(),
            Role::Immigrant(_) => "Immigrant".to_owned(),
            Role::Exile(_) => "Exile".to_owned(),
        }
    }
    fn to_leader(&mut self) ->Result<(),AsgardError> {
        let leader = Leader::new();
        *self = Role::Leader(leader);
        Ok(())
    }
    fn to_follower(&mut self,leader: Option<Address>,voted_for: Address) -> Result<(),AsgardError> {
        let follower = Follower::new(leader,voted_for);
        *self = Role::Follower(follower);
        Ok(())
    }
}