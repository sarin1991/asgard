#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(string, tag="1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(enumeration="PeerState", tag="3")]
    pub peer: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsgardPeerInfo {
    #[prost(message, repeated, tag="1")]
    pub peer_info: ::prost::alloc::vec::Vec<Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsgardSystemMessage {
    #[prost(oneof="asgard_system_message::Message", tags="1")]
    pub message: ::core::option::Option<asgard_system_message::Message>,
}
/// Nested message and enum types in `AsgardSystemMessage`.
pub mod asgard_system_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="1")]
        AsgardPeerInfoMessage(super::AsgardPeerInfo),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsgardLogMessage {
    #[prost(uint64, tag="1")]
    pub log_index: u64,
    #[prost(oneof="asgard_log_message::Message", tags="2, 3")]
    pub message: ::core::option::Option<asgard_log_message::Message>,
}
/// Nested message and enum types in `AsgardLogMessage`.
pub mod asgard_log_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="2")]
        UserMessage(::prost_types::Any),
        #[prost(message, tag="3")]
        SystemMessage(super::AsgardSystemMessage),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEntry {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<AsgardLogMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderHeartbeat {
    #[prost(string, tag="1")]
    pub leader_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub commit_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowerUpdate {
    #[prost(uint64, tag="1")]
    pub log_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebellionRequest {
    #[prost(string, tag="1")]
    pub candidate_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteRequest {
    #[prost(string, tag="1")]
    pub candidate_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub last_log_index_term: u64,
    #[prost(uint64, tag="3")]
    pub last_log_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebellionResponse {
    #[prost(string, tag="1")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub vote_granted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteResponse {
    #[prost(string, tag="1")]
    pub candidate_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub term: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderSync {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<AsgardLogMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowerInitialized {
    #[prost(string, tag="1")]
    pub leader_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAsgardMessage {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(oneof="generic_asgard_message::GenericMessage", tags="2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub generic_message: ::core::option::Option<generic_asgard_message::GenericMessage>,
}
/// Nested message and enum types in `GenericAsgardMessage`.
pub mod generic_asgard_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GenericMessage {
        #[prost(message, tag="2")]
        AddEntryMessage(super::AddEntry),
        #[prost(message, tag="3")]
        HeartBeatMessage(super::LeaderHeartbeat),
        #[prost(message, tag="4")]
        FollowerUpdateMessage(super::FollowerUpdate),
        #[prost(message, tag="5")]
        CanvassRebellionMessage(super::RebellionRequest),
        #[prost(message, tag="6")]
        RequestVotesMessage(super::VoteRequest),
        #[prost(message, tag="7")]
        CanvassResponseMessage(super::RebellionResponse),
        #[prost(message, tag="8")]
        VoteResponseMessage(super::VoteResponse),
        #[prost(message, tag="9")]
        LeaderSyncMessage(super::LeaderSync),
        #[prost(message, tag="10")]
        FollowerInitializedMessage(super::FollowerInitialized),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerState {
    Active = 0,
    Immigrant = 1,
    Exile = 2,
}
