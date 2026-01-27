//! OS Abstraction Layer (OSA).
//!
//! This module provides OS abstraction services for the phytium-mci driver,
//! including memory management and event synchronization. It is designed to
//! work in `no_std` environments without requiring an underlying operating system.
//!
//! # Components
//!
//! - **Memory Pool** - TLSF-based allocator for DMA buffers
//! - **PoolBuffer** - Safe wrapper for aligned memory allocations
//! - **Event Flags** - Synchronization primitives for interrupt handling
//!
//! # Memory Management
//!
//! The module uses a TLSF (Two-Level Segregated Fit) memory allocator
//! optimized for embedded systems:
//! - **Fast allocation/deallocation** - O(1) operations
//! - **Low fragmentation** - Efficient memory utilization
//! - **Aligned allocation** - Support for DMA-aligned buffers
//!
//! # Example
//!
//! ```rust,ignore
//! use phytium_mci::osa::{osa_init, PoolBuffer};
//!
//! // Initialize the memory pool (call once at startup)
//! osa_init();
//!
//! // Allocate an aligned buffer
//! let buffer = PoolBuffer::new(4096, 512)?;
//!
//! // Use the buffer...
//!
//! // Buffer is automatically freed when dropped
//! ```
//!
//! # Global Memory Pool
//!
//! The driver uses a global memory pool of 64KB (configurable via `MAX_POOL_SIZE`)
//! for all DMA operations. This pool must be initialized before any card operations.

use core::{
    alloc::Layout,
    mem::MaybeUninit,
    ptr::NonNull,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
    time::Duration,
};

use alloc::boxed::Box;
use consts::MAX_POOL_SIZE;
use err::FMempError;
use lazy_static::*;
use rlsf::Tlsf;
use spin::Mutex;

use crate::sleep;

pub mod consts;
mod err;
pub mod pool_buffer;

/// Memory managed by TLSF pool.
///
/// This static array serves as the backing storage for the global memory pool.
/// The size is determined by `MAX_POOL_SIZE` (default: 64KB).
static mut POOL: [MaybeUninit<u8>; MAX_POOL_SIZE] = [MaybeUninit::uninit(); MAX_POOL_SIZE];

/// TLSF memory pool controller.
///
/// `FMemp` manages a memory pool using the TLSF (Two-Level Segregated Fit)
/// allocation algorithm, which provides O(1) allocation and deallocation
/// with low fragmentation.
///
/// # Type Parameters
///
/// * `'a` - Lifetime of the memory pool
pub struct FMemp<'a> {
    tlsf_ptr: Tlsf<'a, u32, u32, 32, 32>,
    is_ready: bool,
}

lazy_static! {
    /// Global memory pool manager.
    ///
    /// This is the global instance of the TLSF memory allocator used for
    /// all DMA buffer allocations in the driver.
    pub static ref GLOBAL_FMEMP: Mutex<Box<FMemp<'static>>> =
        Mutex::new(Box::new(FMemp::new()));
}

impl<'a> FMemp<'a> {
    /// Creates a new TLSF memory pool controller.
    pub fn new() -> Self {
        Self {
            tlsf_ptr: Tlsf::new(),
            is_ready: false,
        }
    }

    /// Initializes the memory pool.
    ///
    /// # Safety
    ///
    /// This function must be called once before any memory operations.
    /// It initializes the TLSF allocator with the global memory pool.
    unsafe fn init(&mut self) {
        unsafe { self.tlsf_ptr.insert_free_block(&mut POOL[..]) };
        self.is_ready = true;
    }

    /// Allocates aligned memory from the pool.
    ///
    /// # Arguments
    ///
    /// * `size` - Size of the allocation in bytes
    /// * `align` - Alignment requirement in bytes
    ///
    /// # Returns
    ///
    /// Pointer to the allocated memory, or an error if allocation fails
    ///
    /// # Safety
    ///
    /// The returned pointer must be freed with `dealloc` to avoid memory leaks.
    unsafe fn alloc_aligned(
        &mut self,
        size: usize,
        align: usize,
    ) -> Result<NonNull<u8>, FMempError> {
        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
        if let Some(result) = self.tlsf_ptr.allocate(layout) {
            Ok(result)
        } else {
            Err(FMempError::BadMalloc)
        }
    }

    /// Deallocates memory previously allocated with `alloc_aligned`.
    ///
    /// # Arguments
    ///
    /// * `addr` - Pointer to the memory to deallocate
    /// * `size` - Size of the allocation in bytes
    ///
    /// # Safety
    ///
    /// `addr` must be a pointer previously returned by `alloc_aligned`
    /// and not yet freed.
    unsafe fn dealloc(&mut self, addr: NonNull<u8>, size: usize) {
        unsafe { self.tlsf_ptr.deallocate(addr, size) };
    }
}

impl<'a> Default for FMemp<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Initializes the global memory pool.
///
/// This function must be called once at startup before any SD/MMC
/// operations that require DMA buffers.
///
/// # Example
///
/// ```rust,ignore
/// use phytium_mci::osa::osa_init;
///
/// // Call once during system initialization
/// osa_init();
/// ```
pub fn osa_init() {
    unsafe {
        GLOBAL_FMEMP.lock().init();
    }
}

