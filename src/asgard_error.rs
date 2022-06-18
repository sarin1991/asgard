use tokio;
use crate::messages::Message;
use crate::transport::Address;
use tokio::task::JoinError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsgardError {
    #[error("Join Error")]
    JoinError(JoinError),
    #[error("Send Error")]
    SendError(tokio::sync::mpsc::error::SendError<(Message,Address)>),
}