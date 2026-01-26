//! IOPAD register definitions and traits.
//!
//! This module provides traits and macros for defining and working with
//! IOPAD registers. IOPAD registers are divided into two types:
//! - **REG0**: Function, pull, and drive strength configuration
//! - **REG1**: Delay tuning configuration

use super::constants::*;
use crate::regs::FlagReg;
use bitflags::bitflags;
use core::ops;

/// Trait for REG0 type registers (function, pull, and drive configuration).
///
/// This trait provides methods to get and set pin function, pull resistor,
/// and drive strength configuration bits in REG0 registers.
pub trait XReg0: FlagReg {
    /// Creates a flag value for setting the pin function
    fn func_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 2, 0))
    }

    /// Creates a flag value for setting the drive strength
    fn drive_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 7, 4))
    }

    /// Creates a flag value for setting the pull resistor
    fn pull_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 9, 8))
    }

    /// Extracts the pin function value from the register
    fn func_get(self) -> u32 {
        get_reg32_bits!(self.bits(), 2, 0)
    }

    /// Extracts the drive strength value from the register
    fn drive_get(self) -> u32 {
        get_reg32_bits!(self.bits(), 7, 4)
    }

    /// Extracts the pull resistor value from the register
    fn pull_get(self) -> u32 {
        get_reg32_bits!(self.bits(), 9, 8)
    }
}

/// Trait for REG1 type registers (delay configuration).
///
/// This trait provides methods to get and set signal delay tuning
/// parameters in REG1 registers, including fine/coarse tuning for
/// both input and output paths.
pub trait XReg1: FlagReg {
    /// Creates a flag value for setting output fine delay
    fn out_delay_delicate_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 11, 9))
    }

    /// Creates a flag value for setting output coarse delay
    fn out_delay_rough_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 14, 12))
    }

    /// Creates a flag value for setting input fine delay
    fn in_delay_delicate_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 3, 1))
    }

    /// Creates a flag value for setting input coarse delay
    fn in_delay_rough_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 6, 4))
    }

    /// Extracts the output fine delay value from the register
    fn out_delay_delicate_get(self) -> u32 {
        get_reg32_bits!(self.bits(), 11, 9)
    }

    /// Extracts the output coarse delay value from the register
    fn out_delay_rough_get(self) -> u32 {
        get_reg32_bits!(self.bits(), 14, 12)
    }

    /// Extracts the input fine delay value from the register
    fn in_delay_delicate_get(self) -> u32 {
        get_reg32_bits!(self.bits(), 3, 1)
    }

    /// Extracts the input coarse delay value from the register
    fn in_delay_rough_get(self) -> u32 {
        get_reg32_bits!(self.bits(), 6, 4)
    }

    /// Returns a flag value to enable output delay
    fn out_delay_en() -> Self {
        Self::from_bits_truncate(1 << 8)
    }

    /// Returns a flag value to enable input delay
    fn in_delay_en() -> Self {
        Self::from_bits_truncate(1 << 0)
    }
}

/// Macro to define a REG0 type register.
///
/// This macro generates a bitflags structure for an IOPAD REG0 register
/// which controls pin function, pull resistor, and drive strength.
///
/// # Example
///
/// ```rust,ignore
/// X_REG0!(MyPinReg0, 0x1000);
/// ```
///
/// This generates a `MyPinReg0` struct with the appropriate bit flags
/// and implements the required traits.
#[macro_export]
macro_rules! X_REG0 {
    ($reg_name:ident, $reg_addr:expr) => {
        bitflags! {
            #[derive(Clone, Copy)]
            pub struct $reg_name: u32 {
                const PULL_MASK = genmask!(9, 8);
                const DRIVE_MASK = genmask!(7, 4);
                const FUNC_MASK = genmask!(2, 0);
                const FUNC_BIT0 = 1 << 0;
                const FUNC_BIT1 = 1 << 1;
                const FUNC_BIT2 = 1 << 2;
                const DRIVE_BIT0 = 1 << 4;
                const DRIVE_BIT1 = 1 << 5;
                const DRIVE_BIT2 = 1 << 6;
                const DRIVE_BIT3 = 1 << 7;
                const PULL_BIT0 = 1 << 8;
                const PULL_BIT1 = 1 << 9;
            }
        }

        impl FlagReg for $reg_name {
            const REG: u32 = $reg_addr;
        }

        impl XReg0 for $reg_name {}
    };
}

/// Macro to define a REG1 type register.
///
/// This macro generates a bitflags structure for an IOPAD REG1 register
/// which controls signal delay tuning parameters.
///
/// # Example
///
/// ```rust,ignore
/// X_REG1!(MyPinReg1, 0x2000);
/// ```
///
/// This generates a `MyPinReg1` struct with the appropriate bit flags
/// and implements the required traits.
#[macro_export]
macro_rules! X_REG1 {
    ($reg_name:ident, $reg_addr:expr) => {
        bitflags! {
            #[derive(Clone, Copy)]
            pub struct $reg_name: u32 {
                const OUT_DELAY_EN = 1 << 8;
                const OUT_DELAY_DELICATE_MASK = genmask!(11,9);
                const OUT_DELAY_DELICATE_BIT0 = 1 << 9;
                const OUT_DELAY_DELICATE_BIT1 = 1 << 10;
                const OUT_DELAY_DELICATE_BIT2 = 1 << 11;
                const OUT_DELAY_ROUGH_MASK = genmask!(14,12);
                const OUT_DELAY_ROUGH_BIT0 = 1 << 12;
                const OUT_DELAY_ROUGH_BIT1 = 1 << 13;
                const OUT_DELAY_ROUGH_BIT2 = 1 << 14;
                const IN_DELAY_EN = 1 << 0;
                const IN_DELAY_DELICATE_MASK = genmask!(3,1);
                const IN_DELAY_DELICATE_BIT0 = 1 << 1;
                const IN_DELAY_DELICATE_BIT1 = 1 << 2;
                const IN_DELAY_DELICATE_BIT2 = 1 << 3;
                const IN_DELAY_ROUGH_MASK = genmask!(6,4);
                const IN_DELAY_ROUGH_BIT0 = 1 << 4;
                const IN_DELAY_ROUGH_BIT1 = 1 << 5;
                const IN_DELAY_ROUGH_BIT2 = 1 << 6;
            }
        }

        impl FlagReg for $reg_name {
            const REG: u32 = $reg_addr;
        }

        impl XReg1 for $reg_name {}
    };
}

/// AN59 pin REG0 register (function, pull, drive configuration)
X_REG0!(An59Reg0, FIOPAD_AN59_REG0_OFFSET);

/// AJ49 pin REG1 register (delay configuration)
X_REG1!(Aj49Reg1, FIOPAD_AJ49_REG1_OFFSET);
/// J53 pin REG1 register (delay configuration)
X_REG1!(J53Reg1, FIOPAD_J53_REG1_OFFSET);
