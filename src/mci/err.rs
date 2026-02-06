//! # MCI Error Types
//!
//! This module defines error types for MCI controller operations.

use super::RegError;

/// MCI controller error enumeration.
#[derive(Debug)]
pub enum MCIError {
    /// Operation timeout
    Timeout,
    /// Controller not initialized
    NotInit,
    /// Buffer too short
    ShortBuf,
    /// Operation not supported
    NotSupport,
    /// Invalid controller state
    InvalidState,
    /// Transfer timeout
    TransTimeout,
    /// Command timeout
    CmdTimeout,
    /// No card detected
    NoCard,
    /// Controller busy
    Busy,
    /// DMA buffer not aligned
    DmaBufUnalign,
    /// Invalid timing configuration
    InvalidTiming,
}

impl RegError for MCIError {
    /// Create a timeout error
    fn timeout() -> Self {
        MCIError::Timeout
    }
}

/// Result type for MCI operations.
pub type MCIResult<T = ()> = Result<T, MCIError>;
