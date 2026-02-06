//! # Phytium MCI Driver
//!
//! This is a `no_std` Rust driver for the Phytium Memory Card Interface (MCI) controller.
//! It provides support for SD/MMC card operations including initialization, data transfer,
//! and card management.
//!
//! ## Architecture
//!
//! The driver is organized into several modules:
//!
//! - **mci**: Core MCI controller functionality including register access, command handling,
//!   and data transfer (DMA/PIO modes)
//! - **mci_host**: Host controller abstraction layer providing card detection, command
//!   execution, and SD/MMC card operations
//! - **iopad**: I/O pad configuration for signal timing and electrical characteristics
//! - **osa**: Operating System Abstraction layer providing memory pool management
//!
//! ## Features
//!
//! - DMA and PIO transfer modes
//! - SD and eMMC card support
//! - Configurable bus width (1/4/8 bit)
//! - Variable clock frequency support
//! - Card detection and hot-plug support
//! - Voltage switching (3.3V/1.8V)
//!
//! ## Usage
//!
//! The driver requires the user to implement the [`Kernel`] trait and use the
//! [`set_impl!`] macro to provide sleep functionality.
//!
//! ```rust
//! use phytium_mci::{Kernel, set_impl};
//!
//! struct MyKernel;
//!
//! impl Kernel for MyKernel {
//!     fn sleep(duration: core::time::Duration) {
//!         // Implement sleep functionality
//!     }
//! }
//!
//! set_impl!(MyKernel);
//! ```

#![no_std]

extern crate alloc;

use core::time::Duration;

#[macro_use]
mod regs;
pub mod iopad;
pub mod mci;
pub mod mci_host;
pub mod osa;
mod tools;

pub use iopad::*;
pub use mci_host::*;

// pub use dma_api::{set_impl as set_dma_impl, Direction as DmaDirection, Impl as DmaImpl};

/// Trait that must be implemented by the user to provide kernel functionality.
///
/// This trait abstracts the underlying kernel/OS operations required by the driver.
/// Currently, only a sleep function is required for timing operations.
pub trait Kernel {
    /// Sleep for the specified duration.
    ///
    /// This is used internally for delay operations and timing control.
    fn sleep(duration: Duration);
}

pub(crate) fn sleep(duration: Duration) {
    unsafe extern "Rust" {
        fn _phytium_mci_sleep(duration: Duration);
    }

    unsafe {
        _phytium_mci_sleep(duration);
    }
}

/// Macro to set the kernel implementation for the driver.
///
/// This macro generates the internal function that bridges the driver's sleep
/// calls to the user-provided [`Kernel`] implementation.
///
/// # Example
///
/// ```rust
/// use phytium_mci::set_impl;
///
/// struct MyKernel;
/// impl phytium_mci::Kernel for MyKernel {
///     fn sleep(duration: core::time::Duration) {
///         // implementation
///     }
/// }
///
/// set_impl!(MyKernel);
/// ```
#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[no_mangle]
        unsafe fn _phytium_mci_sleep(duration: Duration) {
            <$t as $crate::Kernel>::sleep(duration)
        }
    };
}
