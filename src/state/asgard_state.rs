use crate::state::common::PeerID;
use std::collections::HashMap;
use tokio::sync::{oneshot,mpsc};
use crate::protobuf_messages::asgard_messages::GenericAsgardMessage;

struct Log{
    reign: u32,
    index: u64,
    message: GenericAsgardMessage,
    transaction_id: Option<String>,
}

struct Peer{
    peer_id: PeerID,
    end_point: String,
}

impl Peer{
    fn new(peer_id:PeerID,end_point:String)->Self{
        Self{
            peer_id,
            end_point,
        }
    }
    fn get_peer_id(&self)->PeerID{
        self.peer_id.clone()
    }
}

pub enum AsgardStateRequest{
    GetPeersRequest,
}

pub enum AsgardStateResponse{
    GetPeersResponse(Vec<Peer>),
}

struct AsgardStateCallbackResponse{
    request: Box<AsgardStateRequest>,
    response: Box<AsgardStateResponse>,
}
impl AsgardStateCallbackResponse{
    fn new(request: Box<AsgardStateRequest>,response: Box<AsgardStateResponse>)->Self{
        Self{
            request,
            response
        }
    }
}

#[derive(Clone)]
pub enum AsgardStateCallback{
    MpscTx(mpsc::Sender<AsgardStateCallbackResponse>),
    OneshotTx(oneshot::Sender<AsgardStateCallbackResponse>),
    None,
}
impl AsgardStateCallback{
    pub fn get_mpsc_callback(channel_capacity: usize) -> (AsgardStateCallback,mpsc::Receiver<AsgardStateCallbackResponse>){
        let (tx,rx) = mpsc::channel<AsgardStateCallbackResponse>(channel_capacity);
        return (AsgardStateCallback::MpscTx(tx),rx)
    }
    pub fn get_oneshot_callback() -> (AsgardStateCallback,oneshot::Receiver<AsgardStateCallbackResponse>) {
        let (tx,rx) = oneshot::channel<AsgardStateCallbackResponse>();
        return (AsgardStateCallback::OneshotTx(tx),rx)
    }
    fn send(self,asgard_state_callback_response:AsgardStateCallbackResponse){
        match self {
            AsgardStateCallback::MpscTx(tx) => tx.send(asgard_state_callback_response).await.unwrap(),
            AsgardStateCallback::OneshotTx(tx) => tx.send(asgard_state_callback_response).unwrap(),
            None => (),
        }
    }
}

type AsgardStateTx = mspc::Sender<(Box<AsgardStateRequest>,AsgardStateCallback)>;

struct AsgardState{
    current_reign: u32,
    voted_for: Option<PeerID>,
    leader_id: Option<PeerID>,
    commit_index: u64,
    latest_log_index: u64,
    logs: Vec<Log>,
    number_of_peers: u16,
    peers: HashMap<PeerID,Peer>,
    asgard_state_tx: AsgardStateTx,
}

impl AsgardState{
    async fn event_handler(self,rx:mpsc::Receiver<(Box<AsgardStateRequest>,AsgardStateCallback)>)->bool{
        loop{
            let (asgard_state_request,asgard_state_callback) = rx.recv().await.unwrap();
            match asgard_state_request {
                AsgardStateRequest::GetPeersRequest => {
                    let response:Vec<Peer> = vec![];
                    for peer in self.peers.values(){
                        response.push(peer.clone());
                    }
                    let asgard_state_response = AsgardStateResponse::GetPeersResponse(response);
                    let asgard_state_callback_response = AsgardStateCallbackResponse::new(asgard_state_request,Box::new(asgard_state_response));
                    asgard_state_callback.send(asgard_state_callback_response);
                }
            }
        }
    }
    fn start(peer:Peer)->AsgardStateTx{
        let mut hash_map = HashMap::new();
        hash_map.insert(peer.get_peer_id(),peer);
        let (tx,rx) = mpsc::channel(128);
        let out_tx = tx.clone();
        let asgard_state = Self{
            current_reign: 0,
            voted_for: None,
            leader_id: None,
            commit_index: 0,
            latest_log_index: 0,
            logs: vec![],
            number_of_peers:1,
            peers: hash_map,
            asgard_state_tx: tx,
        };
        tokio::spawn(
            asgard_state.event_handler(asgard_state,rx).await
        );
        out_tx
    }
}