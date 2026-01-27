#![no_std]
#![deny(warnings, missing_docs)]

//! # phytium-mci
//!
//! A `no_std` SD/MMC host controller driver for Phytium E2000 series SoCs.
//!
//! ## Overview
//!
//! This crate provides a comprehensive driver for SD/MMC cards on Phytium platforms,
//! supporting both SD and eMMC cards with DMA and PIO transfer modes.
//!
//! ## Features
//!
//! - **Full SD Specification Support**: SDSC, SDHC, SDXC (versions 1.0-3.0)
//! - **eMMC Support**: MMC protocol implementation
//! - **Flexible Transfer Modes**: DMA (high-performance) and PIO (simple)
//! - **Voltage Support**: 3.3V (default) and 1.8V (UHS-I modes)
//! - **Bus Widths**: 1-bit, 4-bit, and 8-bit (eMMC) data bus
//! - **High-Speed Modes**: SDR12, SDR25, SDR50, SDR104, DDR50
//! - **Clock Speed**: From 400 KHz to 208 MHz
//! - **Card Detection**: GPIO-based and host-based
//! - **Interrupt Support**: Command, data, and card detection interrupts
//!
//! ## Architecture
//!
//! The driver is organized into distinct layers:
//!
//! ```text
//! Application Layer    (SdCard, MCIHost - High-level API)
//!        ↓
//! Protocol Layer       (Command/Data transfer, Card initialization)
//!        ↓
//! Hardware Abstraction (Register access, DMA/PIO control)
//!        ↓
//! Hardware Support     (IoPad pin configuration, OSA memory/timing)
//! ```
//!
//! ## Modules
//!
//! - [`iopad`] - I/O pad configuration for pin multiplexing
//! - [`mci`] - Hardware controller driver (register access, DMA/PIO, interrupts)
//! - [`mci_host`] - Host controller protocol layer (SD/MMC protocol)
//! - [`osa`] - OS abstraction layer (memory management, event flags)
//!
//! ## Platform Integration
//!
//! To use this driver, you must implement the [`Kernel`] trait to provide
//! platform-specific functionality:
//!
//! ```rust
//! use phytium_mci::{Kernel, set_impl};
//! use core::{ptr::NonNull, time::Duration};
//!
//! struct MyPlatform;
//!
//! impl Kernel for MyPlatform {
//!     fn sleep(duration: Duration) {
//!         // Platform-specific delay implementation
//!     }
//!
//!     #[cfg(feature = "dma")]
//!     fn mmap(virt_addr: NonNull<u8>) -> u64 {
//!         // Virtual to physical address translation
//!     }
//!
//!     fn flush(addr: NonNull<u8>, size: usize) {
//!         // Cache clean for DMA
//!     }
//!
//!     fn invalidate(addr: NonNull<u8>, size: usize) {
//!         // Cache invalidate for DMA
//!     }
//! }
//!
//! // Register your implementation
//! set_impl!(MyPlatform);
//! ```
//!
//! ## Hardware Configuration
//!
//! Target Hardware:
//! - **SoC**: Phytium E2000 series (ARMv8-A)
//! - **Board**: Phytium Pi development board
//! - **Controller**: Phytium SDIF (Synopsys DesignWare-based)
//! - **MCI0 Base**: 0x2800_1000
//! - **MCI1 Base**: 0x2800_2000
//! - **IOPAD Base**: 0x2800_0000
//!
//! ## Example
//!
//! ```rust,ignore
//! use phytium_mci::{sd::SdCard, IoPad};
//! use core::ptr::NonNull;
//!
//! // Initialize IOPAD
//! let iopad = unsafe { IoPad::new(NonNull::new_unchecked(0x2800_0000 as *mut u8)) };
//!
//! // Create SD card instance
//! let mut sdcard = unsafe {
//!     SdCard::new(
//!         NonNull::new_unchecked(0x2800_1000 as *mut u8),
//!         iopad
//!     )
//! };
//!
//! // Initialize the card
//! sdcard.init(NonNull::new_unchecked(0x2800_1000 as *mut u8))?;
//!
//! // Read blocks
//! let mut buffer = Vec::new();
//! sdcard.read_blocks(&mut buffer, 0, 1)?;
//! ```

extern crate alloc;

use core::{ptr::NonNull, time::Duration};

#[macro_use]
mod regs;
mod aarch;
pub mod iopad;
pub mod mci;
pub mod mci_host;
pub mod osa;
mod tools;

pub use iopad::*;
pub use mci_host::*;

/// Platform abstraction trait for phytium-mci driver.
///
/// This trait must be implemented by the platform to provide platform-specific
/// functionality required by the driver, including delay operations, memory
/// management, and cache operations.
///
/// # Example
///
/// ```rust
/// use phytium_mci::Kernel;
/// use core::{ptr::NonNull, time::Duration};
///
/// struct MyPlatform;
///
/// impl Kernel for MyPlatform {
///     fn sleep(duration: Duration) {
///         // Implement platform-specific delay
///         // e.g., using a timer or busy-waiting
///     }
///
///     #[cfg(feature = "dma")]
///     fn mmap(virt_addr: NonNull<u8>) -> u64 {
///         // Convert virtual address to physical address
///         // Required for DMA operations
///     }
///
///     fn flush(addr: NonNull<u8>, size: usize) {
///         // Clean data cache to ensure data is written to memory
///         // Required before DMA write operations
///     }
///
///     fn invalidate(addr: NonNull<u8>, size: usize) {
///         // Invalidate data cache to ensure fresh data is read
///         // Required after DMA read operations
///     }
/// }
/// ```
pub trait Kernel {
    /// Delay execution for the specified duration.
    ///
    /// This is used for timing delays during card initialization and
    /// command processing.
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration to sleep
    fn sleep(duration: Duration);

