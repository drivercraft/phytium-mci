//! # MCI Timing Configuration
//!
//! This module provides timing configuration for different SD/MMC operating modes.
//! It includes clock dividers, source selection, and pad delay settings for
//! various transfer speeds.
//!
//! ## Predefined Timing Configurations
//!
//! - `MMC_SD_400K_HZ`: Initialization mode (400 kHz)
//! - `SD_25MHZ`: SD default speed (25 MHz)
//! - `SD_50MHZ`: SD high speed (50 MHz)
//! - `SD_100MHZ`: SD UHS-I SDR50/DDR50 (100 MHz)
//! - `MMC_26MHZ`: MMC legacy speed (26 MHz)
//! - `MMC_52MHZ`: MMC high speed (52 MHz)
//! - `MMC_66MHZ`: MMC HS200 (66 MHz)
//! - `MMC_100MHZ`: MMC HS200 (100 MHz)

use crate::iopad::IoPad;
use crate::iopad::constants::{FioPadDelay, FioPadDelayDir, FioPadDelayType};
use crate::iopad::regs::{Aj49Reg1, J53Reg1, XReg1};
use crate::regs::BitsOps;

use super::constants::*;

/// MCI timing configuration structure.
///
/// This structure contains all timing-related parameters for SD/MMC operations
/// including clock configuration and pad delay settings.
pub struct MCITiming {
    /// Whether to use hold register
    use_hold: bool,
    /// Clock divider value
    clk_div: u32,
    /// Clock source selection
    clk_src: u32,
    /// Phase shift value
    shift: u32,
    /// Pad delay setting for signal timing
    pad_delay: MCIPadDelay,
}

impl MCITiming {
    /// Create a new default timing configuration.
    pub fn new() -> Self {
        MCITiming {
            use_hold: false,
            clk_div: 0,
            clk_src: 0,
            shift: 0,
            pad_delay: MCIPadDelay::None,
        }
    }
}

/// Pad delay configuration enum.
#[derive(Debug, PartialEq)]
enum MCIPadDelay {
    /// Apply pad delay settings
    Set,
    /// Remove pad delay settings
    Unset,
    /// No pad delay configuration
    None,
}

impl MCITiming {
    /// Apply pad delay settings to the I/O pad.
    ///
    /// # Arguments
    ///
    /// * `iopad` - Mutable reference to the I/O pad
    /// * `mci_id` - MCI controller ID
    pub(crate) fn pad_delay(&self, iopad: &mut IoPad, mci_id: MCIId) {
        match self.pad_delay {
            MCIPadDelay::Set => set_pad_delay(iopad, mci_id),
            MCIPadDelay::Unset => unset_pad_delay(iopad, mci_id),
            MCIPadDelay::None => {}
        }
    }

    /// Get the clock source value.
    pub(crate) fn clk_src(&self) -> u32 {
        self.clk_src
    }

    /// Get the clock divider value.
    pub(crate) fn clk_div(&self) -> u32 {
        self.clk_div
    }

    /// Check if hold register should be used.
    pub(crate) fn use_hold(&self) -> bool {
        self.use_hold
    }

    /// Get the phase shift value.
    pub(crate) fn shift(&self) -> u32 {
        self.shift
    }
}

/// Initialization mode timing (400 kHz).
pub const MMC_SD_400K_HZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x7e7dfa,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: MCIPadDelay::Unset,
};

/// SD default speed mode timing (25 MHz).
pub const SD_25MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: MCIPadDelay::Unset,
};

/// SD high speed mode timing (50 MHz).
pub const SD_50MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// SD UHS-I timing (100 MHz).
pub const SD_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// MMC legacy speed timing (26 MHz).
pub const MMC_26MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// MMC high speed timing (52 MHz).
pub const MMC_52MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x030204,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// MMC HS200 timing (66 MHz).
pub const MMC_66MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::None,
};

/// MMC HS200 timing (100 MHz).
pub const MMC_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/* Pin-related definitions */
/// FSDIF0 SD clock output delay register type
type Fsdif0SdCclkOutDelay = Aj49Reg1;
/// FSDIF1 SD clock output delay register type
type Fsdif1SdCclkOutDelay = J53Reg1;

/// Apply delay settings to I/O pad.
///
/// # Arguments
///
/// * `iopad` - Mutable reference to the I/O pad
/// * `coarse_delay` - Coarse delay value
/// * `fine_delay` - Fine delay value
/// * `enable` - Whether to enable the delay
fn apply_delay_settings<T: XReg1 + BitsOps>(
    iopad: &mut IoPad,
    coarse_delay: FioPadDelay,
    fine_delay: FioPadDelay,
    enable: bool,
) where
    T: 'static,
{
    iopad.delay_set::<T>(
        FioPadDelayDir::OutputDelay,
        FioPadDelayType::DelayCoarseTuning,
        coarse_delay,
    );
    iopad.delay_set::<T>(
        FioPadDelayDir::OutputDelay,
        FioPadDelayType::DelayFineTuning,
        fine_delay,
    );
    iopad.delay_enable_set::<T>(FioPadDelayDir::OutputDelay, enable);
}

/// Set pad delay for high-speed operation.
///
/// # Arguments
///
/// * `iopad` - Mutable reference to the I/O pad
/// * `mci_id` - MCI controller ID
pub fn set_pad_delay(iopad: &mut IoPad, mci_id: MCIId) {
    match mci_id {
        MCIId::MCI0 => apply_delay_settings::<Fsdif0SdCclkOutDelay>(
            iopad,
            FioPadDelay::Delay1,
            FioPadDelay::Delay7,
            true,
        ),
        MCIId::MCI1 => apply_delay_settings::<Fsdif1SdCclkOutDelay>(
            iopad,
            FioPadDelay::Delay1,
            FioPadDelay::Delay7,
            true,
        ),
    }
}

/// Unset pad delay (for low-speed operation).
///
/// # Arguments
///
/// * `iopad` - Mutable reference to the I/O pad
/// * `mci_id` - MCI controller ID
pub fn unset_pad_delay(iopad: &mut IoPad, mci_id: MCIId) {
    match mci_id {
        MCIId::MCI0 => apply_delay_settings::<Fsdif0SdCclkOutDelay>(
            iopad,
            FioPadDelay::DelayNone,
            FioPadDelay::DelayNone,
            false,
        ),
        MCIId::MCI1 => apply_delay_settings::<Fsdif1SdCclkOutDelay>(
            iopad,
            FioPadDelay::DelayNone,
            FioPadDelay::DelayNone,
            false,
        ),
    }
}
