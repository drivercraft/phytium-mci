//! I/O Pad (IOPAD) configuration for Phytium SoCs.
//!
//! This module provides functionality for configuring I/O pads on Phytium E2000
//! series SoCs. I/O pads control pin multiplexing, pull-up/pull-down resistors,
//! drive strength, and signal delay tuning.
//!
//! # Overview
//!
//! The IOPAD controller manages:
//! - **Pin multiplexing** - Configuring pin functions (Func0-Func7)
//! - **Pull resistors** - Pull-up, pull-down, or no pull
//! - **Drive strength** - Output drive current (0-15 levels)
//! - **Delay tuning** - Fine and coarse delay adjustment for high-speed signals
//!
//! # Register Map
//!
//! IOPAD registers are organized into two types:
//! - **REG0** - Function, pull, and drive configuration
//! - **REG1** - Delay configuration
//!
//! # Example
//!
//! ```rust,ignore
//! use phytium_mci::IoPad;
//! use phytium_mci::iopad::{FioPadFunc, FioPadPull, FioPadDrive};
//! use core::ptr::NonNull;
//!
//! // Create IOPAD instance
//! let mut iopad = unsafe { IoPad::new(NonNull::new_unchecked(0x2800_0000 as *mut u8)) };
//!
//! // Configure pin function, pull, and drive
//! iopad.config_set::<YourPinRegister>(
//!     FioPadFunc::Func1,
//!     FioPadPull::PullUp,
//!     FioPadDrive::Drv8
//! );
//! ```
//!
//! # Pin Configuration
//!
//! Each pin can be configured with:
//! - **Function** (Func0-Func7): Selects the peripheral function for the pin
//! - **Pull** (PullNone/PullUp/PullDown): Configures internal pull resistors
//! - **Drive** (Drv0-Drv15): Sets output drive strength
//! - **Delay**: Fine/coarse tuning for signal timing

#![allow(unused)]
pub(crate) mod constants;
mod err;
pub(crate) mod regs;

use crate::regs::{BitsOps, FlagReg, Reg};
pub use constants::*;
use core::ptr::NonNull;
use err::*;
use regs::{XReg0, XReg1};

type IoPadReg = Reg<FioPadError>;

/// I/O Pad configuration interface for Phytium SoCs.
///
/// `IoPad` provides methods to configure pin functionality including
/// multiplexing, pull resistors, drive strength, and signal delay tuning.
///
/// # Generic Type Parameters
///
/// The methods use generic types `T` that must implement the following traits:
/// - `FlagReg`: Provides register flag operations
/// - `XReg0` or `XReg1`: Distinguishes between REG0 (config) and REG1 (delay) registers
/// - `BitsOps`: Provides bit manipulation operations
///
/// # Example
///
/// ```rust,ignore
/// use phytium_mci::IoPad;
/// use phytium_mci::iopad::{FioPadFunc, FioPadPull, FioPadDrive};
/// use core::ptr::NonNull;
///
/// let mut iopad = unsafe {
///     IoPad::new(NonNull::new_unchecked(0x2800_0000 as *mut u8))
/// };
///
/// // Configure a specific pin
/// iopad.config_set::<PinRegister>(
///     FioPadFunc::Func1,  // Use function 1
///     FioPadPull::PullUp, // Enable pull-up
///     FioPadDrive::Drv8    // Medium drive strength
/// );
/// ```
#[derive(Debug)]
pub struct IoPad {
    reg: IoPadReg,
    is_ready: bool,
}

impl IoPad {
    /// Creates a new IOPAD instance.
    ///
    /// # Arguments
    ///
    /// * `reg_base` - Base address of the IOPAD register block (typically 0x2800_0000)
    ///
    /// # Safety
    ///
    /// The caller must ensure that `reg_base` points to a valid memory-mapped
    /// IOPAD register region.
    pub fn new(reg_base: NonNull<u8>) -> Self {
        IoPad {
            reg: IoPadReg::new(reg_base),
            is_ready: true,
        }
    }

    /// Returns the base address of the IOPAD register block.
    pub fn get_base_addr(&self) -> NonNull<u8> {
        self.reg.addr
    }

    /// Gets the pin function configuration.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`)
    ///
    /// # Returns
    ///
    /// The current function setting (Func0-Func7)
    pub fn func_get<T: FlagReg + XReg0 + BitsOps>(&self) -> FioPadFunc {
        let reg_val = self.reg.read_reg::<T>();
        let func = T::func_get(reg_val);
        func.into()
    }

    /// Sets the pin function configuration.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`)
    ///
    /// # Arguments
    ///
    /// * `func` - Function to assign to this pin (Func0-Func7)
    pub fn func_set<T: FlagReg + XReg0 + BitsOps>(&mut self, func: FioPadFunc) {
        self.reg
            .modify_reg::<T>(|reg| reg | T::func_set(func.into()));
    }

    /// Gets the pull resistor configuration.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`)
    ///
    /// # Returns
    ///
    /// The current pull resistor setting (PullNone, PullUp, or PullDown)
    pub fn pull_get<T: FlagReg + XReg0 + BitsOps>(&self) -> FioPadPull {
        let reg_val = self.reg.read_reg::<T>();
        let pull = T::pull_get(reg_val);
        pull.into()
    }

    /// Sets the pull resistor configuration.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`)
    ///
    /// # Arguments
    ///
    /// * `pull` - Pull resistor setting
    pub fn pull_set<T: FlagReg + XReg0 + BitsOps>(&mut self, pull: FioPadPull) {
        self.reg
            .modify_reg::<T>(|reg| reg | T::pull_set(pull.into()));
    }

