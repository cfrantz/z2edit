use thiserror::Error;

#[derive(Debug, Error)]
pub enum NesError {
    #[error("invalid address")]
    InvalidAddress,
    #[error("negative bank not supported")]
    NegativeBank,
}
