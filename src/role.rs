pub(crate) enum RoleFlag {
    LeaderFlag,
    FollowerFlag,
    CandidateFlag,
    ImmigrantFlag,
    ExileFlag,
}

pub(crate)  struct Rebel{

}

pub(crate) struct Leader{

}

impl Leader {
    fn ()
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

pub(crate) enum RoleData {
    Leader(Leader),
    Follower(Follower),
    Candidate(Candidate),
    Immigrant(Immigrant),
    Exile(Exile),
}

impl RoleData {
    pub(crate) fn new() -> Self {
        let immigrant = Immigrant::new();
        RoleData::Immigrant(immigrant)
    }
}