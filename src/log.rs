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
}