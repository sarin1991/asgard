use tokio::sync::mpsc::{Sender,Receiver};
use crate::transport::TransportChannel;

struct Asgardian {
    transport_channel:TransportChannel,
}

impl Asgardian{
    fn new()->Self{

    }
    async fn start(&mut self){
        while let message = self.message_receiver.recv().await.unwrap() {
            let mut role = self.role.take().unwrap();
            let new_role : Role = role.handle_message(self.asgard_state.clone(),message).await;
            match role{
                Role::Exile(exile) => break,
                _ => (),
            }
            self.role = Some(new_role);
        }
    }
}