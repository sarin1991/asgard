use core::time;
use std::collections::{VecDeque, HashMap};
use std::cmp::{Ordering,Reverse};
use std::net::SocketAddr;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};
use crate::asgard_data::{AsgardData};
use crate::asgard_error::{AsgardError,InconsistentRoleError,UnknownPeerError, UnexpectedAddressVariantError, InconsistentInputsError};
use crate::messages::{APIMessage,AsgardianMessage,Message,AsgardElectionTimer,AsgardMessageTimer};
use crate::protobuf_messages::asgard_messages::{VoteRequest,VoteResponse, RebellionRequest, RebellionResponse, AsgardLogRequest, 
                AsgardLogResponse, LeaderHeartbeat, FollowerUpdate, AsgardLogMessage};
use crate::transport::{TransportChannel,Address};
use crate::common::address_to_socket_address;
use log::info;

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
    fn get_vote(&self) -> bool {
        self.received_vote
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
        let mut self_peer_vote = PeerVote::new(asgard_data.address.clone());
        self_peer_vote.set_vote();
        node_vote_map.insert(asgard_data.address.clone(),self_peer_vote);
        Ok(Self { 
            node_vote_map
        })
    }
    fn got_majority(&self)->bool{
        let mut total = 0u32;
        let mut votes_granted = 0u32;
        for (_node,node_vote) in self.node_vote_map.iter() {
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
    fn get_vote(&self, peer:SocketAddr) -> Result<bool,AsgardError> {
        let peer_vote_option = self.node_vote_map.get(&peer);
        match peer_vote_option {
            Some(peer_vote) => Ok(peer_vote.get_vote()),
            None => Err(UnknownPeerError::new("Expected peer not found while checking for vote".to_owned(), peer))?,
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
    fn reset_leader_timeout(&mut self, asgard_data: &AsgardData) -> Result<(),AsgardError> {
        self.leader_timeout = 0;
        self.vote_counter = VoteCounter::new(asgard_data)?;
        Ok(())
    }
    fn add_rebel(&mut self,address:SocketAddr) -> Result<(),AsgardError> {
        self.vote_counter.add_vote(address)
    }
    fn is_rebellion_success(&self) -> bool {
        self.vote_counter.got_majority()
    }
}

pub(crate) struct LeaderUninitialized {
    vote_counter: VoteCounter,
}
impl LeaderUninitialized {
    fn new(asgard_data:&AsgardData) -> Result<Self,AsgardError> {
        let vote_counter = VoteCounter::new(asgard_data)?;
        Ok(
            Self {
                vote_counter,
            }
        )
    }
    fn get_variant(role: &mut Role) -> Result<&mut Self,AsgardError>{
        let leader_uninitialized = match role {
            Role::LeaderUninitialized(leader_uninitialized) => leader_uninitialized,
            _ => Err(InconsistentRoleError::new("Leader Uninitialized".to_owned(),role.get_role_name()))?,
        };
        Ok(leader_uninitialized)
    }
    fn to_leader(role: &mut Role,asgard_data:&AsgardData) ->Result<(),AsgardError> {
        let leader = Leader::new(asgard_data)?;
        *role = Role::Leader(leader);
        panic!("Not Completed!");
        Ok(())
    }
    async fn handle_vote_request(role: &mut Role,asgard_data: &mut AsgardData,vote_request: VoteRequest,sender: Address)->Result<bool,AsgardError>{
        info!("Got vote request message: {:#?}. Ignoring this message as currently in Leader Unitialized state",vote_request);
        Ok(false)
    }
    async fn handle_vote_response(role: &mut Role,asgard_data: &mut AsgardData,vote_response: VoteResponse,sender: Address)->Result<bool,AsgardError>{
        info!("Got vote response message: {:#?}. Ignoring this message as currently in Leader Unitialized state",vote_response);
        Ok(false)
    }
    async fn handle_rebellion_request(role: &mut Role,asgard_data: &mut AsgardData,rebellion_request: RebellionRequest,sender: Address)->Result<bool,AsgardError>{
        info!("Got rebellion request message: {:#?}. Ignoring this message as currently in Leader Unitialized state",rebellion_request);
        Ok(false)
    }
    async fn handle_rebellion_response(role: &mut Role,asgard_data: &mut AsgardData,rebellion_response: RebellionResponse,sender: Address)->Result<bool,AsgardError>{
        unreachable!("LeaderUninitialized received rebellion response message. This should not happen as only follower or candidates can send out a rebellion request!");
    }
    async fn handle_asgard_log_request(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_request: AsgardLogRequest,sender: Address)->Result<bool,AsgardError> {
        let leader_uninitialized = LeaderUninitialized::get_variant(role)?;
        let mut asgard_log_response = AsgardLogResponse::default();
        asgard_log_response.term = asgard_data.term;
        asgard_log_response.log_commit_index = asgard_data.commit_index;
        asgard_log_response.leader_initialized = false;
        let start_index = asgard_log_request.start_log_index as usize;
        let end_index = asgard_log_request.end_log_index as usize;
        if !(start_index<end_index) {
            Err(InconsistentInputsError::new("Start index is higher or equal to end index in asgard_log_request message".to_owned()))?
        }
        let requested_logs = asgard_data.get_logs(start_index, end_index)?;
        asgard_log_response.message = requested_logs;
        let message = AsgardianMessage::AsgardLogResponse(asgard_log_response);
        let _ = asgard_data.send_asgardian_message(message, sender).await?;
        Ok(false)
    }
    async fn handle_asgard_log_response(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_response: AsgardLogResponse,sender: Address)->Result<bool,AsgardError> {
        unreachable!("LeaderUninitialized received asgard log response message. This should not happen as only follower or candidates can receive this message!");
    }
    async fn handle_leader_heartbeat(role: &mut Role,asgard_data: &mut AsgardData,leader_heartbeat: LeaderHeartbeat,sender: Address)->Result<bool,AsgardError>{
        unreachable!("LeaderUninitialized received leader heartbeat message. This should not happen as only follower or candidates can receive this message!");
    }
    async fn handle_follower_update(role: &mut Role,asgard_data: &mut AsgardData,follower_update: FollowerUpdate,sender: Address)->Result<bool,AsgardError>{
        let leader_uninitialized = LeaderUninitialized::get_variant(role)?;
        let mut majority_initialized_flag = false;
        if follower_update.follower_initialized {
            let socket_address = address_to_socket_address(sender,&asgard_data.address)?;
            leader_uninitialized.vote_counter.add_vote(socket_address)?;
            if leader_uninitialized.vote_counter.got_majority() {
                majority_initialized_flag = true;
            }
        }
        if majority_initialized_flag {
            info!("Got majority of followers that are initialized, so shifting to leader role");
            LeaderUninitialized::to_leader(role, asgard_data)?;
        }
        Ok(false)
    }
    async fn handle_asgard_message_timer(role: &mut Role,asgard_data: &mut AsgardData,_asgard_message_timer: AsgardMessageTimer,_sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_election_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_election_timer: AsgardElectionTimer,sender: Address)->Result<bool,AsgardError>{
        info!("Ignoring asgard election timer, since already in leader uninitialized state. So we are trying to be leader.");
        Ok(false)
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        let break_flag = match asgardian_message {
            AsgardianMessage::VoteRequest(vote_request) => Self::handle_vote_request(role,asgard_data,vote_request,sender).await?,
            AsgardianMessage::VoteResponse(vote_response) => Self::handle_vote_response(role,asgard_data,vote_response,sender).await?,
            AsgardianMessage::RebellionRequest(rebellion_request) => Self::handle_rebellion_request(role,asgard_data,rebellion_request,sender).await?,
            AsgardianMessage::RebellionResponse(rebellion_response) => Self::handle_rebellion_response(role,asgard_data,rebellion_response,sender).await?,
            AsgardianMessage::AsgardLogRequest(asgard_log_request) => Self::handle_asgard_log_request(role,asgard_data,asgard_log_request,sender).await?,
            AsgardianMessage::AsgardLogResponse(asgard_log_response) => Self::handle_asgard_log_response(role,asgard_data,asgard_log_response,sender).await?,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => Self::handle_leader_heartbeat(role,asgard_data,leader_heartbeat,sender).await?,
            AsgardianMessage::FollowerUpdate(follower_update) => Self::handle_follower_update(role,asgard_data,follower_update,sender).await?,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => Self::handle_asgard_message_timer(role,asgard_data,asgard_message_timer,sender).await?,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => Self::handle_asgard_election_timer(role,asgard_data,asgard_election_timer,sender).await?,
        };
        Ok(break_flag)
    }
}

pub(crate) struct FollowerUninitialized {
}
impl FollowerUninitialized {
    fn new(leader: Option<Address>,voted_for: Address,asgard_data:&AsgardData) -> Result<Self,AsgardError> {
        Ok(Self {
        })
    }
    async fn handle_vote_request(role: &mut Role,asgard_data: &mut AsgardData,vote_request: VoteRequest,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_vote_response(role: &mut Role,asgard_data: &mut AsgardData,vote_response: VoteResponse,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_rebellion_request(role: &mut Role,asgard_data: &mut AsgardData,rebellion_request: RebellionRequest,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_rebellion_response(role: &mut Role,asgard_data: &mut AsgardData,rebellion_response: RebellionResponse,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_log_request(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_request: AsgardLogRequest,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
    }
    async fn handle_asgard_log_response(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_response: AsgardLogResponse,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
    }
    async fn handle_leader_heartbeat(role: &mut Role,asgard_data: &mut AsgardData,leader_heartbeat: LeaderHeartbeat,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");   
    }
    async fn handle_follower_update(role: &mut Role,asgard_data: &mut AsgardData,follower_update: FollowerUpdate,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_message_timer(role: &mut Role,asgard_data: &mut AsgardData,_asgard_message_timer: AsgardMessageTimer,_sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_election_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_election_timer: AsgardElectionTimer,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        let break_flag = match asgardian_message {
            AsgardianMessage::VoteRequest(vote_request) => Self::handle_vote_request(role,asgard_data,vote_request,sender).await?,
            AsgardianMessage::VoteResponse(vote_response) => Self::handle_vote_response(role,asgard_data,vote_response,sender).await?,
            AsgardianMessage::RebellionRequest(rebellion_request) => Self::handle_rebellion_request(role,asgard_data,rebellion_request,sender).await?,
            AsgardianMessage::RebellionResponse(rebellion_response) => Self::handle_rebellion_response(role,asgard_data,rebellion_response,sender).await?,
            AsgardianMessage::AsgardLogRequest(asgard_log_request) => Self::handle_asgard_log_request(role,asgard_data,asgard_log_request,sender).await?,
            AsgardianMessage::AsgardLogResponse(asgard_log_response) => Self::handle_asgard_log_response(role,asgard_data,asgard_log_response,sender).await?,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => Self::handle_leader_heartbeat(role,asgard_data,leader_heartbeat,sender).await?,
            AsgardianMessage::FollowerUpdate(follower_update) => Self::handle_follower_update(role,asgard_data,follower_update,sender).await?,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => Self::handle_asgard_message_timer(role,asgard_data,asgard_message_timer,sender).await?,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => Self::handle_asgard_election_timer(role,asgard_data,asgard_election_timer,sender).await?,
        };
        Ok(break_flag)
    }
}
struct FollowerInfo {
    uncommitted_log_index: u64,
    socket_address: SocketAddr,
}
impl FollowerInfo {
    fn new(socket_address:SocketAddr) -> Self {
        Self{
            uncommitted_log_index:0,
            socket_address,
        }
    }
    fn update_uncommitted_log_index(&mut self,log_index:u64) -> (){
        if log_index>self.uncommitted_log_index {
            self.uncommitted_log_index = log_index;
        }
        else {
            info!("Index not updated because given log index is lower than old one!");
        }
    }
}

pub(crate) struct Leader{
    initial_commit_safe_index: u64,
    follower_info_hash_map: HashMap<SocketAddr,FollowerInfo>,
}

impl Leader {
    fn new(asgard_data: &AsgardData) -> Result<Self,AsgardError> {
        let peers = asgard_data.get_active_peers()?;
        let mut follower_info_hash_map:HashMap<SocketAddr,FollowerInfo> = HashMap::new();
        let initial_commit_safe_index = asgard_data.committed_log.get_last_log_index();
        for peer in peers {
            follower_info_hash_map.insert(peer,FollowerInfo::new(peer));
        }
        Ok(Self {
            initial_commit_safe_index,
            follower_info_hash_map
        })
    }
    fn get_commit_safe_index(&self,asgard_data: &AsgardData) -> u64{
        let mut log_indexes = vec![];
        for (_node,follower_info) in self.follower_info_hash_map.iter() {
            log_indexes.push(follower_info.uncommitted_log_index);
        }
        log_indexes.push(asgard_data.get_last_log_index());
        log_indexes.sort();
        let mid = log_indexes.len() / 2;
        std::cmp::max(log_indexes[mid],self.initial_commit_safe_index)
    }
    fn update_node_uncommitted_log_index(&mut self,node:SocketAddr,log_index:u64) -> Result<(),AsgardError>{
        let log_index_option = self.follower_info_hash_map.get_mut(&node);
        match log_index_option {
            Some(follower_info) => {
                follower_info.update_uncommitted_log_index(log_index);
                Ok(())
            },
            None => Err(UnknownPeerError::new("Expected peer not found while updating node log index".to_owned(),node))?,
        }
    }
    fn get_variant(role: &mut Role) -> Result<&mut Self,AsgardError>{
        let leader = match role {
            Role::Leader(leader) => leader,
            _ => Err(InconsistentRoleError::new("Leader".to_owned(),role.get_role_name()))?,
        };
        Ok(leader)
    }
    async fn handle_vote_request(role: &mut Role,asgard_data: &mut AsgardData,vote_request: VoteRequest,sender: Address)->Result<bool,AsgardError>{
        info!("Got vote request message: {:#?}. Ignoring this message as currently in Leader state",vote_request);
        Ok(false)
    }
    async fn handle_vote_response(role: &mut Role,asgard_data: &mut AsgardData,vote_response: VoteResponse,sender: Address)->Result<bool,AsgardError>{
        info!("Got vote response message: {:#?}. Ignoring this message as currently in Leader state",vote_response);
        Ok(false)
    }
    async fn handle_rebellion_request(role: &mut Role,asgard_data: &mut AsgardData,rebellion_request: RebellionRequest,sender: Address)->Result<bool,AsgardError>{
        info!("Got rebellion request message: {:#?}. Ignoring this message as currently in Leader state",rebellion_request);
        Ok(false)
    }
    async fn handle_rebellion_response(role: &mut Role,asgard_data: &mut AsgardData,rebellion_response: RebellionResponse,sender: Address)->Result<bool,AsgardError>{
        unreachable!("Leader received rebellion response message. This should not happen as only follower or candidates can send out a rebellion request!");
    }
    async fn handle_asgard_log_request(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_request: AsgardLogRequest,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
    }
    async fn handle_asgard_log_response(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_response: AsgardLogResponse,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
    }
    async fn handle_leader_heartbeat(role: &mut Role,asgard_data: &mut AsgardData,leader_heartbeat: LeaderHeartbeat,sender: Address)->Result<bool,AsgardError>{
        unreachable!("LeaderUninitialized received leader heartbeat message. This should not happen as only follower or candidates can receive this message!");
    }
    async fn handle_follower_update(role: &mut Role,asgard_data: &mut AsgardData,follower_update: FollowerUpdate,sender: Address)->Result<bool,AsgardError>{
        let leader = Leader::get_variant(role)?;
        let follower_socker_address = address_to_socket_address(sender,&asgard_data.address)?;
        leader.update_node_uncommitted_log_index(follower_socker_address, follower_update.log_index)?;
        Ok(false)
    }
    async fn handle_asgard_message_timer(role: &mut Role,asgard_data: &mut AsgardData,_asgard_message_timer: AsgardMessageTimer,_sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_election_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_election_timer: AsgardElectionTimer,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        let break_flag = match asgardian_message {
            AsgardianMessage::VoteRequest(vote_request) => Self::handle_vote_request(role,asgard_data,vote_request,sender).await?,
            AsgardianMessage::VoteResponse(vote_response) => Self::handle_vote_response(role,asgard_data,vote_response,sender).await?,
            AsgardianMessage::RebellionRequest(rebellion_request) => Self::handle_rebellion_request(role,asgard_data,rebellion_request,sender).await?,
            AsgardianMessage::RebellionResponse(rebellion_response) => Self::handle_rebellion_response(role,asgard_data,rebellion_response,sender).await?,
            AsgardianMessage::AsgardLogRequest(asgard_log_request) => Self::handle_asgard_log_request(role,asgard_data,asgard_log_request,sender).await?,
            AsgardianMessage::AsgardLogResponse(asgard_log_response) => Self::handle_asgard_log_response(role,asgard_data,asgard_log_response,sender).await?,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => Self::handle_leader_heartbeat(role,asgard_data,leader_heartbeat,sender).await?,
            AsgardianMessage::FollowerUpdate(follower_update) => Self::handle_follower_update(role,asgard_data,follower_update,sender).await?,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => Self::handle_asgard_message_timer(role,asgard_data,asgard_message_timer,sender).await?,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => Self::handle_asgard_election_timer(role,asgard_data,asgard_election_timer,sender).await?,
        };
        Ok(break_flag)
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
    async fn handle_vote_request(role: &mut Role,asgard_data: &mut AsgardData,vote_request: VoteRequest,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_vote_response(role: &mut Role,asgard_data: &mut AsgardData,vote_response: VoteResponse,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_rebellion_request(role: &mut Role,asgard_data: &mut AsgardData,rebellion_request: RebellionRequest,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_rebellion_response(role: &mut Role,asgard_data: &mut AsgardData,rebellion_response: RebellionResponse,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_log_request(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_request: AsgardLogRequest,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
    }
    async fn handle_asgard_log_response(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_response: AsgardLogResponse,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
    }
    async fn handle_leader_heartbeat(role: &mut Role,asgard_data: &mut AsgardData,leader_heartbeat: LeaderHeartbeat,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");   
    }
    async fn handle_follower_update(role: &mut Role,asgard_data: &mut AsgardData,follower_update: FollowerUpdate,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_message_timer(role: &mut Role,asgard_data: &mut AsgardData,_asgard_message_timer: AsgardMessageTimer,_sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    async fn handle_asgard_election_timer(role: &mut Role,asgard_data: &mut AsgardData,asgard_election_timer: AsgardElectionTimer,sender: Address)->Result<bool,AsgardError>{
        panic!("Unimplemented!");
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        let break_flag = match asgardian_message {
            AsgardianMessage::VoteRequest(vote_request) => Self::handle_vote_request(role,asgard_data,vote_request,sender).await?,
            AsgardianMessage::VoteResponse(vote_response) => Self::handle_vote_response(role,asgard_data,vote_response,sender).await?,
            AsgardianMessage::RebellionRequest(rebellion_request) => Self::handle_rebellion_request(role,asgard_data,rebellion_request,sender).await?,
            AsgardianMessage::RebellionResponse(rebellion_response) => Self::handle_rebellion_response(role,asgard_data,rebellion_response,sender).await?,
            AsgardianMessage::AsgardLogRequest(asgard_log_request) => Self::handle_asgard_log_request(role,asgard_data,asgard_log_request,sender).await?,
            AsgardianMessage::AsgardLogResponse(asgard_log_response) => Self::handle_asgard_log_response(role,asgard_data,asgard_log_response,sender).await?,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => Self::handle_leader_heartbeat(role,asgard_data,leader_heartbeat,sender).await?,
            AsgardianMessage::FollowerUpdate(follower_update) => Self::handle_follower_update(role,asgard_data,follower_update,sender).await?,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => Self::handle_asgard_message_timer(role,asgard_data,asgard_message_timer,sender).await?,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => Self::handle_asgard_election_timer(role,asgard_data,asgard_election_timer,sender).await?,
        };
        Ok(break_flag)
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
    fn get_variant(role: &mut Role) -> Result<&mut Self,AsgardError>{
        let candidate = match role {
            Role::Candidate(candidate) => candidate,
            _ => Err(InconsistentRoleError::new("Candidate".to_owned(),role.get_role_name()))?,
        };
        Ok(candidate)
    }
    fn to_leader_uninitialized(role: &mut Role,asgard_data:&AsgardData) ->Result<(),AsgardError> {
        let leader_unintialized = LeaderUninitialized::new(asgard_data)?;
        *role = Role::LeaderUninitialized(leader_unintialized);
        Ok(())
    }
    fn to_follower(role: &mut Role,leader: Option<Address>,voted_for: Address,asgard_data:&AsgardData) -> Result<(),AsgardError> {
        let follower = Follower::new(leader,voted_for,asgard_data)?;
        *role = Role::Follower(follower);
        Ok(())
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
    async fn handle_vote_response(role: &mut Role,asgard_data: &mut AsgardData,vote_response: VoteResponse,sender: Address)->Result<bool,AsgardError>{
        let candidate = Candidate::get_variant(role)?;
        match sender {
            Address::IP(socket_address) => candidate.vote_counter.add_vote(socket_address)?,
            Address::Local => candidate.vote_counter.add_vote(asgard_data.address.clone())?,
            Address::Broadcast => Err(UnexpectedAddressVariantError::new("IP or Local".to_owned(),"Broadcast".to_owned()))?,
        };
        if candidate.vote_counter.got_majority() {
            //Candidate is now leader
            Candidate::to_leader_uninitialized(role,asgard_data)?;
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
    async fn handle_asgard_log_request(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_request: AsgardLogRequest,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
    }
    async fn handle_asgard_log_response(role: &mut Role,asgard_data: &mut AsgardData,asgard_log_response: AsgardLogResponse,sender: Address)->Result<bool,AsgardError> {
        panic!("Unimplemented!");
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
    async fn handle_follower_update(role: &mut Role,asgard_data: &mut AsgardData,follower_update: FollowerUpdate,sender: Address)->Result<bool,AsgardError>{
        unreachable!("Candidate received follower update message. This should not happen as only leaders can receive this message!");
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
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message: AsgardianMessage,sender: Address)->Result<bool,AsgardError>{
        let break_flag = match asgardian_message {
            AsgardianMessage::VoteRequest(vote_request) => Self::handle_vote_request(role,asgard_data,vote_request,sender).await?,
            AsgardianMessage::VoteResponse(vote_response) => Self::handle_vote_response(role,asgard_data,vote_response,sender).await?,
            AsgardianMessage::RebellionRequest(rebellion_request) => Self::handle_rebellion_request(role,asgard_data,rebellion_request,sender).await?,
            AsgardianMessage::RebellionResponse(rebellion_response) => Self::handle_rebellion_response(role,asgard_data,rebellion_response,sender).await?,
            AsgardianMessage::AsgardLogRequest(asgard_log_request) => Self::handle_asgard_log_request(role,asgard_data,asgard_log_request,sender).await?,
            AsgardianMessage::AsgardLogResponse(asgard_log_response) => Self::handle_asgard_log_response(role,asgard_data,asgard_log_response,sender).await?,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => Self::handle_leader_heartbeat(role,asgard_data,leader_heartbeat,sender).await?,
            AsgardianMessage::FollowerUpdate(follower_update) => Self::handle_follower_update(role,asgard_data,follower_update,sender).await?,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => Self::handle_asgard_message_timer(role,asgard_data,asgard_message_timer,sender).await?,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => Self::handle_asgard_election_timer(role,asgard_data,asgard_election_timer,sender).await?,
        };
        Ok(break_flag)
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
    LeaderUninitialized(LeaderUninitialized),
    FollowerUninitialized(FollowerUninitialized),
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
            Role::LeaderUninitialized(_) => "LeaderUninitialized".to_owned(),
            Role::FollowerUninitialized(_) => "FollowerUninitialized".to_owned(),
            Role::Leader(_) => "Leader".to_owned(),
            Role::Follower(_) => "Follower".to_owned(),
            Role::Candidate(_) => "Candidate".to_owned(),
            Role::Immigrant(_) => "Immigrant".to_owned(),
            Role::Exile(_) => "Exile".to_owned(),
        }
    }
    pub(crate) async fn handle_asgardian_message(role: &mut Role,asgard_data: &mut AsgardData,asgardian_message:AsgardianMessage,sender:Address)->Result<bool,AsgardError>{
        let break_flag = match &role {
            Role::LeaderUninitialized(_) => LeaderUninitialized::handle_asgardian_message(role, asgard_data, asgardian_message, sender).await?,
            Role::FollowerUninitialized(_) => FollowerUninitialized::handle_asgardian_message(role, asgard_data, asgardian_message, sender).await?,
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