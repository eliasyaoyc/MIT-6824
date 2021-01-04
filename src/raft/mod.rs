mod master;
mod slave;
mod persisener;
mod api;
mod raft_proto;
mod errors;
mod config;
mod service;
mod tests;

pub enum machine_state {

}

pub enum raft_role {
    Leaders,
    Candidates,
    Followers,
}
