//! # OSA Error Types
//!
//! This module defines error types for memory pool operations.

/// Memory pool error enumeration.
#[derive(Debug)]
pub enum FMempError {
    /// Invalid buffer
    InvalidBuf,
    /// TLSF initialization error
    InitTlsfError,
    /// Memory allocation failed
    BadMalloc,
    // PoolBuffer related errors
    /// PoolBuffer size too small to copy contents from a slice
    NotEnoughSpace,
    /// PoolBuffer size isn't aligned to size::T
    SizeNotAligned,
}

/// Result type for memory pool operations.
#[allow(unused)]
pub type FMempStatus<T = ()> = Result<T, FMempError>;
