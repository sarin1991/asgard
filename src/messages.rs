use crate::protobuf_messages::asgard_messages::{VoteRequest,VoteResponse,RebellionRequest,RebellionResponse,
    AsgardLogRequest,AsgardLogResponse,LeaderHeartbeat,FollowerUpdate};


#[derive(Clone,Debug)]
pub(crate) struct AsgardMessageTimer{
}

#[derive(Clone,Debug)]
pub(crate) struct AsgardElectionTimer{
}

#[derive(Clone,Debug)]
pub(crate) enum AsgardianMessage{
    VoteRequest(VoteRequest),
    VoteResponse(VoteResponse),
    RebellionRequest(RebellionRequest),
    RebellionResponse(RebellionResponse),
    AsgardLogRequest(AsgardLogRequest),
    AsgardLogResponse(AsgardLogResponse),
    LeaderHeartbeat(LeaderHeartbeat),
    FollowerUpdate(FollowerUpdate),
    AsgardMessageTimer(AsgardMessageTimer),
    AsgardElectionTimer(AsgardElectionTimer),
}

impl AsgardianMessage {
    pub(crate) fn is_higher_term(&self,current_term:u64) -> bool {
        match self {
            AsgardianMessage::VoteRequest(vote_request) => vote_request.term>current_term,
            AsgardianMessage::VoteResponse(vote_response) => vote_response.term>current_term,
            AsgardianMessage::RebellionRequest(rebellion_request) => rebellion_request.term>current_term,
            AsgardianMessage::RebellionResponse(rebellion_response) => rebellion_response.term>current_term,
            AsgardianMessage::AsgardLogRequest(asgard_log_request) => asgard_log_request.term>current_term,
            AsgardianMessage::AsgardLogResponse(asgard_log_response) => asgard_log_response.term>current_term,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => leader_heartbeat.term>current_term,
            AsgardianMessage::FollowerUpdate(follower_update) => follower_update.term>current_term,
            AsgardianMessage::AsgardMessageTimer(_) => false,
            AsgardianMessage::AsgardElectionTimer(_) => false,
        }
    }
    pub(crate) fn is_lower_term(&self,current_term:u64) -> bool {
        match self {
            AsgardianMessage::VoteRequest(vote_request) => vote_request.term<current_term,
            AsgardianMessage::VoteResponse(vote_response) => vote_response.term<current_term,
            AsgardianMessage::RebellionRequest(rebellion_request) => rebellion_request.term<current_term,
            AsgardianMessage::RebellionResponse(rebellion_response) => rebellion_response.term<current_term,
            AsgardianMessage::AsgardLogRequest(asgard_log_request) => asgard_log_request.term<current_term,
            AsgardianMessage::AsgardLogResponse(asgard_log_response) => asgard_log_response.term<current_term,
            AsgardianMessage::LeaderHeartbeat(leader_heartbeat) => leader_heartbeat.term<current_term,
            AsgardianMessage::FollowerUpdate(follower_update) => follower_update.term<current_term,
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