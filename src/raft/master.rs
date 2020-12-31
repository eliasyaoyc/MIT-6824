use crate::raft::raft_proto::Record;

pub struct master {
    current_term: u32,
    voted_for: String,
    logs: Vec<Record>,
    commit_index: u32,
    last_applied: u32,
    next_index: Vec<u32>,
    match_index: Vec<u32>,
}

impl master {
    pub fn new() -> master {
        master {
            current_term: 0,
            voted_for: "".to_string(),
            logs: vec![],
            commit_index: 0,
            last_applied: 0,
            next_index: vec![],
            match_index: vec![],
        }
    }
}