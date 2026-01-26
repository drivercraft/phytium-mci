//! Timing and delay configuration for MCI operations
//!
//! This module provides timing parameters and I/O pad delay configuration
//! for different SD/MMC operating modes and clock frequencies.

use crate::iopad::IoPad;
use crate::iopad::constants::{FioPadDelay, FioPadDelayDir, FioPadDelayType};
use crate::iopad::regs::{Aj49Reg1, J53Reg1, XReg1};
use crate::regs::BitsOps;

use super::consts::*;

/// Timing configuration for MCI operations
///
/// Contains clock divider, source, and I/O pad delay settings for
/// a specific operating mode.
pub struct MCITiming {
    use_hold: bool,
    clk_div: u32,
    clk_src: u32,
    shift: u32,
    pad_delay: MCIPadDelay, //* Used to adjust IO delay */
}

impl Default for MCITiming {
    fn default() -> Self {
        Self::new()
    }
}

impl MCITiming {
    /// Creates a new timing configuration with default values
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

#[derive(Debug, PartialEq)]
enum MCIPadDelay {
    Set,
    Unset,
    None,
}

impl MCITiming {
    pub(crate) fn pad_delay(&self, iopad: &mut IoPad, mci_id: MCIId) {
        match self.pad_delay {
            MCIPadDelay::Set => set_pad_delay(iopad, mci_id),
            MCIPadDelay::Unset => unset_pad_delay(iopad, mci_id),
            MCIPadDelay::None => {}
        }
    }

    pub(crate) fn clk_src(&self) -> u32 {
        self.clk_src
    }

    pub(crate) fn clk_div(&self) -> u32 {
        self.clk_div
    }

    pub(crate) fn use_hold(&self) -> bool {
        self.use_hold
    }

    pub(crate) fn shift(&self) -> u32 {
        self.shift
    }
}

/// Timing configuration for 400 KHz (initialization)
pub const MMC_SD_400K_HZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x7e7dfa,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: MCIPadDelay::Unset,
};

/// Timing configuration for SD 25 MHz (default speed)
pub const SD_25MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: MCIPadDelay::Unset,
};

/// Timing configuration for SD 50 MHz (high speed)
pub const SD_50MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// Timing configuration for SD 100 MHz (UHS-I SDR104)
pub const SD_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// Timing configuration for MMC 26 MHz
pub const MMC_26MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// Timing configuration for MMC 52 MHz (high speed)
pub const MMC_52MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x030204,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/// Timing configuration for MMC 66 MHz
pub const MMC_66MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::None,
};

/// Timing configuration for MMC 100 MHz (HS200)
pub const MMC_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

/* Pin related definitions */
type Fsdif0SdCclkOutDelay = Aj49Reg1;
type Fsdif1SdCclkOutDelay = J53Reg1;

fn apply_delay_settings<T: XReg1 + BitsOps + 'static>(
    iopad: &mut IoPad,
    coarse_delay: FioPadDelay,
    fine_delay: FioPadDelay,
    enable: bool,
) {
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

/// Sets the I/O pad delay for high-speed operations
///
/// # Arguments
///
/// * `iopad` - I/O pad instance to configure
/// * `mci_id` - MCI instance identifier
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

/// Unsets the I/O pad delay
///
/// # Arguments
///
/// * `iopad` - I/O pad instance to configure
/// * `mci_id` - MCI instance identifier
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
