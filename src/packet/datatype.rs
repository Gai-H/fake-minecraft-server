use std::{error, fmt};

pub mod long;
pub mod string;
pub mod unsigned_short;
pub mod uuid;
pub mod varint;

#[derive(Debug, PartialEq)]
pub enum DatatypeError {
    ReadError,
    ConvertError,
    TooLongStringError,
}

impl fmt::Display for DatatypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DatatypeError::ReadError => write!(f, "Could not read bytes from stream."),
            DatatypeError::ConvertError => write!(f, "Could not convert bytes."),
            DatatypeError::TooLongStringError => write!(f, "String is too long."),
        }
    }
}

impl error::Error for DatatypeError {}
