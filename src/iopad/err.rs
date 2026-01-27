//! IOPAD error types.
//!
//! This module defines error types for IOPAD operations.

use crate::regs::RegError;

/// Errors that can occur during IOPAD operations.
#[derive(Debug)]
pub enum FioPadError {
    /// Invalid parameter provided to an IOPAD function
    InvalParam,
    /// IOPAD device is not ready for operation
    NotReady,
    /// Requested operation is not supported
    NotNotSupport,
    /// Operation timed out
    Timeout,
}

impl RegError for FioPadError {
    /// Returns a timeout error
    fn timeout() -> Self {
        FioPadError::Timeout
    }
}

/// Result type for IOPAD operations
pub type FioPadResult<T = ()> = Result<T, FioPadError>;
