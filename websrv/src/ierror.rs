use crate::threadpool::Job;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io::Error as IoError;
use std::str::Utf8Error;
use std::sync::PoisonError;
use std::time::SystemTimeError;
#[derive(Debug)]
pub enum IError {
    IOError(IoError),
    Utf8Error(Utf8Error),
    ParseArgError(&'static str),
    InvalidRequestError,
    PoolCreationError,
    SystemTimeError(SystemTimeError),
    PoisonError(PoisonError<Job>),
}

impl Display for IError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            IError::IOError(e) => e.fmt(f),
            IError::Utf8Error(e) => e.fmt(f),
            IError::ParseArgError(e) => e.fmt(f),
            IError::InvalidRequestError => write!(f, "Invalid request format"),
            IError::PoolCreationError => write!(f, "Ivalid create thread number"),
            IError::SystemTimeError(e) => e.fmt(f),
            IError::PoisonError(e) => e.fmt(f),
        }
    }
}

impl Error for IError {}

impl From<IoError> for IError {
    fn from(value: IoError) -> Self {
        IError::IOError(value)
    }
}

impl From<Utf8Error> for IError {
    fn from(value: Utf8Error) -> Self {
        IError::Utf8Error(value)
    }
}

impl From<&'static str> for IError {
    fn from(value: &'static str) -> Self {
        IError::ParseArgError(value)
    }
}

impl From<SystemTimeError> for IError {
    fn from(value: SystemTimeError) -> Self {
        IError::SystemTimeError(value)
    }
}

impl From<PoisonError<Job>> for IError {
    fn from(value: PoisonError<Job>) -> Self {
        IError::PoisonError(value)
    }
}
