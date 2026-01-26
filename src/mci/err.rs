//! Error types for MCI operations
//!
//! This module defines the error types and result type used throughout
//! the MCI driver for error handling.

use super::RegError;

/// Errors that can occur during MCI operations
#[derive(Debug)]
pub enum MCIError {
    /// Operation timed out
    Timeout,
    /// Device not initialized
    NotInit,
    /// Buffer too short for operation
    ShortBuf,
    /// Operation not supported
    NotSupport,
    /// Device in invalid state
    InvalidState,
    /// Data transfer timeout
    TransTimeout,
    /// Command timeout
    CmdTimeout,
    /// No card detected
    NoCard,
    /// Device busy
    Busy,
    /// DMA buffer not properly aligned
    DmaBufUnalign,
    /// Invalid timing configuration
    InvalidTiming,
}

impl RegError for MCIError {
    fn timeout() -> Self {
        MCIError::Timeout
    }
}

/// Result type for MCI operations
pub type MCIResult<T = ()> = Result<T, MCIError>;
