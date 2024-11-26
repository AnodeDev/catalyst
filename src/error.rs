use std::fmt;
use x11rb::errors;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ConnectError(errors::ConnectError),
    ConnectionError(errors::ConnectionError),
    ReplyOrIdError(errors::ReplyOrIdError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<errors::ConnectError> for Error {
    fn from(error: errors::ConnectError) -> Self {
        Error::ConnectError(error)
    }
}

impl From<errors::ConnectionError> for Error {
    fn from(error: errors::ConnectionError) -> Self {
        Error::ConnectionError(error)
    }
}

impl From<errors::ReplyOrIdError> for Error {
    fn from(error: errors::ReplyOrIdError) -> Self {
        Error::ReplyOrIdError(error)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "{}", e),
            Error::ConnectError(e) => write!(f, "{}", e),
            Error::ConnectionError(e) => write!(f, "{}", e),
            Error::ReplyOrIdError(e) => write!(f, "{}", e),
        }
    }
}

