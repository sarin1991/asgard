use std::net::{SocketAddr,IpAddr,Ipv4Addr};
use crate::protobuf_messages::asgard_messages::Peer;
use crate::protobuf_messages::asgard_messages::PeerState;
use crate::protobuf_messages::asgard_messages::SocketAddress;
use crate::transport::Address;
use crate::transport::TransportChannel;
use crate::log::{CommittedLog,UncommittedLog};
use crate::asgard_error::{AsgardError,ProtobufParsingError};
use crate::messages::{AsgardianMessage,Message};

pub(crate) struct AsgardData {
    pub(crate) address:SocketAddr,
    pub(crate) term:u64,
    pub(crate) commit_index:u64,
    pub(crate) transport_channel:TransportChannel,
    pub(crate) uncommmitted_log:UncommittedLog,
    pub(crate) committed_log:CommittedLog,
    pub(crate) peers: Vec<Peer>,
}
impl AsgardData {
    pub(crate) fn new(transport_channel:TransportChannel,address:SocketAddr)->Self{
        Self {
            address,
            term:0,
            commit_index:0,
            transport_channel,
            uncommmitted_log:UncommittedLog::new(),
            committed_log:CommittedLog::new(),
            peers: vec![],
        }
    }
    fn u32_to_u8(num:u32)->Result<u8,AsgardError>{
        let result = u8::try_from(num);
        match result {
            Ok(num_u8) => Ok(num_u8),
            _ => Err(ProtobufParsingError::new("Not able to convert u32 number in IP Address to u8".to_owned()))?,
        }
    }
    fn get_socket_address(protobuf_socket_address:&SocketAddress)->Result<SocketAddr,AsgardError> {
        let protobuf_ipv4_address = match &protobuf_socket_address.ipv4_address {
            Some(protobuf_ipv4_address) => protobuf_ipv4_address,
            None => Err(ProtobufParsingError::new("Socket address did not contain any ipv4address".to_owned()))?,
        };
        let ipv4_address = Ipv4Addr::new(AsgardData::u32_to_u8(protobuf_ipv4_address.address1)?,AsgardData::u32_to_u8(protobuf_ipv4_address.address2)?,
                                            AsgardData::u32_to_u8(protobuf_ipv4_address.address3)?,AsgardData::u32_to_u8(protobuf_ipv4_address.address4)?);
        let port = match u16::try_from(protobuf_socket_address.port) {
            Ok(port) => port,
            _ => Err(ProtobufParsingError::new("Port number in socket address cannot be casted to u16".to_owned()))?,
        };
        let socket_address = SocketAddr::new(IpAddr::V4(ipv4_address), port);
        Ok(socket_address)
    }
    pub(crate) fn get_active_peers(&self) -> Result<Vec<SocketAddr>,AsgardError> {
        let mut peers = vec![];
        for peer in self.peers.iter() {
            let peer_state_option = PeerState::from_i32(peer.peer);
            let peer_state = match peer_state_option {
                Some(peer_state) => peer_state,
                None => Err(ProtobufParsingError::new("Unable to parse peer state enum".to_owned()))?,
            };
            match peer_state {
                PeerState::Active => {
                    let protobuf_socket_address = match &peer.socket_address {
                        Some(protobuf_socket_address) => protobuf_socket_address,
                        None => Err(ProtobufParsingError::new("Peer did not contain any socket address".to_owned()))?,
                    };
                    let socket_address = AsgardData::get_socket_address(protobuf_socket_address)?;
                    peers.push(socket_address);
                },
                _ =>(),
            }
        }
        Ok(peers)
    }
    pub(crate) async fn send_asgardian_message(&self,message:AsgardianMessage,address:Address)->Result<(),AsgardError>{
        let tx = self.transport_channel.outbound_asgardian_message_sender.clone();
        tx.send((message,address)).await?;
        Ok(())
    }
    pub(crate) async fn repeat_message(&self,message:Message,address:Address)->Result<(),AsgardError>{
        let tx = self.transport_channel.inbound_message_sender.clone();
        tx.send((message,address)).await?;
        Ok(())
    }
    pub(crate) fn get_last_log_index(&self)->u64 {
        let uncommitted_last_log_index_option = self.uncommmitted_log.get_last_log_index();
        match uncommitted_last_log_index_option {
            Some(uncommitted_last_log_index) => uncommitted_last_log_index,
            None => self.committed_log.get_last_log_index(),
        }
    }
    pub(crate) fn get_last_log_index_term(&self)->u64{
        let uncommitted_last_log_index_term_option = self.uncommmitted_log.get_last_log_index_term();
        match uncommitted_last_log_index_term_option {
            Some(uncommitted_last_log_index_term) => uncommitted_last_log_index_term,
            None => self.committed_log.get_last_log_index_term(),
        }
    }
}