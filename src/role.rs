use std::collections::{VecDeque, HashMap};
use std::net::SocketAddr;

use crate::asgard_data::{AsgardData, self};
use crate::asgard_error::{AsgardError,InconsistentRoleError,UnknownPeerError, UnexpectedAddressVariantError};
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use crate::protobuf_messages::asgard_messages::AsgardLogMessage;
use crate::protobuf_messages::asgard_messages::{LeaderSync,LeaderHeartbeat,
    VoteResponse,VoteRequest,RebellionResponse,RebellionRequest,FollowerUpdate,AddEntry};
use crate::transport::{TransportChannel,Address};

struct PeerVote {
    address:SocketAddr,
    received_vote:bool,
}
impl PeerVote {
    fn new(address:SocketAddr) -> Self {
        Self { 
            address,
            received_vote:false,
        }
    }
    fn set_vote(&mut self) {
        self.received_vote = true;
    }
}

struct VoteCounter {
    node_vote_map: HashMap<SocketAddr,PeerVote>,
}
impl VoteCounter {
    fn new(asgard_data: &AsgardData) -> Result<Self,AsgardError> {
        let peers = asgard_data.get_active_peers()?;
        let mut node_vote_map:HashMap<SocketAddr,PeerVote> = HashMap::new();
        for peer in peers {
            node_vote_map.insert(peer.clone(),PeerVote::new(peer));
        }
        //add self
        node_vote_map.insert(asgard_data.address.clone(),PeerVote::new(asgard_data.address.clone()));
        Ok(Self { 
            node_vote_map
        })
    }
    fn got_majority(&self)->bool{
        let mut total = 0u32;
        let mut votes_granted = 0u32;
        for (node,node_vote) in self.node_vote_map.iter() {
            if node_vote.received_vote {
                votes_granted = votes_granted+1;
            }
            total=total+1;
        }
        if (votes_granted as f64) > (0.5 *total as f64) {
            true
        }
        else {
            false
        }
    }
    fn add_vote(&mut self, peer: SocketAddr) -> Result<(),AsgardError>{
        let peer_vote_option = self.node_vote_map.get_mut(&peer);
        match peer_vote_option {
            Some(peer_vote) => Ok(peer_vote.set_vote()),
            None => Err(UnknownPeerError::new("Expected peer not found while adding vote".to_owned(), peer))?,
        }
    }
}

pub(crate)  struct Rebel{
    leader_timeout:u32,
    vote_counter: VoteCounter,
}
impl Rebel {
    fn new(asgard_data: &AsgardData) -> Result<Self,AsgardError> {
        Ok(Self {
            leader_timeout:0,
            vote_counter:VoteCounter::new(asgard_data)?,
        })
    }
    fn is_rebel(&self) -> bool{
        self.leader_timeout>1
    }
    fn increment_leader_timeout(&mut self) {
        self.leader_timeout = self.leader_timeout+1;
    }
    fn reset_leader_timeout(&mut self) {
        self.leader_timeout = 0;
    }
    fn add_rebel(&mut self,address:SocketAddr) -> Result<(),AsgardError> {
        self.vote_counter.add_vote(address)
    }
    fn is_rebellion_success(&self) -> bool {
        self.vote_counter.got_majority()
    }
}

struct FollowerInfo {
    uncommitted_log_index: u64,
    committed_log_index: u64,
    initialization_flag: bool,
    socket_address: SocketAddr,
}
impl FollowerInfo {
    fn new(initial_uncommitted_log_index:u64,initial_committed_log_index:u64,socket_address:SocketAddr) -> Self {
        Self{
            uncommitted_log_index:initial_uncommitted_log_index,
            committed_log_index:initial_committed_log_index,
            initialization_flag:false,
            socket_address,
        }
    }
    fn update_uncommitted_log_index(&mut self,log_index:u64) -> Result<(),AsgardError>{
        if log_index>self.uncommitted_log_index {
            self.uncommitted_log_index = log_index;
        }
        else {
            println!("Index not updated because given log index is lower than old one!");
        }
        Ok(())
    }
}

pub(crate) struct Leader{
    follower_info_hash_map: HashMap<SocketAddr,FollowerInfo>,
}

