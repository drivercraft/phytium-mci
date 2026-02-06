//! # MCI Card Base Structure
//!
//! This module provides the base structure for SD/MMC card operations.
//! It contains common fields and methods used by all card types.

use crate::osa::pool_buffer::PoolBuffer;

use super::MCIHost;

/// Base structure for SD/MMC card operations.
///
/// This structure contains common fields used by all card types including
/// host controller reference, status flags, and operational parameters.
pub(crate) struct MCICardBase {
    /// Host controller reference
    pub host: Option<MCIHost>,
    /// Whether the host controller is ready
    pub is_host_ready: bool,
    /// No internal alignment required
    pub no_interal_align: bool,
    /// Internal buffer for data transfer
    pub internal_buffer: PoolBuffer,
    /// Current bus clock frequency in Hz
    pub bus_clk_hz: u32,
    /// Card relative address
    pub relative_address: u32,
    /// Operation Condition Register value
    pub ocr: u32,
    /// Card block size in bytes
    pub block_size: u32,
}

impl MCICardBase {
    /// Create a new card base structure from a buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Pool buffer for internal data transfer
    pub fn from_buffer(buffer: PoolBuffer) -> Self {
        MCICardBase {
            host: None,
            is_host_ready: false,
            no_interal_align: false,
            internal_buffer: buffer,
            bus_clk_hz: 0,
            relative_address: 0,
            ocr: 0,
            block_size: 0,
        }
    }

    /// Get the card block size.
    pub fn block_size(&self) -> u32 {
        self.block_size
    }
}
