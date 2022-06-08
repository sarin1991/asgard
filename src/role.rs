pub(crate)  struct Rebel{

}

pub(crate) struct Leader{

}

pub(crate) struct Follower{
    rebel: Rebel,
}

pub(crate) struct Candidate{
    rebel: Rebel,
}

pub(crate)  struct Exile{

}

pub(crate)  struct Immigrant{

}

impl Immigrant {
    pub(crate) fn new() -> Self {
        Self{}
    }
}

pub(crate) enum Role {
    Leader(Leader),
    Follower(Follower),
    Candidate(Candidate),
    Immigrant(Immigrant),
    Exile(Exile),
}

impl Role {
    pub(crate) fn new() -> Self {
        let immigrant = Immigrant::new();
        Role::Immigrant(immigrant)
    }
}