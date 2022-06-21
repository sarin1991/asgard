use std::collections::VecDeque;

use crate::protobuf_messages::asgard_messages::AsgardLogMessage;
pub(crate) struct CommitLog {
    logs: Vec<AsgardLogMessage>,
}

pub(crate) struct UncommittedLog{
    logs: VecDeque<AsgardLogMessage>,
}