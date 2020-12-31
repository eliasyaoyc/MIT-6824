use std::result;

pub enum raft_error {

}


pub type Result = result<(), raft_error>;