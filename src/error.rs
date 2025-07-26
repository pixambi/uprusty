use std::fmt;

#[derive(Debug)]
pub enum Error {
    Client(crate::client::ClientError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Client(e) => write!(f, "Client error: {:?}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<crate::client::ClientError> for Error {
    fn from(e: crate::client::ClientError) -> Self {
        Error::Client(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
