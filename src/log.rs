use std::collections::VecDeque;

use crate::protobuf_messages::asgard_messages::AsgardLogMessage;
use crate::asgard_error::{AsgardError,LogIndexOutOfBoundError};
pub(crate) struct CommittedLog {
    logs: Vec<AsgardLogMessage>,
}
impl CommittedLog {
    pub(crate) fn new()->Self{
        Self {
            logs: Vec::<AsgardLogMessage>::new(),
        }
    }
    pub(crate) fn get_last_log_index(&self) -> u64 {
        let last_log_option = self.logs.last();
        match last_log_option {
            Some(last_log) => last_log.log_index,
            None => 0
        }
    }
    pub(crate) fn get_last_log_index_term(&self) -> u64 {
        let last_log_option = self.logs.last();
        match last_log_option {
            Some(last_log) => last_log.term,
            None => 0
        }
    }
    pub(crate) fn get_logs(&self,start_index:usize,end_index:usize) -> Result<Vec<AsgardLogMessage>,AsgardError> {
        let log_slice_option = self.logs.get(start_index..end_index);
        let log_slice = match log_slice_option {
            Some(log_slice) => log_slice,
            None => Err(LogIndexOutOfBoundError::new(format!("Committed Log is of size {}. Got start\
             and end index {} and {} respectively",self.logs.len(),start_index,end_index).to_owned()))?,
        };
        let mut log_messages = vec![];
        for log_message in log_slice {
            log_messages.push(log_message.clone());
        }
        Ok(log_messages)
    }
}

pub(crate) struct UncommittedLog{
    logs: VecDeque<AsgardLogMessage>,
}
impl UncommittedLog{
    pub(crate) fn new()->Self{
        Self {
            logs: VecDeque::<AsgardLogMessage>::new(),
        }
    }
    pub(crate) fn get_last_log_index(&self) -> Option<u64> {
        match self.logs.back() {
            Some(asgard_log_message) => Some(asgard_log_message.log_index),
            None => None
        }
    }
    pub(crate) fn get_last_log_index_term(&self) -> Option<u64> {
        match self.logs.back() {
            Some(asgard_log_message) => Some(asgard_log_message.term),
            None => None
        }
    }
    pub(crate) fn get_logs(&self) -> Vec<AsgardLogMessage> {
        let mut log_messages = vec![];
        for log_message in self.logs.iter() {
            log_messages.push(log_message.clone());
        }
        log_messages
    }
}