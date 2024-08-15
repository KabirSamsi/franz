use std::io;

#[derive(Debug)]
pub enum FranzError {
    FlattenError,
    ParseError,
    UnboundError,
    IO(io::Error)
}

pub type FranzResult<T> = Result<T, FranzError>;
