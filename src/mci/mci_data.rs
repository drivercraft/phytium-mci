//! # MCI Data Transfer Structure
//!
//! This module provides the data transfer structure for MCI operations.

use alloc::vec::Vec;

/// MCI data transfer structure.
///
/// This structure contains information about data transfers including
/// buffer pointers, block size, block count, and data length.
#[derive(Debug, Clone)]
pub(crate) struct MCIData {
    //TODO Will using smart pointers affect performance?
    /// Data buffer (CPU address)
    buf: Option<Vec<u32>>,
    /// Data buffer DMA address
    #[cfg(feature = "dma")]
    buf_dma: usize,
    /// Block size
    blksz: u32,
    /// Block count
    blkcnt: u32,
    /// Total data length
    datalen: u32,
}

impl MCIData {
    pub(crate) fn new() -> Self {
        MCIData {
            buf: None,
            #[cfg(feature = "dma")]
            buf_dma: 0,
            blksz: 0,
            blkcnt: 0,
            datalen: 0,
        }
    }

    pub(crate) fn blksz(&self) -> u32 {
        self.blksz
    }

    pub(crate) fn blksz_set(&mut self, blksz: u32) {
        self.blksz = blksz
    }

    pub(crate) fn blkcnt(&self) -> u32 {
        self.blkcnt
    }

    pub(crate) fn blkcnt_set(&mut self, blkcnt: u32) {
        self.blkcnt = blkcnt
    }

    pub(crate) fn datalen(&self) -> u32 {
        self.datalen
    }

    pub(crate) fn datalen_set(&mut self, datalen: u32) {
        self.datalen = datalen
    }

    pub(crate) fn buf(&self) -> Option<&Vec<u32>> {
        self.buf.as_ref()
    }

    pub(crate) fn buf_mut(&mut self) -> Option<&mut Vec<u32>> {
        self.buf.as_mut()
    }

    pub(crate) fn buf_set(&mut self, buf: Option<Vec<u32>>) {
        self.buf = buf
    }

    #[cfg(feature = "dma")]
    pub(crate) fn buf_dma(&self) -> usize {
        self.buf_dma
    }

    #[cfg(feature = "dma")]
    pub(crate) fn buf_dma_set(&mut self, buf_dma: usize) {
        self.buf_dma = buf_dma;
    }
}
