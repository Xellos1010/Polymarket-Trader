use thiserror::Error;

#[derive(Debug, Error)]
pub enum PtError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("http error: {0}")]
    Http(String),
    #[error("serialization error: {0}")]
    Serde(String),
    #[error("risk violation: {0}")]
    Risk(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

pub type PtResult<T> = Result<T, PtError>;
