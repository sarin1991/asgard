use crate::protobuf_messages::asgard_messages::GenericAsgardMessage;
use tokio::sync::mpsc::{Sender,Receiver};

type AsgardTx = Sender<GenericAsgardMessage>;
type AsgardRx = Receiver<GenericAsgardMessage>;