impl Leader {
    fn new(asgard_data: &AsgardData) -> Result<Self,AsgardError> {
        let peers = asgard_data.get_active_peers()?;
        let mut follower_info_hash_map:HashMap<SocketAddr,FollowerInfo> = HashMap::new();
        let initial_uncommitted_log_index = asgard_data.get_last_log_index();
        let initial_committed_log_index = asgard_data.committed_log.get_last_log_index();
        for peer in peers {
            follower_info_hash_map.insert(peer,FollowerInfo::new(initial_uncommitted_log_index,initial_committed_log_index,peer));
        }
        //add self
        follower_info_hash_map.insert(asgard_data.address.clone(),FollowerInfo::new(initial_uncommitted_log_index,initial_committed_log_index,asgard_data.address.clone()));
        Ok(Self { 
            follower_info_hash_map
        })
    }
    fn get_commit_safe_index(&self) -> u64{
        let mut log_indexes = vec![];
        for (node,follower_info) in self.follower_info_hash_map.iter() {
            log_indexes.push(follower_info.uncommitted_log_index);
        }
        log_indexes.sort();
        let mid = log_indexes.len() / 2;
        log_indexes[mid]
    }
    fn update_node_log_index(&mut self,node:SocketAddr,log_index:u64) -> Result<(),AsgardError>{
        let log_index_option = self.follower_info_hash_map.get_mut(&node);
        match log_index_option {
            Some(follower_info) => {
                follower_info.update_uncommitted_log_index(log_index)
            },
            None => Err(UnknownPeerError::new("Expected peer not found while updating node log index".to_owned(),node))?,
        }
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
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
    fn new(leader: Option<Address>,voted_for: Address,asgard_data:&AsgardData) -> Result<Self,AsgardError> {
        Ok(Self {
            leader,
            voted_for,
            initialization_flag:false,
            rebel:Rebel::new(asgard_data)?,
            leader_message_queue: LeaderMessageQueue::new(),
        })
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate) struct Candidate{
    voted_for_self: bool,
    rebel: Rebel,
    vote_counter: VoteCounter,
}
impl Candidate {
    pub(crate) fn new(asgard_data: &mut AsgardData) -> Result<Self,AsgardError> {
        let vote_counter = VoteCounter::new(asgard_data)?;
        Ok(Self {
            voted_for_self: false,
            rebel: Rebel::new(asgard_data)?,
            vote_counter,
        })
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        let break_flag = match asgardian_message {
            AsgardianMessage::LeaderSync(leader_sync) => Candidate::handle_leader_sync(role,asgard_data,leader_sync,sender).await?,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => Candidate::handle_leader_heartbeat(role,asgard_data,leader_heartbeat,sender).await?,
            AsgardianMessage::VoteResponse(vote_response) => Candidate::handle_vote_response(role,asgard_data,vote_response,sender).await?,
            AsgardianMessage::VoteRequest(vote_request) => Candidate::handle_vote_request(role,asgard_data,vote_request,sender).await?,
            AsgardianMessage::RebellionResponse(rebellion_response) => Candidate::handle_rebellion_response(role,asgard_data,rebellion_response,sender).await?,
            AsgardianMessage::RebellionRequest(rebellion_request) => Candidate::handle_rebellion_request(role,asgard_data,rebellion_request,sender).await?,
            AsgardianMessage::FollowerUpdate(follower_update) => Candidate::handle_follower_update(role,asgard_data,follower_update,sender).await?,
            AsgardianMessage::AddEntry(add_entry) => Candidate::handle_add_entry(role,asgard_data,add_entry,sender).await?,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => Candidate::handle_asgard_message_timer(role,asgard_data,asgard_message_timer,sender).await?,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => Candidate::handle_asgard_election_timer(role,asgard_data,asgard_election_timer,sender).await?,
        };
        Ok(break_flag)
    }
    fn get_variant(role: &mut Role) -> Result<&mut Self,AsgardError>{
        let candidate = match role {
            Role::Candidate(candidate) => candidate,
            _ => Err(InconsistentRoleError::new("Candidate".to_owned(),role.get_role_name()))?,
        };
        Ok(candidate)
    }
    fn to_leader(role: &mut Role,asgard_data:&AsgardData) ->Result<(),AsgardError> {
        let leader = Leader::new(asgard_data)?;
        *role = Role::Leader(leader);
        panic!("Not Completed!");
        Ok(())
    }
    fn to_follower(role: &mut Role,leader: Option<Address>,voted_for: Address,asgard_data:&AsgardData) -> Result<(),AsgardError> {
        let follower = Follower::new(leader,voted_for,asgard_data)?;
        *role = Role::Follower(follower);
        Ok(())
    }
    async fn handle_leader_sync(role: &mut Role,asgard_data: &mut AsgardData,leader_sync: LeaderSync,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        let voted_for = match candidate.voted_for_self {
            true => Address::IP(asgard_data.address.clone()),
            false => sender.clone(),
        };
        Candidate::to_follower(role, Some(sender.clone()), voted_for, asgard_data)?;
        let message = Message::AsgardianMessage(AsgardianMessage::LeaderSync(leader_sync));
        asgard_data.repeat_message(message, sender).await?;
        Ok(false)
    }
    async fn handle_leader_heartbeat(role: &mut Role,asgard_data: &mut AsgardData,leader_heartbeat: LeaderHeartbeat,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        let voted_for = match candidate.voted_for_self {
            true => Address::IP(asgard_data.address.clone()),
            false => sender.clone(),
        };
        Candidate::to_follower(role, Some(sender.clone()), voted_for, asgard_data)?;
        let message = Message::AsgardianMessage(AsgardianMessage::LeaderHeartbeat(leader_heartbeat));
        asgard_data.repeat_message(message, sender).await?;
        Ok(false)
    }
    async fn handle_vote_response(role: &mut Role,asgard_data: &mut AsgardData,vote_response: VoteResponse,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        match sender {
            Address::IP(socket_address) => candidate.vote_counter.add_vote(socket_address)?,
            Address::Local => candidate.vote_counter.add_vote(asgard_data.address.clone())?,
            Address::Broadcast => Err(UnexpectedAddressVariantError::new("IP or Local".to_owned(),"Broadcast".to_owned()))?,
        };
        if candidate.vote_counter.got_majority() {
            //Candidate is now leader
            Candidate::to_leader(role,asgard_data)?;
        }
        Ok(false)
    }
    async fn handle_vote_request(role: &mut Role,asgard_data: &mut AsgardData,vote_request: VoteRequest,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        let mut vote_response = VoteResponse::default();
        vote_response.candidate_id = sender.to_string();
        vote_response.term = asgard_data.term;
        let message = AsgardianMessage::VoteResponse(vote_response);
        if !candidate.voted_for_self {
            //Candidate hasn't voted for self yet so is open to agreeing to new leader
            if vote_request.last_log_index_term>asgard_data.get_last_log_index_term() {
                //Requester has log messages with higher term so accept his request to be leader
                asgard_data.send_asgardian_message(message, sender.clone()).await?;
                //Switch to follower since we voted for another node. So no longer a candidate
                Candidate::to_follower(role, Some(sender.clone()), sender.clone(), asgard_data)?;
            }
            else if vote_request.last_log_index_term==asgard_data.get_last_log_index_term(){
                if vote_request.last_log_index>=asgard_data.get_last_log_index() {
                    //Requester has log messages with same term as node but log index is as high or higher than node
                    //so accept his request to be leader
                    asgard_data.send_asgardian_message(message, sender.clone()).await?;
                    //Switch to follower since we voted for another node. So no longer a candidate
                    Candidate::to_follower(role, Some(sender.clone()), sender.clone(), asgard_data)?;
                }
            }
        }
        Ok(false)
    }
    async fn handle_rebellion_response(role: &mut Role,asgard_data: &mut AsgardData,rebellion_response: RebellionResponse,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        if candidate.rebel.is_rebel() & rebellion_response.vote_granted {
            //Only taking action if node is rebel
            let rebel = &mut candidate.rebel;
            match sender {
                Address::IP(socket_address) => rebel.add_rebel(socket_address)?,
                Address::Local => rebel.add_rebel(asgard_data.address.clone())?,
                Address::Broadcast => Err(UnexpectedAddressVariantError::new("IP or Local".to_owned(),"Broadcast".to_owned()))?,
            };
            if rebel.is_rebellion_success(){
                //Rebellion succeeded so can increment term
                Role::increment_term(role, asgard_data).await?;
            }
        }
        Ok(false)
    }
    async fn handle_rebellion_request(role: &mut Role,asgard_data: &mut AsgardData,rebellion_request: RebellionRequest,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        if candidate.rebel.is_rebel() {
            let mut rebellion_response = RebellionResponse::default();
            rebellion_response.term = asgard_data.term;
            rebellion_response.vote_granted = true;
            rebellion_response.peer_id = asgard_data.address.to_string();
            let asgardian_message = AsgardianMessage::RebellionResponse(rebellion_response);
            asgard_data.send_asgardian_message(asgardian_message, sender).await?;
        }
        Ok(false)
    }
    async fn handle_follower_update(role: &mut Role,asgard_data: &mut AsgardData,follower_update: FollowerUpdate,sender: Address)->Result<bool,AsgardError>{
        unreachable!("Candidate received follower update message. This should not happen as only leaders can receive this message!");
    }
    async fn handle_add_entry(role: &mut Role,asgard_data: &mut AsgardData,add_entry: AddEntry,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        let voted_for = match candidate.voted_for_self {
            true => Address::IP(asgard_data.address.clone()),
            false => sender.clone(),
        };
        Candidate::to_follower(role, Some(sender.clone()), voted_for, asgard_data)?;
        let message = Message::AsgardianMessage(AsgardianMessage::AddEntry(add_entry));
        asgard_data.repeat_message(message, sender).await?;
        Ok(false)
    }
    async fn handle_asgard_message_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_message_timer: AsgardMessageTimer,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        //Since node is still candidate means it has already voted for self or hasn't voted for anyone else
        candidate.voted_for_self = true;
        //Send vote requests to other nodes
        let mut vote_request = VoteRequest::default();
        vote_request.term = asgard_data.term;
        vote_request.candidate_id = asgard_data.address.to_string();
        vote_request.last_log_index = asgard_data.get_last_log_index();
        vote_request.last_log_index_term = asgard_data.get_last_log_index_term();
        let message = AsgardianMessage::VoteRequest(vote_request);
        let peers = asgard_data.get_active_peers()?;
        for peer in peers.iter() {
            asgard_data.send_asgardian_message(message.clone(), Address::IP(peer.clone())).await?;
        }
        //Send rebellion requests if rebel
        if candidate.rebel.is_rebel() {
            let mut rebellion_request = RebellionRequest::default();
            rebellion_request.candidate_id = asgard_data.address.to_string();
            rebellion_request.term = asgard_data.term;
            let rebellion_message = AsgardianMessage::RebellionRequest(rebellion_request);
            for peer in peers {
                asgard_data.send_asgardian_message(rebellion_message.clone(), Address::IP(peer)).await?;
            }
            if candidate.rebel.is_rebellion_success() {
                //Rebellion succeeded increment term
                Role::increment_term(role, asgard_data).await?;
            }
        }
        Ok(false)
    }
    async fn handle_asgard_election_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_election_timer: AsgardElectionTimer,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        candidate.rebel.increment_leader_timeout();
        Ok(false)
    }
}

pub(crate)  struct Exile{

}
impl Exile {
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
}

pub(crate)  struct Immigrant{

}

impl Immigrant {
    pub(crate) fn new() -> Self {
        Self{}
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
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
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message:AsgardianMessage,sender:Address)->Result<bool,AsgardError>{
        let break_flag = match &role {
            Role::Leader(_) => Leader::handle_asgardian_message(role,asgard_data,asgardian_message,sender).await?,
            Role::Follower(_) => Follower::handle_asgardian_message( role,asgard_data,asgardian_message,sender).await?,
            Role::Candidate(_) => Candidate::handle_asgardian_message( role,asgard_data,asgardian_message,sender).await?,
            Role::Immigrant(_) => Immigrant::handle_asgardian_message(role,asgard_data,asgardian_message,sender).await?,
            Role::Exile(_) => Exile::handle_asgardian_message(role,asgard_data,asgardian_message,sender).await?,
        };
        Ok(break_flag)
    }
    pub(crate) async fn increment_term(role: &mut Role,asgard_data: &mut AsgardData) -> Result<(),AsgardError> {
        panic!("Unimplemented!");
    }
}