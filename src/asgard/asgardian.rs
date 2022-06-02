use tokio::sync::mpsc::{Sender,Receiver};

struct Asgardian {
    asgard_state_tx: AsgardStateTx,
    asgard_state_join_handle: tokio::task::JoinHandle<()>,
    role_tx: RoleTx,
    role_join_handle: tokio::task::JoinHandle<()>,
    message_receiver: Receiver<Message>,
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