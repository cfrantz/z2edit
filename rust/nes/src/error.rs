use thiserror::Error;

use super::AddressRange;

#[derive(Debug, Error)]
pub enum NesError {
    #[error("invalid address")]
    InvalidAddress,
    #[error("negative bank not supported")]
    NegativeBank,
    #[error("cannot convert: {0}")]
    Convert(String),
    #[error("overlap error: {0}")]
    Overlaps(String),
    #[error("address range {0:x?} too small; wanted {1}")]
    RangeTooSmall(AddressRange, u16),
    #[error("no freespace available: {0}")]
    NoMemory(String),
    #[error("unsupported mapper: {0}")]
    UnsupportedMapper(u16),
    #[error("invalid nes file: {0}")]
    InvalidNesFile(String),
}
