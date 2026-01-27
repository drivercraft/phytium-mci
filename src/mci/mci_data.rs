//! Data transfer structures for MCI operations
//!
//! This module defines the data structures used for transferring data
//! between the host and SD/MMC cards.

use alloc::vec::Vec;

/// Data transfer structure for MCI operations
///
/// Represents a data buffer with associated metadata for block transfers.
#[derive(Debug, Clone)]
pub(crate) struct MCIData {
    // todo Using smart pointers involves extensive fine-tuning and code modifications,
    // and can easily cause potential illegal memory access
    // Temporarily not considering using pointers to represent buf here as in the source code
    buf: Option<Vec<u32>>,
    buf_dma: usize,
    blksz: u32,
    blkcnt: u32,
    datalen: u32,
}

impl MCIData {
    pub(crate) fn new() -> Self {
        MCIData {
            buf: None,
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

    pub(crate) fn buf_take(&mut self) -> Option<Vec<u32>> {
        self.buf.take()
    }

    pub(crate) fn buf_set(&mut self, buf: Option<Vec<u32>>) {
        self.buf = buf
    }

    pub(crate) fn buf_dma(&self) -> usize {
        self.buf_dma
    }

    pub(crate) fn buf_dma_set(&mut self, buf_dma: usize) {
        self.buf_dma = buf_dma;
    }
}
