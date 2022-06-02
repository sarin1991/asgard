use crate::state::common::PeerID;
use crate::state::role::common::{AsgardTx,AsgardRx};
use crate::state::asgard_state::{AsgardStateRequest,AsgardStateResponse,AsgardStateTx,AsgardStateCallback};
use std::time::Instant;
use std::collections::VecDeque;
use tokio::sync::{oneshot, mpsc};

const LOG_STAT_ARRAY_LEN:usize = 100; 

enum KingInternalMessage{

}

struct PeerState{
    current_log_index: Option<u64>;
    current_reign: Option<u32>; 
}

struct MessageRetryHandler {
    reign: u32;
    log_index: u64;
    last_send_time: Instant;
}

struct LogReplicationStatistic{
    reign: u32;
    log_index: u64;
    num_peers_replicated: u16;
}

struct LogReplicationDeque{
    replication_stat_deque: VecDeque<LogReplicationStatistic>;
    front_log_index: u64;
    back_log_index: u64;
}

struct King{
    peer_state: HashMap<PeerID,PeerState>,
    log_replication_statistic_deque: LogReplicationDeque,
    asgard_tx: AsgardTx,
    asgard_rx: AsgardRx,
    asgard_state_tx: AsgardStateTx,
    task_killer_sender: Sender,
    kill_signal_rx: Receiver,
}

impl King{
    fn new(asgard_tx:AsgardTx,asgard_rx:AsgardRx,asgard_state_tx:AsgardStateTx)->Self{
        let (tx,rx) = AsgardStateCallback::get_oneshot_callback();
        let asgard_request = Box::new(AsgardStateRequest::GET_PEERS_REQUEST);
        asgard_state_tx.send((asgard_request,tx));
        let asgard_callback_response = rx.await.unwrap();
        #TODO:
    }
    async fn event_handler(mut self,kill_signal_rx: Receiver<bool>,asgard_transport_rx: Receiver<Message>,asgard_state_tx:Sender<AsgardStateMessage,Option>){
        loop {
            tokio::select!{

            }
        }
    }
    async fn message_retry_handler();
    async fn handle_message(Self,asgard_state: Arc<RwLock<AsgardState>>, message:Message)->Role{
        match message {
            Message::ClientRequest(client_message) => self.handle_client_message(client_message),

        }
        if cond1
            Role::King(Self)
        else
            Role
    }
}