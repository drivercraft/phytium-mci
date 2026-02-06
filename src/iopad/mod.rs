//! # I/O Pad Module
//!
//! This module provides I/O pad configuration for signal timing and electrical characteristics.
//! It is used to configure pad delay settings for optimal signal timing in high-speed SD/MMC operations.
//!
//! ## Functionality
//!
//! - Function multiplexing configuration
//! - Pull-up/pull-down resistor configuration
//! - Drive strength configuration
//! - Input/output delay tuning (coarse and fine)

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

/// I/O Pad controller.
///
/// This structure manages I/O pad configuration including:
/// - Function selection (multiplexing)
/// - Pull resistor configuration
/// - Drive strength setting
/// - Signal delay tuning for high-speed operations
///
/// # Example
///
/// ```rust
/// use phytium_mci::IoPad;
/// use core::ptr::NonNull;
///
/// let base_addr = NonNull::new(0x28000000 as *mut u8).unwrap();
/// let mut iopad = IoPad::new(base_addr);
///
/// // Configure pad function, pull, and drive strength
/// iopad.config_set::<RegisterType>(
///     FioPadFunc::Func1,
///     FioPadPull::PullUp,
///     FioPadDrive::Drv4
/// );
/// ```
#[derive(Debug)]
pub struct IoPad {
    reg: IoPadReg,
    is_ready: bool,
}

impl IoPad {
    /// Create a new I/O Pad controller instance.
    ///
    /// # Arguments
    ///
    /// * `reg_base` - Base address of the I/O pad registers
    pub fn new(reg_base: NonNull<u8>) -> Self {
        IoPad {
            reg: IoPadReg::new(reg_base),
            is_ready: true,
        }
    }

    /// Get the base address of the I/O pad registers.
    pub fn get_base_addr(&self) -> NonNull<u8> {
        self.reg.addr
    }

    /// Get the function multiplexing setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, and BitsOps traits
    ///
    /// # Returns
    ///
    /// The current function setting for the pad
    pub fn func_get<T: FlagReg + XReg0 + BitsOps>(&self) -> FioPadFunc {
        let reg_val = self.reg.read_reg::<T>();
        let func = T::func_get(reg_val);
        func.into()
    }

    /// Set the function multiplexing setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, and BitsOps traits
    ///
    /// # Arguments
    ///
    /// * `func` - Function setting to apply
    pub fn func_set<T: FlagReg + XReg0 + BitsOps>(&mut self, func: FioPadFunc) {
        self.reg
            .modify_reg::<T>(|reg| reg | T::func_set(func.into()));
    }

    /// Get the pull resistor setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, and BitsOps traits
    ///
    /// # Returns
    ///
    /// The current pull resistor setting (none, pull-up, or pull-down)
    pub fn pull_get<T: FlagReg + XReg0 + BitsOps>(&self) -> FioPadPull {
        let reg_val = self.reg.read_reg::<T>();
        let pull = T::pull_get(reg_val);
        pull.into()
    }

    /// Set the pull resistor setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, and BitsOps traits
    ///
    /// # Arguments
    ///
    /// * `pull` - Pull resistor setting to apply (none, pull-up, or pull-down)
    pub fn pull_set<T: FlagReg + XReg0 + BitsOps>(&mut self, pull: FioPadPull) {
        self.reg
            .modify_reg::<T>(|reg| reg | T::pull_set(pull.into()));
    }

    /// Get the drive strength setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, and BitsOps traits
    ///
    /// # Returns
    ///
    /// The current drive strength setting (0-15)
    pub fn drive_get<T: FlagReg + XReg0 + BitsOps>(&self) -> FioPadDrive {
        let reg_val = self.reg.read_reg::<T>();
        let drive = T::drive_get(reg_val);
        drive.into()
    }

    /// Set the drive strength setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, and BitsOps traits
    ///
    /// # Arguments
    ///
    /// * `drive` - Drive strength setting to apply (0-15)
    pub fn drive_set<T: FlagReg + XReg0 + BitsOps>(&mut self, drive: FioPadDrive) {
        self.reg
            .modify_reg::<T>(|reg| reg | T::drive_set(drive.into()));
    }

    /// Set function, pull, and drive strength for a pad in one operation.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, and BitsOps traits
    ///
    /// # Arguments
    ///
    /// * `func` - Function multiplexing setting
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

    /// Get function, pull, and drive strength settings for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg0, BitsOps, and Copy traits
    ///
    /// # Returns
    ///
    /// A tuple containing (function, pull, drive) settings
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

    /// Get the delay setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg1, and BitsOps traits
    ///
    /// # Arguments
    ///
    /// * `dir` - Delay direction (input or output)
    /// * `typ` - Delay type (coarse or fine tuning)
    ///
    /// # Returns
    ///
    /// The current delay setting
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

    /// Set the delay setting for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg1, and BitsOps traits
    ///
    /// # Arguments
    ///
    /// * `dir` - Delay direction (input or output)
    /// * `typ` - Delay type (coarse or fine tuning)
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

    /// Enable or disable delay for a pad.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Register type implementing FlagReg, XReg1, and BitsOps traits
    ///
    /// # Arguments
    ///
    /// * `dir` - Delay direction (input or output)
    /// * `enable` - Whether to enable or disable the delay
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
