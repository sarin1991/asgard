enum Role {
    King(King),
    Follower(Follower),
    Usurper(Usurper),
    Rebel(Rebel),
    Immigrant(Immigrant),
    Exile(Exile),
}

impl Role {
    fn new()->AsgardianTx{
        TODO:Implementation
    }
    async fn event_handler(Self){
        loop{
            match Self {
                Role::King(king_role) => Self=king_role.event_handler(rx).await,
                Role::Follower(follower_role) => Self=follower_role.event_handler(rx).await,
                Role::Usurper(usurper_role) => Self=usurper_role.event_handler(rx).await,
                Role::Rebel(rebel_role) => Self=rebel_role.event_handler(rx).await,
                Role::Immigrant(immigrant_role) => immigrant_role.event_handler(rx).await,
                Role::Exile(exile_role) => break,
            }
        }
    }
}