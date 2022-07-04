use crate::protobuf_messages::asgard_messages::Peer;
use crate::protobuf_messages::asgard_messages::PeerState;
use crate::transport::Address;
use crate::transport::TransportChannel;
use crate::log::{CommittedLog,UncommittedLog};
use crate::asgard_error::{AsgardError,ProtobufParsingError};

pub(crate) struct AsgardData {
    pub(crate) term:u64,
    pub(crate) latest_log_index:u64,
    pub(crate) commit_index:u64,
    pub(crate) transport_channel:TransportChannel,
    pub(crate) uncommmitted_log:UncommittedLog,
    pub(crate) committed_log:CommittedLog,
    pub(crate) peers: Vec<Peer>,
}
impl AsgardData {
    pub(crate) fn new(transport_channel:TransportChannel)->Self{
        Self {
            term:0,
            latest_log_index:0,
            commit_index:0,
            transport_channel,
            uncommmitted_log:UncommittedLog::new(),
            committed_log:CommittedLog::new(),
            peers: vec![],
        }
    }
    pub(crate) fn get_active_peers(&self) -> Result<Vec<Address>,AsgardError> {
        let mut peers = vec![];
        for peer in self.peers.iter() {
            let peer_state_option = PeerState::from_i32(peer.peer);
            let peer_state = match peer_state_option {
                Some(peer_state) => peer_state,
                None => Err(ProtobufParsingError::new("Unable to parse peer state enum".to_owned()))?,
            };
            match peer_state {
                PeerState::Active => peers.push(Address::IP(peer.peer_id.clone())),
                _ =>(),
            }
        }
        Ok(peers)
    }
}