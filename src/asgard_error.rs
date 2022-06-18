use tokio;
use crate::messages::Message;
use crate::transport::Address;
use tokio::task::JoinError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsgardError {
    #[error("transparent")]
    JoinError(#[from] JoinError),
    #[error("transparent")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<(Message,Address)>),
}