    /// Convert virtual address to physical address.
    ///
    /// This is required for DMA operations to provide the physical
    /// address of DMA buffers to the hardware.
    ///
    /// # Arguments
    ///
    /// * `virt_addr` - Virtual address to translate
    ///
    /// # Returns
    ///
    /// Physical address corresponding to the virtual address
    #[cfg(feature = "dma")]
    fn mmap(virt_addr: NonNull<u8>) -> u64;

    /// Clean data cache for the specified memory range.
    ///
    /// Ensures that data in the cache is written back to memory.
    /// This must be called before DMA write operations to ensure
    /// the hardware sees the correct data.
    ///
    /// # Arguments
    ///
    /// * `addr` - Start address of the memory range
    /// * `size` - Size of the memory range in bytes
    fn flush(addr: NonNull<u8>, size: usize);

    /// Invalidate data cache for the specified memory range.
    ///
    /// Discards cached data so that subsequent reads fetch fresh
    /// data from memory. This must be called after DMA read
    /// operations to ensure the CPU sees the data written by the hardware.
    ///
    /// # Arguments
    ///
    /// * `addr` - Start address of the memory range
    /// * `size` - Size of the memory range in bytes
    fn invalidate(addr: core::ptr::NonNull<u8>, size: usize);
}

pub(crate) fn sleep(duration: Duration) {
    unsafe extern "Rust" {
        fn _phytium_mci_sleep(duration: Duration);
    }

    unsafe {
        _phytium_mci_sleep(duration);
    }
}

#[cfg(feature = "dma")]
pub(crate) fn mmap(virt_addr: NonNull<u8>) -> u64 {
    unsafe extern "Rust" {
        fn _phytium_mci_map(virt_addr: NonNull<u8>) -> u64;
    }

    unsafe { _phytium_mci_map(virt_addr) }
}

pub(crate) fn flush(addr: NonNull<u8>, size: usize) {
    unsafe extern "Rust" {
        fn _phytium_mci_flush(addr: NonNull<u8>, size: usize);
    }

    unsafe {
        _phytium_mci_flush(addr, size);
    }
}

pub(crate) fn invalidate(addr: core::ptr::NonNull<u8>, size: usize) {
    unsafe extern "Rust" {
        fn _phytium_mci_invalidate(addr: core::ptr::NonNull<u8>, size: usize);
    }

    unsafe {
        _phytium_mci_invalidate(addr, size);
    }
}

/// Register a platform-specific implementation of the [`Kernel`] trait.
///
/// This macro generates the external interface functions that the phytium-mci
/// driver uses to call platform-specific operations. It should be called once
/// with your platform type that implements the [`Kernel`] trait.
///
/// # Example
///
/// ```rust
/// use phytium_mci::{Kernel, set_impl};
/// use core::{ptr::NonNull, time::Duration};
///
/// struct MyPlatform;
///
/// impl Kernel for MyPlatform {
///     fn sleep(duration: Duration) {
///         // Your implementation
///     }
///
///     #[cfg(feature = "dma")]
///     fn mmap(virt_addr: NonNull<u8>) -> u64 {
///         // Your implementation
///     }
///
///     fn flush(addr: NonNull<u8>, size: usize) {
///         // Your implementation
///     }
///
///     fn invalidate(addr: NonNull<u8>, size: usize) {
///         // Your implementation
///     }
/// }
///
/// // Register the implementation
/// set_impl!(MyPlatform);
/// ```
///
/// # Generated Functions
///
/// This macro generates the following `no_mangle` functions:
/// - `_phytium_mci_sleep` - Sleep/delay functionality
/// - `_phytium_mci_map` - Virtual to physical address translation (DMA only)
/// - `_phytium_mci_flush` - Cache clean operation
/// - `_phytium_mci_invalidate` - Cache invalidate operation
#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[unsafe(no_mangle)]
        unsafe fn _phytium_mci_sleep(duration: core::time::Duration) {
            <$t as $crate::Kernel>::sleep(duration)
        }
        #[cfg(feature = "dma")]
        #[unsafe(no_mangle)]
        fn _phytium_mci_map(addr: core::ptr::NonNull<u8>) -> u64 {
            <$t as $crate::Kernel>::mmap(addr)
        }
        #[unsafe(no_mangle)]
        fn _phytium_mci_flush(addr: core::ptr::NonNull<u8>, size: usize) {
            <$t as $crate::Kernel>::flush(addr, size)
        }
        #[unsafe(no_mangle)]
        fn _phytium_mci_invalidate(addr: core::ptr::NonNull<u8>, size: usize) {
            <$t as $crate::Kernel>::invalidate(addr, size)
        }
    };
}
