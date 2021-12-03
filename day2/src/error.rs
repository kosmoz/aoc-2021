use std::{
    fmt::{self, Debug},
    io,
    num::ParseIntError,
};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ParseError(ParseIntError),
    CustomError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(err: core::num::ParseIntError) -> Self {
        Error::ParseError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(inner) => write!(f, "{}", inner),
            Error::ParseError(inner) => write!(f, "{}", inner),
            Error::CustomError(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {}
