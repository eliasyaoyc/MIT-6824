use crate::raft::raft_proto::Record;

pub struct slave {
    current_term: u32,
    voted_for: String,
    logs: Vec<Record>,
    commit_index: u32,
    last_applied: u32,
}

impl slave {
    pub fn new() -> Self{
        slave{
            current_term: 0,
            voted_for: "".to_string(),
            logs: vec![],
            commit_index: 0,
            last_applied: 0
        }
    }
}