/// Allocates memory from the global pool.
///
/// # Arguments
///
/// * `size` - Size of the allocation in bytes
///
/// # Returns
///
/// Pointer to the allocated memory, or an error if allocation fails
///
/// # Alignment
///
/// Memory is aligned to the system word size (`sizeof::<usize>()`).
pub fn osa_alloc(size: usize) -> Result<NonNull<u8>, FMempError> {
    unsafe { GLOBAL_FMEMP.lock().alloc_aligned(size, size_of::<usize>()) }
}

/// Alloc 'size' bytes space, aligned to 'align' bytes
pub fn osa_alloc_aligned(size: usize, align: usize) -> Result<NonNull<u8>, FMempError> {
    unsafe { GLOBAL_FMEMP.lock().alloc_aligned(size, align) }
}

/// Dealloc 'size' bytes space from 'addr'
pub fn osa_dealloc(addr: NonNull<u8>, size: usize) {
    unsafe {
        GLOBAL_FMEMP.lock().dealloc(addr, size);
    }
}

/// Event flag synchronization primitive.
///
/// `OSAEvent` provides a simple event flag mechanism for thread synchronization
/// in `no_std` environments. It supports setting, waiting, and clearing event flags.
///
/// This is typically used for interrupt synchronization where interrupt handlers
/// set flags and main code waits for them.
pub struct OSAEvent {
    event_flag: AtomicU32,
    notification: AtomicBool,
}

impl Default for OSAEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl OSAEvent {
    /// Creates a new event flag instance.
    ///
    /// # Returns
    ///
    /// A new `OSAEvent` with all flags cleared
    pub const fn new() -> Self {
        Self {
            event_flag: AtomicU32::new(0),
            notification: AtomicBool::new(false),
        }
    }

    /// Sets the specified event flags.
    ///
    /// This is typically called from interrupt handlers to signal that
    /// an event has occurred.
    ///
    /// # Arguments
    ///
    /// * `event_type` - Event flags to set (bitmask)
    pub fn osa_event_set(&self, event_type: u32) {
        self.event_flag.fetch_or(event_type, Ordering::SeqCst);
        self.notification.store(true, Ordering::Release);
    }

    /// Waits for the specified event flags with a timeout.
    ///
    /// This spins until one or more of the specified event flags are set,
    /// or the timeout expires.
    ///
    /// # Arguments
    ///
    /// * `event_type` - Event flags to wait for (bitmask)
    /// * `timeout_ticks` - Maximum time to wait in arbitrary ticks
    ///
    /// # Returns
    ///
    /// The current event flags if signaled, or an error if timeout occurred
    pub fn osa_event_wait(&self, event_type: u32, timeout_ticks: u32) -> Result<u32, &'static str> {
        let mut ticks = 0;

        loop {
            if self.notification.load(Ordering::Acquire) {
                let events = self.event_flag.load(Ordering::SeqCst);
                if events & event_type != 0 {
                    self.notification.store(false, Ordering::Release);
                    return Ok(events);
                }
            }

            if ticks >= timeout_ticks {
                return Err("timeout");
            }

            ticks += 1;
            #[cfg(feature = "pio")]
            sleep(Duration::from_millis(2));

            core::hint::spin_loop();
        }
    }

    /// Clears the specified event flags.
    ///
    /// # Arguments
    ///
    /// * `event_type` - Event flags to clear (bitmask)
    pub fn osa_event_clear(&self, event_type: u32) {
        self.event_flag.fetch_and(!event_type, Ordering::SeqCst);

        if self.event_flag.load(Ordering::SeqCst) == 0 {
            self.notification.store(false, Ordering::Release);
        }
    }

    /// Gets the current event flags.
    ///
    /// # Returns
    ///
    /// Current event flags as a bitmask
    pub fn osa_event_get(&self) -> u32 {
        self.event_flag.load(Ordering::SeqCst)
    }
}

lazy_static! {
    /// Global event flag instance for synchronization.
    static ref OSA_EVENT: OSAEvent = OSAEvent::new();
}

/// Sets the specified event flags.
///
/// # Arguments
///
/// * `event_type` - Event flags to set (bitmask)
pub fn osa_event_set(event_type: u32) {
    OSA_EVENT.osa_event_set(event_type);
}

/// Waits for the specified event flags with a timeout.
///
/// # Arguments
///
/// * `event_type` - Event flags to wait for (bitmask)
/// * `timeout` - Maximum time to wait in milliseconds
///
/// # Returns
///
/// `Ok(())` if the event was signaled, or an error if timeout occurred
pub fn osa_event_wait(event_type: u32, timeout: u32) -> Result<(), &'static str> {
    OSA_EVENT.osa_event_wait(event_type, timeout).map(|_| ())
}

/// Gets the current event flags.
///
/// # Returns
///
/// Current event flags as a bitmask
pub fn osa_event_get() -> u32 {
    OSA_EVENT.osa_event_get()
}

/// Clears the specified event flags.
///
/// # Arguments
///
/// * `event_type` - Event flags to clear (bitmask)
pub fn osa_event_clear(event_type: u32) {
    OSA_EVENT.osa_event_clear(event_type);
}
