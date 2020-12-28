use std::{result};

#[derive(Fail, Debug)]
pub enum mrError {

    #[fail(display = "env input args error")]
    CommandLineError,

    #[fail(display = "function map occur error")]
    MapFuncError,

    #[fail(display = "function reduce occur error")]
    ReduceFuncError,

    #[fail(display = "unexpected error")]
    UnexpectedError,
}

pub type Result<T> = result::Result<T, mrError>;