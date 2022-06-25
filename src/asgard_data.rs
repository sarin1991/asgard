use crate::transport::TransportChannel;
use crate::log::{CommittedLog,UncommittedLog};

pub(crate) struct AsgardData {
    pub(crate) term:u64,
    pub(crate) latest_log_index:u64,
    pub(crate) commit_index:u64,
    pub(crate) transport_channel:TransportChannel,
}
impl AsgardData {
    pub(crate) fn new(transport_channel:TransportChannel)->Self{
        Self {
            term:0,
            latest_log_index:0,
            commit_index:0,
            transport_channel,
        }
    }
}