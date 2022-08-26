use std::net::SocketAddr;
use crate::transport::Address;
use crate::asgard_error::{AsgardError,InconsistentRoleError,UnknownPeerError, UnexpectedAddressVariantError, InconsistentStateError};

pub(crate) fn address_to_socket_address(address: Address,local_address: &SocketAddr) -> Result<SocketAddr,AsgardError>{
    match address {
        Address::IP(socket_address) => Ok(socket_address),
        Address::Local => Ok(local_address.clone()),
        Address::Broadcast => Err(AsgardError::UnexpectedAddressVariantError(UnexpectedAddressVariantError::new("IP or Local".to_owned(),"Broadcast".to_owned()))),
    }
}