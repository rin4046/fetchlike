use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    HyperError(hyper::Error),
    #[error("{0}")]
    SerdeJsonError(serde_json::Error),
    #[error("{0}")]
    FromUtf8Error(std::string::FromUtf8Error),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self::HyperError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(err)
    }
}
