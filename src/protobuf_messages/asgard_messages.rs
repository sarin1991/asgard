#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(uint64, tag="1")]
    pub peer_id: u64,
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
pub struct OptionalMessageReign {
    #[prost(uint32, tag="1")]
    pub message_reign: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalTransactionId {
    #[prost(string, tag="1")]
    pub transaction_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsgardMessage {
    #[prost(uint64, tag="1")]
    pub log_index: u64,
    #[prost(message, optional, tag="4")]
    pub message_reign: ::core::option::Option<OptionalMessageReign>,
    #[prost(message, optional, tag="5")]
    pub transaction_id: ::core::option::Option<OptionalTransactionId>,
    #[prost(oneof="asgard_message::Message", tags="2, 3")]
    pub message: ::core::option::Option<asgard_message::Message>,
}
/// Nested message and enum types in `AsgardMessage`.
pub mod asgard_message {
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
    #[prost(uint32, tag="1")]
    pub reign: u32,
    #[prost(message, repeated, tag="3")]
    pub messages: ::prost::alloc::vec::Vec<AsgardMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {
    #[prost(uint32, tag="1")]
    pub reign: u32,
    #[prost(uint64, tag="2")]
    pub leader_id: u64,
    #[prost(uint64, tag="3")]
    pub commit_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowerUpdate {
    #[prost(uint32, tag="1")]
    pub reign: u32,
    #[prost(uint64, tag="2")]
    pub log_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanvassRebellion {
    #[prost(uint32, tag="1")]
    pub reign: u32,
    #[prost(uint64, tag="2")]
    pub candidate_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestVotes {
    #[prost(uint32, tag="1")]
    pub reign: u32,
    #[prost(uint64, tag="2")]
    pub candidate_id: u64,
    #[prost(uint64, tag="3")]
    pub last_log_index: u64,
    #[prost(uint32, tag="4")]
    pub reign_of_last_log_index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanvassResponse {
    #[prost(uint32, tag="1")]
    pub reign: u32,
    #[prost(uint64, tag="2")]
    pub peer_id: u64,
    #[prost(bool, tag="3")]
    pub vote_granted: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAsgardMessage {
    #[prost(oneof="generic_asgard_message::GenericMessage", tags="1, 2, 3, 4, 5, 6")]
    pub generic_message: ::core::option::Option<generic_asgard_message::GenericMessage>,
}
/// Nested message and enum types in `GenericAsgardMessage`.
pub mod generic_asgard_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GenericMessage {
        #[prost(message, tag="1")]
        AddEntryMessage(super::AddEntry),
        #[prost(message, tag="2")]
        HeartBeatMessage(super::Heartbeat),
        #[prost(message, tag="3")]
        FollowerUpdateMessage(super::FollowerUpdate),
        #[prost(message, tag="4")]
        CanvassRebellionMessage(super::CanvassRebellion),
        #[prost(message, tag="5")]
        RequestVotesMessage(super::RequestVotes),
        #[prost(message, tag="6")]
        CanvassResponseMessage(super::CanvassResponse),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerState {
    Active = 0,
    Immigrant = 1,
    Exile = 2,
}
