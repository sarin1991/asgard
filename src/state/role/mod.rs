struct Usurper{
    election_count: u16;
    peer_received_vote_map: HashMap<PeerID,bool>;
}

struct Rebel{
    election_count: u16;
    peer_received_vote_map: HashMap<PeerID,bool>;
}

struct Follower{
    min_heap_log_messages: BinaryHeap<Reverse<u64>>;
}

struct Spy{

}

struct Exile{

}