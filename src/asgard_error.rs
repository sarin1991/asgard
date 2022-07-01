use tokio;
use crate::messages::Message;
use crate::transport::Address;
use tokio::task::JoinError;
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

#[derive(Error, Debug)]
pub(crate) enum AsgardError {
    #[error("transparent")]
    JoinError(#[from] JoinError),
    #[error("transparent")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<(Message,Address)>),
    #[error("transparent")]
    InconsistentRoleError(#[from] InconsistentRoleError),
}