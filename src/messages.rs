use crate::protobuf_messages::asgard_messages::{LeaderSync,FollowerInitialized,LeaderHeartbeat,
    VoteResponse,VoteRequest,RebellionResponse,RebellionRequest,FollowerUpdate,AddEntry};


#[derive(Clone,Debug)]
pub struct AsgardMessageTimer{

}

#[derive(Clone,Debug)]
pub struct AsgardElectionTimer{

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

#[derive(Clone,Debug)]
pub enum APIMessage{

}

#[derive(Clone,Debug)]
pub enum Message{
    AsgardianMessage(AsgardianMessage),
    APIMessage(APIMessage),
}