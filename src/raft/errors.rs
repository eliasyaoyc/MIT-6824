use std::result;
use std::fmt;
use serde::export::Formatter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NotLeader,
    Timeout,
}

impl Error {
    pub fn get_str(&self) -> &'static str {
        match *self {
            Error::Timeout => "rpc timeout",
            Error::NotLeader => "no leader",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.get_str())
    }
}

pub type Result = result::Result<(), Error>;