    /// Gets the drive strength configuration.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`)
    ///
    /// # Returns
    ///
    /// The current drive strength setting (Drv0-Drv15, where higher = stronger)
    pub fn drive_get<T: FlagReg + XReg0 + BitsOps>(&self) -> FioPadDrive {
        let reg_val = self.reg.read_reg::<T>();
        let drive = T::drive_get(reg_val);
        drive.into()
    }

    /// Sets the drive strength configuration.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`)
    ///
    /// # Arguments
    ///
    /// * `drive` - Drive strength setting (Drv0-Drv15)
    pub fn drive_set<T: FlagReg + XReg0 + BitsOps>(&mut self, drive: FioPadDrive) {
        self.reg
            .modify_reg::<T>(|reg| reg | T::drive_set(drive.into()));
    }

    /// Sets all pin configuration parameters at once.
    ///
    /// This is a convenience method that configures function, pull, and drive
    /// in a single register write.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`)
    ///
    /// # Arguments
    ///
    /// * `func` - Pin function (Func0-Func7)
    /// * `pull` - Pull resistor setting
    /// * `drive` - Drive strength setting
    pub fn config_set<T: FlagReg + XReg0 + BitsOps>(
        &mut self,
        func: FioPadFunc,
        pull: FioPadPull,
        drive: FioPadDrive,
    ) {
        self.reg.modify_reg::<T>(|reg| {
            reg | T::func_set(func.into()) | T::pull_set(pull.into()) | T::drive_set(drive.into())
        });
    }

    /// Gets all pin configuration parameters.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg0`, `BitsOps`, `Copy`)
    ///
    /// # Returns
    ///
    /// A tuple of (function, pull, drive)
    pub fn config_get<T: FlagReg + XReg0 + BitsOps + Copy>(
        &self,
    ) -> (FioPadFunc, FioPadPull, FioPadDrive) {
        let reg_val = self.reg.read_reg::<T>();
        let func = T::func_get(reg_val);
        let pull = T::pull_get(reg_val);
        let drive = T::drive_get(reg_val);
        (
            FioPadFunc::from(func),
            FioPadPull::from(pull),
            FioPadDrive::from(drive),
        )
    }

    /// Gets the delay configuration for a pin.
    ///
    /// Delay tuning is used for high-speed signals to adjust timing.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg1`, `BitsOps`)
    ///
    /// # Arguments
    ///
    /// * `dir` - Delay direction (input or output)
    /// * `typ` - Delay type (fine or coarse tuning)
    ///
    /// # Returns
    ///
    /// The current delay value
    pub fn delay_get<T: FlagReg + XReg1 + BitsOps>(
        &self,
        dir: FioPadDelayDir,
        typ: FioPadDelayType,
    ) -> FioPadDelay {
        let reg_val = self.reg.read_reg::<T>();
        let mut delay = 0;
        if dir == FioPadDelayDir::OutputDelay {
            if typ == FioPadDelayType::DelayFineTuning {
                delay = T::out_delay_delicate_get(reg_val);
            } else if typ == FioPadDelayType::DelayCoarseTuning {
                delay = T::out_delay_rough_get(reg_val);
            }
        } else if dir == FioPadDelayDir::InputDelay {
            if typ == FioPadDelayType::DelayFineTuning {
                delay = T::in_delay_delicate_get(reg_val);
            } else if typ == FioPadDelayType::DelayCoarseTuning {
                delay = T::in_delay_rough_get(reg_val);
            }
        }
        delay.into()
    }

    /// Sets the delay configuration for a pin.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg1`, `BitsOps`)
    ///
    /// # Arguments
    ///
    /// * `dir` - Delay direction (input or output)
    /// * `typ` - Delay type (fine or coarse tuning)
    /// * `delay` - Delay value to set
    pub fn delay_set<T: FlagReg + XReg1 + BitsOps>(
        &mut self,
        dir: FioPadDelayDir,
        typ: FioPadDelayType,
        delay: FioPadDelay,
    ) {
        if dir == FioPadDelayDir::OutputDelay {
            if typ == FioPadDelayType::DelayFineTuning {
                self.reg
                    .modify_reg::<T>(|reg| reg | T::out_delay_delicate_set(delay.into()));
            } else if typ == FioPadDelayType::DelayCoarseTuning {
                self.reg
                    .modify_reg::<T>(|reg| reg | T::out_delay_rough_set(delay.into()));
            }
        } else if dir == FioPadDelayDir::InputDelay {
            if typ == FioPadDelayType::DelayFineTuning {
                self.reg
                    .modify_reg::<T>(|reg| reg | T::in_delay_delicate_set(delay.into()));
            } else if typ == FioPadDelayType::DelayCoarseTuning {
                self.reg
                    .modify_reg::<T>(|reg| reg | T::in_delay_rough_set(delay.into()));
            }
        }
    }

    /// Enables or disables delay for a pin.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type for the specific pin (must implement `FlagReg`, `XReg1`, `BitsOps`)
    ///
    /// # Arguments
    ///
    /// * `dir` - Delay direction (input or output)
    /// * `enable` - true to enable delay, false to disable
    pub fn delay_enable_set<T: FlagReg + XReg1 + BitsOps>(
        &mut self,
        dir: FioPadDelayDir,
        enable: bool,
    ) {
        if dir == FioPadDelayDir::OutputDelay {
            self.reg.modify_reg::<T>(|reg| {
                if enable {
                    reg | T::out_delay_en()
                } else {
                    reg & !T::out_delay_en()
                }
            });
        } else if dir == FioPadDelayDir::InputDelay {
            self.reg.modify_reg::<T>(|reg| {
                if enable {
                    reg | T::in_delay_en()
                } else {
                    reg & !T::in_delay_en()
                }
            });
        }
    }
}
