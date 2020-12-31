mod master;
mod slave;
mod persisener;
mod api;
mod raft_proto;
mod error;

pub enum machine_state {

}

pub enum raft_role {
    Leaders,
    Candidates,
    Followers,
}


pub struct record {}

impl record {
    pub fn new() -> Self {

    }
}
