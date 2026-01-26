//! OSA (OS Abstraction) error types.
//!
//! This module defines error types for memory pool and buffer operations.

/// Errors that can occur during memory pool or buffer operations.
#[derive(Debug)]
pub enum FMempError {
    /// Invalid buffer pointer or size
    InvalidBuf,
    /// TLSF memory pool initialization failed
    InitTlsfError,
    /// Memory allocation failed (out of memory)
    BadMalloc,
    /// PoolBuffer size too small to copy contents from a slice
    NotEnoughSpace,
    /// PoolBuffer size is not aligned to the requested type size
    SizeNotAligned,
}

/// Result type for memory pool operations
#[allow(unused)]
pub type FMempStatus<T = ()> = Result<T, FMempError>;
