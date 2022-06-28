use crate::protobuf_messages::asgard_messages::{LeaderSync,LeaderHeartbeat,
    VoteResponse,VoteRequest,RebellionResponse,RebellionRequest,FollowerUpdate,AddEntry};


#[derive(Clone,Debug)]
pub(crate) struct AsgardMessageTimer{
}

#[derive(Clone,Debug)]
pub(crate) struct AsgardElectionTimer{
}

#[derive(Clone,Debug)]
pub(crate) enum AsgardianMessage{
    LeaderSync(LeaderSync),
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
    pub(crate) fn is_higher_term(&self,current_term:u64) -> bool {
        match self {
            AsgardianMessage::LeaderSync(leader_sync) => leader_sync.term>current_term,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => leader_heartbeat.term>current_term,
            AsgardianMessage::VoteResponse(vote_response) => vote_response.term>current_term,
            AsgardianMessage::VoteRequest(vote_request) => vote_request.term>current_term,
            AsgardianMessage::RebellionResponse(rebellion_response) => rebellion_response.term>current_term,
            AsgardianMessage::RebellionRequest(rebellion_request) => rebellion_request.term>current_term,
            AsgardianMessage::FollowerUpdate(follower_update) => follower_update.term>current_term,
            AsgardianMessage::AddEntry(add_entry) => add_entry.term>current_term,
            AsgardianMessage::AsgardMessageTimer(_) => false,
            AsgardianMessage::AsgardElectionTimer(_) => false,
        }
    }
    pub(crate) fn is_lower_term(&self,current_term:u64) -> bool {
        match self {
            AsgardianMessage::LeaderSync(leader_sync) => leader_sync.term<current_term,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => leader_heartbeat.term<current_term,
            AsgardianMessage::VoteResponse(vote_response) => vote_response.term<current_term,
            AsgardianMessage::VoteRequest(vote_request) => vote_request.term<current_term,
            AsgardianMessage::RebellionResponse(rebellion_response) => rebellion_response.term<current_term,
            AsgardianMessage::RebellionRequest(rebellion_request) => rebellion_request.term<current_term,
            AsgardianMessage::FollowerUpdate(follower_update) => follower_update.term<current_term,
            AsgardianMessage::AddEntry(add_entry) => add_entry.term<current_term,
            AsgardianMessage::AsgardMessageTimer(_) => false,
            AsgardianMessage::AsgardElectionTimer(_) => false,
        }
    }
}

#[derive(Clone,Debug)]
pub enum APIMessage{

}

#[derive(Clone,Debug)]
pub(crate) enum Message{
    AsgardianMessage(AsgardianMessage),
    APIMessage(APIMessage),
}