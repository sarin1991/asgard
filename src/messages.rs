use crate::protobuf_messages::asgard_messages::{LeaderSync,FollowerInitialized,LeaderHeartbeat,
    VoteResponse,VoteRequest,RebellionResponse,RebellionRequest,FollowerUpdate,AddEntry};


#[derive(Clone,Debug)]
pub(crate) struct AsgardMessageTimer{
    pub(crate) term:u64,
}

#[derive(Clone,Debug)]
pub(crate) struct AsgardElectionTimer{
    pub(crate) term:u64,
}

#[derive(Clone,Debug)]
pub enum AsgardianMessage{
    LeaderSync(LeaderSync),
    FollowerInitialized(FollowerInitialized),
    LeaderHeartbeat(LeaderHeartbeat),
    VoteResponse(VoteResponse),
    VoteRequest(VoteRequest),
    RebellionResponse(RebellionResponse),
    RebellionRequest(RebellionRequest),
    FollowerUpdate(FollowerUpdate),
    AddEntry(AddEntry),
    AsgardMessageTimer(AsgardMessageTimer),
    AsgardElectionTimer(AsgardElectionTimer),
}

impl AsgardianMessage {
    pub(crate) fn get_message_term(&self) -> u64 {
        match self {
            AsgardianMessage::LeaderSync(leader_sync) => leader_sync.term,
            AsgardianMessage::FollowerInitialized(follower_initialized) => follower_initialized.term,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => leader_heartbeat.term,
            AsgardianMessage::VoteResponse(vote_response) => vote_response.term,
            AsgardianMessage::VoteRequest(vote_request) => vote_request.term,
            AsgardianMessage::RebellionResponse(rebellion_response) => rebellion_response.term,
            AsgardianMessage::RebellionRequest(rebellion_request) => rebellion_request.term,
            AsgardianMessage::FollowerUpdate(follower_update) => follower_update.term,
            AsgardianMessage::AddEntry(add_entry) => add_entry.term,
            AsgardianMessage::AsgardMessageTimer(asgard_message_timer) => asgard_message_timer.term,
            AsgardianMessage::AsgardElectionTimer(asgard_election_timer) => asgard_election_timer.term,
        }
    }
}

#[derive(Clone,Debug)]
pub enum APIMessage{

}

#[derive(Clone,Debug)]
pub enum Message{
    AsgardianMessage(AsgardianMessage),
    APIMessage(APIMessage),
}