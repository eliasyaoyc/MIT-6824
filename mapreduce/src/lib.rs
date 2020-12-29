mod master;
mod mrsequential;

pub use mrsequential::sequential;

mod utils;

pub use utils::*;

mod mrwc;

pub use mrwc::{map, reduce};

mod common_map;
mod common_reduce;