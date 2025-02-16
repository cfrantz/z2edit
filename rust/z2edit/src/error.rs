use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("cast error: {0}")]
    Cast(String),
    #[error("not found error: {0}")]
    NotFound(String),
    #[error("not implemented: {0}")]
    NotImplemented(String),
    #[error("configuration error: {0}")]
    Configuration(String),
    #[error("map error: {0}")]
    Map(String),
}
