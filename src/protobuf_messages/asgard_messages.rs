#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ipv4Address {
    #[prost(uint32, tag="1")]
    pub address1: u32,
    #[prost(uint32, tag="2")]
    pub address2: u32,
    #[prost(uint32, tag="3")]
    pub address3: u32,
    #[prost(uint32, tag="4")]
    pub address4: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocketAddress {
    #[prost(message, optional, tag="1")]
    pub ipv4_address: ::core::option::Option<Ipv4Address>,
    #[prost(uint32, tag="2")]
    pub port: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(message, optional, tag="1")]
    pub socket_address: ::core::option::Option<SocketAddress>,
    #[prost(enumeration="PeerState", tag="2")]
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
    pub term: u64,
    #[prost(uint64, tag="2")]
    pub log_index: u64,
    #[prost(oneof="asgard_log_message::Message", tags="3, 4")]
    pub message: ::core::option::Option<asgard_log_message::Message>,
}
/// Nested message and enum types in `AsgardLogMessage`.
pub mod asgard_log_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag="3")]
        UserMessage(::prost_types::Any),
        #[prost(message, tag="4")]
        SystemMessage(super::AsgardSystemMessage),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEntry {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(message, repeated, tag="2")]
    pub messages: ::prost::alloc::vec::Vec<AsgardLogMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderHeartbeat {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(string, tag="2")]
    pub leader_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub commit_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowerUpdate {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(uint64, tag="2")]
    pub log_index: u64,
    #[prost(bool, tag="3")]
    pub initialization_flag: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebellionRequest {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(string, tag="2")]
    pub candidate_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteRequest {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(string, tag="2")]
    pub candidate_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub last_log_index_term: u64,
    #[prost(uint64, tag="4")]
    pub last_log_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebellionResponse {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(string, tag="2")]
    pub peer_id: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub vote_granted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteResponse {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(string, tag="2")]
    pub candidate_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderSync {
    #[prost(uint64, tag="1")]
    pub term: u64,
    #[prost(message, repeated, tag="2")]
    pub messages: ::prost::alloc::vec::Vec<AsgardLogMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAsgardMessage {
    #[prost(oneof="generic_asgard_message::GenericMessage", tags="1, 2, 3, 4, 5, 6, 7, 8")]
    pub generic_message: ::core::option::Option<generic_asgard_message::GenericMessage>,
}
/// Nested message and enum types in `GenericAsgardMessage`.
pub mod generic_asgard_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GenericMessage {
        #[prost(message, tag="1")]
        AddEntryMessage(super::AddEntry),
        #[prost(message, tag="2")]
        HeartBeatMessage(super::LeaderHeartbeat),
        #[prost(message, tag="3")]
        FollowerUpdateMessage(super::FollowerUpdate),
        #[prost(message, tag="4")]
        CanvassRebellionMessage(super::RebellionRequest),
        #[prost(message, tag="5")]
        RequestVotesMessage(super::VoteRequest),
        #[prost(message, tag="6")]
        CanvassResponseMessage(super::RebellionResponse),
        #[prost(message, tag="7")]
        VoteResponseMessage(super::VoteResponse),
        #[prost(message, tag="8")]
        LeaderSyncMessage(super::LeaderSync),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerState {
    Active = 0,
    Immigrant = 1,
    Exile = 2,
}
