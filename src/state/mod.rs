use std::collections::{HashMap,BinaryHeap};
use std::cmp::Reverse;
use std::sync::{Arc,Mutex,RwLock}
use tokio::sync::mpsc::{Sender,Receiver};
use crate::transport::Transport;
use crate::state::asgard_state::{AsgardState,AsgardStateRequest,AsgardStateResponse,AsgardStateCallback,AsgardStateTx};
use crate::state::king::King;
use crate::state::follower::Follower;
mod asgard_state;
mod asgardian;
mod common;

enum AsgardMessage

enum Message{
    Heartbeat;
    AddEntry;
    Update;
    CanvassRebellion;
    RequestVotes;
    CanvassResponse;
    Vote;
    ClientRequest;
}

enum ClientRequest{
    SendMessage;
    GetMessage;
}