use tokio;
use crate::messages::{Message, AsgardianMessage};
use crate::transport::Address;
use tokio::task::JoinError;
use std::net::SocketAddr;
use std::{error::Error, fmt};
use thiserror::Error;

#[derive(Debug)]
pub(crate) struct InconsistentRoleError{
    error_string:String,
}
impl InconsistentRoleError {
    pub(crate) fn new(expected_role_name:String,actual_role_name:String) -> Self {
        Self {
            error_string:Self::get_error_string(expected_role_name.clone(), actual_role_name.clone()),
        }
    }
    fn get_error_string(expected_role_name:String,actual_role_name:String)->String {
        let s = format!("Error: Inconsistent role. Expected role {} but got role {}",expected_role_name.as_str(),actual_role_name.as_str());
        s
    }
}
impl fmt::Display for InconsistentRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for InconsistentRoleError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Debug)]
pub(crate) struct UnknownPeerError{
    error_string:String,
}
impl UnknownPeerError {
    pub(crate) fn new(context:String,peer:SocketAddr) -> Self {
        Self {
            error_string: Self::get_error_string(context,peer),
        }
    }
    fn get_error_string(context:String,peer:SocketAddr)->String {
        let s = format!("{}: Peer - {} was not found",context.as_str(),peer);
        s
    }
}
impl fmt::Display for UnknownPeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for UnknownPeerError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Debug)]
pub(crate) struct ProtobufParsingError{
    error_string:String,
}
impl ProtobufParsingError {
    pub(crate) fn new(error_string:String) -> Self {
        Self {
            error_string,
        }
    }
}
impl fmt::Display for ProtobufParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for ProtobufParsingError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Debug)]
pub(crate) struct AddressSerializationError{
    error_string:String,
}
impl AddressSerializationError {
    pub(crate) fn new() -> Self {
        Self {
            error_string:"Unable to convert Address to String".to_owned(),
        }
    }
}
impl fmt::Display for AddressSerializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for AddressSerializationError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Debug)]
pub(crate) struct UnexpectedAddressVariantError{
    error_string:String,
}
impl UnexpectedAddressVariantError {
    pub(crate) fn new(expected_variant:String,actual_variant:String) -> Self {
        Self {
            error_string:format!("Unexpected Address Variant error: Expected address variant {}, but got variant {}",expected_variant,actual_variant).to_owned(),
        }
    }
}
impl fmt::Display for UnexpectedAddressVariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for UnexpectedAddressVariantError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Debug)]
pub(crate) struct InconsistentStateError{
    error_string:String,
}
impl InconsistentStateError {
    pub(crate) fn new(error_string:String) -> Self {
        Self {
            error_string:format!("InconsistentStateError! - {}",error_string).to_owned(),
        }
    }
}
impl fmt::Display for InconsistentStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for InconsistentStateError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Debug)]
pub(crate) struct LogIndexOutOfBoundError{
    error_string:String,
}
impl LogIndexOutOfBoundError {
    pub(crate) fn new(error_string:String) -> Self {
        Self {
            error_string:format!("LogIndexOutOfBoundError! - {}",error_string).to_owned(),
        }
    }
}
impl fmt::Display for LogIndexOutOfBoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for LogIndexOutOfBoundError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Debug)]
pub(crate) struct InconsistentInputsError{
    error_string:String,
}
impl InconsistentInputsError {
    pub(crate) fn new(error_string:String) -> Self {
        Self {
            error_string:format!("InconsistentInputsError! - {}",error_string).to_owned(),
        }
    }
}
impl fmt::Display for InconsistentInputsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.error_string)
    }
}
impl Error for InconsistentInputsError {
    fn description(&self) -> &str {
        &self.error_string
    }
}

#[derive(Error, Debug)]
pub(crate) enum AsgardError {
    #[error("transparent")]
    JoinError(#[from] JoinError),
    #[error("transparent")]
    SendMessageError(#[from] tokio::sync::mpsc::error::SendError<(Message,Address)>),
    #[error("transparent")]
    SendAsgardianMessageError(#[from] tokio::sync::mpsc::error::SendError<(AsgardianMessage,Address)>),
    #[error("transparent")]
    InconsistentRoleError(#[from] InconsistentRoleError),
    #[error("transparent")]
    UnknownPeerError(#[from] UnknownPeerError),
    #[error("transparent")]
    ProtobufParsingError(#[from] ProtobufParsingError),
    #[error("transparent")]
    AddressSerializationError(#[from] AddressSerializationError),
    #[error("transparent")]
    UnexpectedAddressVariantError(#[from] UnexpectedAddressVariantError),
    #[error("transparent")]
    InconsistentStateError(#[from] InconsistentStateError),
    #[error("transparent")]
    LogIndexOutOfBoundError(#[from] LogIndexOutOfBoundError),
    #[error("transparent")]
    InconsistentInputsError(#[from] InconsistentInputsError),
}