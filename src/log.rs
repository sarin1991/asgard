use std::collections::VecDeque;

use crate::protobuf_messages::asgard_messages::AsgardLogMessage;
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
}