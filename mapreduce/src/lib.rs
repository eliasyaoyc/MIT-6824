mod master;

pub use master::run_master;

mod mrsequential;

pub use mrsequential::sequential;

mod utils;

pub use utils::*;

mod mrwc;

pub use mrwc::{map, reduce};

mod common_map;
mod common_reduce;
mod worker;
mod common_rpc;

pub use worker::run_worker;