use std::io;

pub enum FranzError {
    FlattenError,
    IO(io::Error)
}

pub type FranzResult<T> = Result<T, FranzError>;
