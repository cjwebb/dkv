pub enum Gossip {
    Heartbeat,
    NodeJoin { node_id: u64 },
    NodeLeave { node_id: u64 },
}
