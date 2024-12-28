use thiserror::Error;

#[derive(Debug, Error)]
pub enum NesError {
    #[error("Invalid Address")]
    InvalidAddress,
}
