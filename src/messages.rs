pub struct AsgardianMessage{

}

pub struct APIMessage{

}

pub enum Message{
    AsgardianMessage(AsgardianMessage),
    APIMessage(APIMessage),
}