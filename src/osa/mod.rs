//! # Operating System Abstraction Layer (OSA)
//!
//! This module provides memory management functionality for the driver.
//! It implements a memory pool using the TLSF (Two-Level Segregated Fit) algorithm.
//!
//! ## Components
//!
//! - **FMemp**: Memory pool manager using TLSF
//! - **PoolBuffer**: Safe wrapper for aligned memory allocations
//!
//! ## Usage
//!
//! ```rust
//! use phytium_mci::osa::{osa_init, osa_alloc_aligned};
//!
//! // Initialize the memory pool
//! osa_init();
//!
//! // Allocate aligned memory
//! let buffer = osa_alloc_aligned(4096, 512).unwrap();
//! ```

#![deny(missing_docs)]
use core::{alloc::Layout, mem::MaybeUninit, ptr::NonNull};

use consts::MAX_POOL_SIZE;
use err::FMempError;
use lazy_static::*;
use pool_buffer::PoolBuffer;
use rlsf::Tlsf;
use spin::Mutex;

mod consts;
mod err;
pub mod pool_buffer;

/// Memory managed by Tlsf pool
static mut POOL: [MaybeUninit<u8>; MAX_POOL_SIZE] = [MaybeUninit::uninit(); MAX_POOL_SIZE];

/// Tlsf memory pool controller.
///
/// This structure manages the memory pool using the TLSF algorithm
/// for efficient allocation and deallocation of variable-sized blocks.
pub struct FMemp<'a> {
    tlsf_ptr: Tlsf<'a, u32, u32, 32, 32>,
    /// Whether the pool is initialized
    is_ready: bool,
}

lazy_static! {
    /// Global memory pool manager
    pub static ref GLOBAL_FMEMP: Mutex<FMemp<'static>> =
        Mutex::new(FMemp::new());
}

impl<'a> FMemp<'a> {
    /// Constructor
    pub fn new() -> Self {
        Self {
            tlsf_ptr: Tlsf::new(),
            is_ready: false,
        }
    }

    unsafe fn init(&mut self) {
        self.tlsf_ptr.insert_free_block(&mut POOL[..]);
        self.is_ready = true;
    }

    unsafe fn alloc_aligned(
        &mut self,
        size: usize,
        align: usize,
    ) -> Result<PoolBuffer, FMempError> {
        let layout = Layout::from_size_align_unchecked(size, align);
        if let Some(result) = self.tlsf_ptr.allocate(layout) {
            let buffer = PoolBuffer::new(size, result);
            Ok(buffer)
        } else {
            Err(FMempError::BadMalloc)
        }
    }

    unsafe fn dealloc(&mut self, addr: NonNull<u8>, size: usize) {
        self.tlsf_ptr.deallocate(addr, size);
    }
}

/// Init memory pool with size of ['MAX_POOL_SIZE']
pub fn osa_init() {
    unsafe {
        GLOBAL_FMEMP.lock().init();
    }
}

/// Alloc 'size' bytes space, aligned to 64 KiB by default
pub fn osa_alloc(size: usize) -> Result<PoolBuffer, FMempError> {
    unsafe { GLOBAL_FMEMP.lock().alloc_aligned(size, size_of::<usize>()) }
}

/// Alloc 'size' bytes space, aligned to 'align' bytes
pub fn osa_alloc_aligned(size: usize, align: usize) -> Result<PoolBuffer, FMempError> {
    unsafe { GLOBAL_FMEMP.lock().alloc_aligned(size, align) }
}

/// Dealloc 'size' bytes space from 'addr'
pub fn osa_dealloc(addr: NonNull<u8>, size: usize) {
    unsafe {
        GLOBAL_FMEMP.lock().dealloc(addr, size);
    }
}
