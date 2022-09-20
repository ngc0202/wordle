use std::{error::Error, fmt::Display, io};

#[derive(Copy, Clone, Debug)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ParseError")
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub enum LoadError {
    ParseError,
    IoError(io::Error),
}

impl From<ParseError> for LoadError {
    fn from(_: ParseError) -> Self {
        Self::ParseError
    }
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError => f.write_str("ParseError"),
            Self::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for LoadError {}

pub type LoadResult<T> = Result<T, LoadError>;
