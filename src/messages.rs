use crate::protobuf_messages::asgard_messages::{LeaderSync,FollowerInitialized,LeaderHeartbeat,
    VoteResponse,VoteRequest,RebellionResponse,RebellionRequest,FollowerUpdate,AddEntry};


#[derive(Clone)]
pub struct AsgardMessageTimer{

}

#[derive(Clone)]
pub struct AsgardElectionTimer{

}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum APIMessage{

}

#[derive(Clone)]
pub enum Message{
    AsgardianMessage(AsgardianMessage),
    APIMessage(APIMessage),
}