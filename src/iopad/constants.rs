//! # IOPAD Constants
//!
//! This module provides constants and enumerations for configuring the Phytium
//! I/O Pad (IOPAD) controller. The IOPAD is responsible for pin multiplexing,
//! signal delay tuning, and electrical characteristics configuration.
//!
//! ## Overview
//!
//! The IOPAD controller manages:
//! - **Pin Function Selection**: Each pin can be configured to one of 8 functions (Func0-Func7)
//! - **Drive Strength**: 16 levels of drive strength control (Drv0-Drv15)
//! - **Pull Configuration**: Pull-up, pull-down, or no pull resistor
//! - **Signal Delay**: Coarse and fine delay tuning for high-speed signals
//!
//! ## Pin Naming Convention
//!
//! Pin names follow the pattern: `{PORT}{PIN}` where:
//! - `PORT` is a letter (A, B, C, etc.) representing the GPIO port
//! - `PIN` is the pin number within that port
//!
//! For example: `AN59` = Port A, Pin 59 (with additional signal qualifier N)

/// I/O Pad function selection.
///
/// Each pin on the Phytium SoC can be configured to one of 8 different
/// peripheral functions through multiplexing.
///
/// # Variants
///
/// - `Func0` - Function 0 (default GPIO)
/// - `Func1` - Function 1
/// - `Func2` - Function 2
/// - `Func3` - Function 3
/// - `Func4` - Function 4
/// - `Func5` - Function 5
/// - `Func6` - Function 6
/// - `Func7` - Function 7
/// - `NumOfFunc` - Sentinel value representing the count of functions
#[derive(Debug, Clone, Copy)]
pub enum FioPadFunc {
    /// Function 0 (default GPIO or primary function)
    Func0 = 0b000,
    /// Function 1
    Func1,
    /// Function 2
    Func2,
    /// Function 3
    Func3 = 0b011,
    /// Function 4
    Func4,
    /// Function 5
    Func5,
    /// Function 6
    Func6,
    /// Function 7
    Func7 = 0b111,
    /// Sentinel value representing the count of functions
    NumOfFunc,
}

impl From<u32> for FioPadFunc {
    fn from(value: u32) -> Self {
        match value {
            0b000 => FioPadFunc::Func0,
            0b001 => FioPadFunc::Func1,
            0b010 => FioPadFunc::Func2,
            0b011 => FioPadFunc::Func3,
            0b100 => FioPadFunc::Func4,
            0b101 => FioPadFunc::Func5,
            0b110 => FioPadFunc::Func6,
            0b111 => FioPadFunc::Func7,
            _ => panic!("Invalid value for FioPadFunc"),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for FioPadFunc {
    fn into(self) -> u32 {
        match self {
            FioPadFunc::Func0 => 0b000,
            FioPadFunc::Func1 => 0b001,
            FioPadFunc::Func2 => 0b010,
            FioPadFunc::Func3 => 0b011,
            FioPadFunc::Func4 => 0b100,
            FioPadFunc::Func5 => 0b101,
            FioPadFunc::Func6 => 0b110,
            FioPadFunc::Func7 => 0b111,
            _ => panic!("Invalid value for FioPadFunc"),
        }
    }
}

/// I/O Pad drive strength configuration.
///
/// Controls the output drive strength for a pin, with 16 levels from
/// weakest (Drv0) to strongest (Drv15). Higher drive strength allows
/// faster signal edges but consumes more power.
///
/// # Variants
///
/// - `Drv0` - Minimum drive strength (lowest power, slowest edges)
/// - `Drv1` through `Drv14` - Intermediate drive strengths
/// - `Drv15` - Maximum drive strength (highest power, fastest edges)
/// - `NumOfDrive` - Sentinel value representing the count of drive levels
#[derive(Debug, Clone, Copy)]
pub enum FioPadDrive {
    /// Minimum drive strength (lowest power, slowest edges)
    Drv0 = 0b0000,
    /// Drive strength level 1
    Drv1,
    /// Drive strength level 2
    Drv2,
    /// Drive strength level 3
    Drv3,
    /// Drive strength level 4
    Drv4,
    /// Drive strength level 5
    Drv5,
    /// Drive strength level 6
    Drv6,
    /// Drive strength level 7
    Drv7,
    /// Drive strength level 8
    Drv8,
    /// Drive strength level 9
    Drv9,
    /// Drive strength level 10
    Drv10,
    /// Drive strength level 11
    Drv11,
    /// Drive strength level 12
    Drv12,
    /// Drive strength level 13
    Drv13,
    /// Drive strength level 14
    Drv14,
    /// Maximum drive strength (highest power, fastest edges)
    Drv15 = 0b1111,
    /// Sentinel value representing the count of drive levels
    NumOfDrive,
}

impl From<u32> for FioPadDrive {
    fn from(value: u32) -> Self {
        match value {
            0b0000 => FioPadDrive::Drv0,
            0b0001 => FioPadDrive::Drv1,
            0b0010 => FioPadDrive::Drv2,
            0b0011 => FioPadDrive::Drv3,
            0b0100 => FioPadDrive::Drv4,
            0b0101 => FioPadDrive::Drv5,
            0b0110 => FioPadDrive::Drv6,
            0b0111 => FioPadDrive::Drv7,
            0b1000 => FioPadDrive::Drv8,
            0b1001 => FioPadDrive::Drv9,
            0b1010 => FioPadDrive::Drv10,
            0b1011 => FioPadDrive::Drv11,
            0b1100 => FioPadDrive::Drv12,
            0b1101 => FioPadDrive::Drv13,
            0b1110 => FioPadDrive::Drv14,
            0b1111 => FioPadDrive::Drv15,
            _ => panic!("Invalid value for FioPadDrive"),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for FioPadDrive {
    fn into(self) -> u32 {
        match self {
            FioPadDrive::Drv0 => 0b0000,
            FioPadDrive::Drv1 => 0b0001,
            FioPadDrive::Drv2 => 0b0010,
            FioPadDrive::Drv3 => 0b0011,
            FioPadDrive::Drv4 => 0b0100,
            FioPadDrive::Drv5 => 0b0101,
            FioPadDrive::Drv6 => 0b0110,
            FioPadDrive::Drv7 => 0b0111,
            FioPadDrive::Drv8 => 0b1000,
            FioPadDrive::Drv9 => 0b1001,
            FioPadDrive::Drv10 => 0b1010,
            FioPadDrive::Drv11 => 0b1011,
            FioPadDrive::Drv12 => 0b1100,
            FioPadDrive::Drv13 => 0b1101,
            FioPadDrive::Drv14 => 0b1110,
            FioPadDrive::Drv15 => 0b1111,
            _ => panic!("Invalid value for FioPadDrive"),
        }
    }
}

/// I/O Pad pull resistor configuration.
///
/// Controls the internal pull-up or pull-down resistors for a pin.
///
/// # Variants
///
/// - `PullNone` - No pull resistor (floating)
/// - `PullDown` - Pull-down resistor enabled
/// - `PullUp` - Pull-up resistor enabled
/// - `NumOfPull` - Sentinel value representing the count of pull configurations
#[derive(Debug, Clone, Copy)]
pub enum FioPadPull {
    /// No pull resistor (floating)
    PullNone = 0b00,
    /// Pull-down resistor enabled
    PullDown = 0b01,
    /// Pull-up resistor enabled
    PullUp = 0b10,
    /// Sentinel value representing the count of pull configurations
    NumOfPull,
}

impl From<u32> for FioPadPull {
    fn from(value: u32) -> Self {
        match value {
            0b00 => FioPadPull::PullNone,
            0b01 => FioPadPull::PullDown,
            0b10 => FioPadPull::PullUp,
            _ => panic!("Invalid value for FioPadPull"),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for FioPadPull {
    fn into(self) -> u32 {
        match self {
            FioPadPull::PullNone => 0b00,
            FioPadPull::PullDown => 0b01,
            FioPadPull::PullUp => 0b10,
            _ => panic!("Invalid value for FioPadPull"),
        }
    }
}

impl FioPadPull {
    /// Returns `true` if no pull resistor is configured.
    pub fn is_pull_none(&self) -> bool {
        matches!(self, FioPadPull::PullNone)
    }

    /// Returns `true` if pull-down resistor is configured.
    pub fn is_pull_down(&self) -> bool {
        matches!(self, FioPadPull::PullDown)
    }

    /// Returns `true` if pull-up resistor is configured.
    pub fn is_pull_up(&self) -> bool {
        matches!(self, FioPadPull::PullUp)
    }
}

/// I/O Pad delay direction.
///
/// Specifies whether the delay is applied to the output or input path.
///
/// # Variants
///
/// - `OutputDelay` - Delay applied to output signals
/// - `InputDelay` - Delay applied to input signals
/// - `NumOfDelayDir` - Sentinel value representing the count of delay directions
#[derive(Debug, PartialEq)]
pub enum FioPadDelayDir {
    /// Delay applied to output signals
    OutputDelay = 0,
    /// Delay applied to input signals
    InputDelay,
    /// Sentinel value representing the count of delay directions
    NumOfDelayDir,
}

/// I/O Pad delay tuning type.
///
/// Specifies the type of delay tuning to apply.
///
/// # Variants
///
/// - `DelayCoarseTuning` - Coarse delay adjustment (larger steps)
/// - `DelayFineTuning` - Fine delay adjustment (smaller steps)
/// - `NumOfDelayType` - Sentinel value representing the count of delay types
#[derive(Debug, PartialEq)]
pub enum FioPadDelayType {
    /// Coarse delay adjustment (larger steps)
    DelayCoarseTuning = 0,
    /// Fine delay adjustment (smaller steps)
    DelayFineTuning,
    /// Sentinel value representing the count of delay types
    NumOfDelayType,
}

/// I/O Pad delay value.
///
/// Represents the delay amount to be applied to a signal.
///
/// # Variants
///
/// - `DelayNone` - No delay applied
/// - `Delay1` through `Delay7` - Increasing delay amounts
/// - `NumOfDelay` - Sentinel value representing the count of delay values
#[derive(Debug, Clone, Copy)]
pub enum FioPadDelay {
    /// No delay applied
    DelayNone = 0,
    /// Delay level 1
    Delay1,
    /// Delay level 2
    Delay2,
    /// Delay level 3
    Delay3,
    /// Delay level 4
    Delay4,
    /// Delay level 5
    Delay5,
    /// Delay level 6
    Delay6,
    /// Delay level 7
    Delay7,
    /// Sentinel value representing the count of delay values
    NumOfDelay,
}

impl From<u32> for FioPadDelay {
    fn from(value: u32) -> Self {
        match value {
            0 => FioPadDelay::DelayNone,
            1 => FioPadDelay::Delay1,
            2 => FioPadDelay::Delay2,
            3 => FioPadDelay::Delay3,
            4 => FioPadDelay::Delay4,
            5 => FioPadDelay::Delay5,
            6 => FioPadDelay::Delay6,
            7 => FioPadDelay::Delay7,
            _ => panic!("Invalid value for FioPadDelay"),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u32> for FioPadDelay {
    fn into(self) -> u32 {
        match self {
            FioPadDelay::DelayNone => 0,
            FioPadDelay::Delay1 => 1,
            FioPadDelay::Delay2 => 2,
            FioPadDelay::Delay3 => 3,
            FioPadDelay::Delay4 => 4,
            FioPadDelay::Delay5 => 5,
            FioPadDelay::Delay6 => 6,
            FioPadDelay::Delay7 => 7,
            _ => panic!("Invalid value for FioPadDelay"),
        }
    }
}

// register offset of iopad function / pull / driver strength
//
// The following constants define register offsets for IOPAD pin configuration.
// Each register controls:
// - Pin function selection (Func0-Func7)
// - Pull resistor configuration (pull-up, pull-down, or none)
// - Drive strength (Drv0-Drv15)
//
// Register naming convention: FIOPAD_{PIN_NAME}_REG0_OFFSET
// where {PIN_NAME} follows the pattern {PORT}{PIN} (e.g., AN59 = Port A, Pin 59, signal N)
//
// These are all REG0 type registers used for function, pull, and drive configuration.
/// Register offset for AN59 pin function, pull, and drive strength configuration
/// Register offset for FIOPAD_AN59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AN59_REG0_OFFSET: u32 = 0x0000;
/// Register offset for AW47 pin function, pull, and drive strength configuration
/// Register offset for FIOPAD_AW47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AW47_REG0_OFFSET: u32 = 0x0004;
/// Register offset for FIOPAD_AR55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AR55_REG0_OFFSET: u32 = 0x0020;
/// Register offset for FIOPAD_AJ55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AJ55_REG0_OFFSET: u32 = 0x0024;
/// Register offset for FIOPAD_AL55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AL55_REG0_OFFSET: u32 = 0x0028;
/// Register offset for FIOPAD_AL53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AL53_REG0_OFFSET: u32 = 0x002C;
/// Register offset for FIOPAD_AN51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AN51_REG0_OFFSET: u32 = 0x0030;
/// Register offset for FIOPAD_AR51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AR51_REG0_OFFSET: u32 = 0x0034;
/// Register offset for FIOPAD_BA57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_BA57_REG0_OFFSET: u32 = 0x0038;
/// Register offset for FIOPAD_BA59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_BA59_REG0_OFFSET: u32 = 0x003C;
/// Register offset for FIOPAD_AW57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AW57_REG0_OFFSET: u32 = 0x0040;
/// Register offset for FIOPAD_AW59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AW59_REG0_OFFSET: u32 = 0x0044;
/// Register offset for FIOPAD_AU55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AU55_REG0_OFFSET: u32 = 0x0048;
/// Register offset for FIOPAD_AN57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AN57_REG0_OFFSET: u32 = 0x004C;
/// Register offset for FIOPAD_AL59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AL59_REG0_OFFSET: u32 = 0x0050;
/// Register offset for FIOPAD_AJ59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AJ59_REG0_OFFSET: u32 = 0x0054;
/// Register offset for FIOPAD_AJ57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AJ57_REG0_OFFSET: u32 = 0x0058;
/// Register offset for FIOPAD_AG59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AG59_REG0_OFFSET: u32 = 0x005C;
/// Register offset for FIOPAD_AG57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AG57_REG0_OFFSET: u32 = 0x0060;
/// Register offset for FIOPAD_AE59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AE59_REG0_OFFSET: u32 = 0x0064;
/// Register offset for FIOPAD_AC59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AC59_REG0_OFFSET: u32 = 0x0068;
/// Register offset for FIOPAD_AC57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AC57_REG0_OFFSET: u32 = 0x006C;
/// Register offset for FIOPAD_AR49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AR49_REG0_OFFSET: u32 = 0x0070;
/// Register offset for FIOPAD_BA55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_BA55_REG0_OFFSET: u32 = 0x0074;
/// Register offset for FIOPAD_BA53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_BA53_REG0_OFFSET: u32 = 0x0078;
/// Register offset for FIOPAD_AR59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AR59_REG0_OFFSET: u32 = 0x007C;
/// Register offset for FIOPAD_AU59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AU59_REG0_OFFSET: u32 = 0x0080;
/// Register offset for FIOPAD_AR57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AR57_REG0_OFFSET: u32 = 0x0084;
/// Register offset for FIOPAD_BA49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_BA49_REG0_OFFSET: u32 = 0x0088;
/// Register offset for FIOPAD_AW55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AW55_REG0_OFFSET: u32 = 0x008C;
/// Register offset for FIOPAD_A35_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A35_REG0_OFFSET: u32 = 0x0090;
/// Register offset for FIOPAD_R57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_R57_REG0_OFFSET: u32 = 0x0094;
/// Register offset for FIOPAD_R59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_R59_REG0_OFFSET: u32 = 0x0098;
/// Register offset for FIOPAD_U59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_U59_REG0_OFFSET: u32 = 0x009C;
/// Register offset for FIOPAD_W59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_W59_REG0_OFFSET: u32 = 0x00A0;
/// Register offset for FIOPAD_U57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_U57_REG0_OFFSET: u32 = 0x00A4;
/// Register offset for FIOPAD_AA57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AA57_REG0_OFFSET: u32 = 0x00A8;
/// Register offset for FIOPAD_AA59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AA59_REG0_OFFSET: u32 = 0x00AC;
/// Register offset for FIOPAD_AW51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AW51_REG0_OFFSET: u32 = 0x00B0;
/// Register offset for FIOPAD_AU51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AU51_REG0_OFFSET: u32 = 0x00B4;
/// Register offset for FIOPAD_A39_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A39_REG0_OFFSET: u32 = 0x00B8;
/// Register offset for FIOPAD_C39_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C39_REG0_OFFSET: u32 = 0x00BC;
/// Register offset for FIOPAD_C37_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C37_REG0_OFFSET: u32 = 0x00C0;
/// Register offset for FIOPAD_A37_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A37_REG0_OFFSET: u32 = 0x00C4;
/// Register offset for FIOPAD_A41_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A41_REG0_OFFSET: u32 = 0x00C8;
/// Register offset for FIOPAD_A43_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A43_REG0_OFFSET: u32 = 0x00CC;
/// Register offset for FIOPAD_A45_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A45_REG0_OFFSET: u32 = 0x00D0;
/// Register offset for FIOPAD_C45_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C45_REG0_OFFSET: u32 = 0x00D4;
/// Register offset for FIOPAD_A47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A47_REG0_OFFSET: u32 = 0x00D8;
/// Register offset for FIOPAD_A49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A49_REG0_OFFSET: u32 = 0x00DC;
/// Register offset for FIOPAD_C49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C49_REG0_OFFSET: u32 = 0x00E0;
/// Register offset for FIOPAD_A51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A51_REG0_OFFSET: u32 = 0x00E4;
/// Register offset for FIOPAD_A33_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A33_REG0_OFFSET: u32 = 0x00E8;
/// Register offset for FIOPAD_C33_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C33_REG0_OFFSET: u32 = 0x00EC;
/// Register offset for FIOPAD_C31_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C31_REG0_OFFSET: u32 = 0x00F0;
/// Register offset for FIOPAD_A31_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_A31_REG0_OFFSET: u32 = 0x00F4;
/// Register offset for FIOPAD_AJ53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AJ53_REG0_OFFSET: u32 = 0x00F8;
/// Register offset for FIOPAD_AL49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AL49_REG0_OFFSET: u32 = 0x00FC;
/// Register offset for FIOPAD_AL47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AL47_REG0_OFFSET: u32 = 0x0100;
/// Register offset for FIOPAD_AN49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AN49_REG0_OFFSET: u32 = 0x0104;
/// Register offset for FIOPAD_AG51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AG51_REG0_OFFSET: u32 = 0x0108;
/// Register offset for FIOPAD_AJ51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AJ51_REG0_OFFSET: u32 = 0x010C;
/// Register offset for FIOPAD_AG49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AG49_REG0_OFFSET: u32 = 0x0110;
/// Register offset for FIOPAD_AE55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AE55_REG0_OFFSET: u32 = 0x0114;
/// Register offset for FIOPAD_AE53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AE53_REG0_OFFSET: u32 = 0x0118;
/// Register offset for FIOPAD_AG55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AG55_REG0_OFFSET: u32 = 0x011C;
/// Register offset for FIOPAD_AJ49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AJ49_REG0_OFFSET: u32 = 0x0120;
/// Register offset for FIOPAD_AC55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AC55_REG0_OFFSET: u32 = 0x0124;
/// Register offset for FIOPAD_AC53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AC53_REG0_OFFSET: u32 = 0x0128;
/// Register offset for FIOPAD_AE51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AE51_REG0_OFFSET: u32 = 0x012C;
/// Register offset for FIOPAD_W51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_W51_REG0_OFFSET: u32 = 0x0130;
/// Register offset for FIOPAD_W55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_W55_REG0_OFFSET: u32 = 0x0134;
/// Register offset for FIOPAD_W53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_W53_REG0_OFFSET: u32 = 0x0138;
/// Register offset for FIOPAD_U55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_U55_REG0_OFFSET: u32 = 0x013C;
/// Register offset for FIOPAD_U53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_U53_REG0_OFFSET: u32 = 0x0140;
/// Register offset for FIOPAD_AE49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AE49_REG0_OFFSET: u32 = 0x0144;
/// Register offset for FIOPAD_AC49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AC49_REG0_OFFSET: u32 = 0x0148;
/// Register offset for FIOPAD_AE47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AE47_REG0_OFFSET: u32 = 0x014C;
/// Register offset for FIOPAD_AA47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AA47_REG0_OFFSET: u32 = 0x0150;
/// Register offset for FIOPAD_AA49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AA49_REG0_OFFSET: u32 = 0x0154;
/// Register offset for FIOPAD_W49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_W49_REG0_OFFSET: u32 = 0x0158;
/// Register offset for FIOPAD_AA51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_AA51_REG0_OFFSET: u32 = 0x015C;
/// Register offset for FIOPAD_U49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_U49_REG0_OFFSET: u32 = 0x0160;
/// Register offset for FIOPAD_G59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_G59_REG0_OFFSET: u32 = 0x0164;
/// Register offset for FIOPAD_J59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J59_REG0_OFFSET: u32 = 0x0168;
/// Register offset for FIOPAD_L57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L57_REG0_OFFSET: u32 = 0x016C;
/// Register offset for FIOPAD_C59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C59_REG0_OFFSET: u32 = 0x0170;
/// Register offset for FIOPAD_E59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E59_REG0_OFFSET: u32 = 0x0174;
/// Register offset for FIOPAD_J57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J57_REG0_OFFSET: u32 = 0x0178;
/// Register offset for FIOPAD_L59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L59_REG0_OFFSET: u32 = 0x017C;
/// Register offset for FIOPAD_N59_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N59_REG0_OFFSET: u32 = 0x0180;
/// Register offset for FIOPAD_C57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C57_REG0_OFFSET: u32 = 0x0184;
/// Register offset for FIOPAD_E57_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E57_REG0_OFFSET: u32 = 0x0188;
/// Register offset for FIOPAD_E31_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E31_REG0_OFFSET: u32 = 0x018C;
/// Register offset for FIOPAD_G31_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_G31_REG0_OFFSET: u32 = 0x0190;
/// Register offset for FIOPAD_N41_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N41_REG0_OFFSET: u32 = 0x0194;
/// Register offset for FIOPAD_N39_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N39_REG0_OFFSET: u32 = 0x0198;
/// Register offset for FIOPAD_J33_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J33_REG0_OFFSET: u32 = 0x019C;
/// Register offset for FIOPAD_N33_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N33_REG0_OFFSET: u32 = 0x01A0;
/// Register offset for FIOPAD_L33_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L33_REG0_OFFSET: u32 = 0x01A4;
/// Register offset for FIOPAD_N45_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N45_REG0_OFFSET: u32 = 0x01A8;
/// Register offset for FIOPAD_N43_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N43_REG0_OFFSET: u32 = 0x01AC;
/// Register offset for FIOPAD_L31_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L31_REG0_OFFSET: u32 = 0x01B0;
/// Register offset for FIOPAD_J31_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J31_REG0_OFFSET: u32 = 0x01B4;
/// Register offset for FIOPAD_J29_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J29_REG0_OFFSET: u32 = 0x01B8;
/// Register offset for FIOPAD_E29_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E29_REG0_OFFSET: u32 = 0x01BC;
/// Register offset for FIOPAD_G29_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_G29_REG0_OFFSET: u32 = 0x01C0;
/// Register offset for FIOPAD_N27_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N27_REG0_OFFSET: u32 = 0x01C4;
/// Register offset for FIOPAD_L29_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L29_REG0_OFFSET: u32 = 0x01C8;
/// Register offset for FIOPAD_J37_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J37_REG0_OFFSET: u32 = 0x01CC;
/// Register offset for FIOPAD_J39_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J39_REG0_OFFSET: u32 = 0x01D0;
/// Register offset for FIOPAD_G41_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_G41_REG0_OFFSET: u32 = 0x01D4;
/// Register offset for FIOPAD_E43_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E43_REG0_OFFSET: u32 = 0x01D8;
/// Register offset for FIOPAD_L43_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L43_REG0_OFFSET: u32 = 0x01DC;
/// Register offset for FIOPAD_C43_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_C43_REG0_OFFSET: u32 = 0x01E0;
/// Register offset for FIOPAD_E41_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E41_REG0_OFFSET: u32 = 0x01E4;
/// Register offset for FIOPAD_L45_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L45_REG0_OFFSET: u32 = 0x01E8;
/// Register offset for FIOPAD_J43_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J43_REG0_OFFSET: u32 = 0x01EC;
/// Register offset for FIOPAD_J41_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J41_REG0_OFFSET: u32 = 0x01F0;
/// Register offset for FIOPAD_L39_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L39_REG0_OFFSET: u32 = 0x01F4;
/// Register offset for FIOPAD_E37_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E37_REG0_OFFSET: u32 = 0x01F8;
/// Register offset for FIOPAD_E35_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E35_REG0_OFFSET: u32 = 0x01FC;
/// Register offset for FIOPAD_G35_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_G35_REG0_OFFSET: u32 = 0x0200;
/// Register offset for FIOPAD_J35_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J35_REG0_OFFSET: u32 = 0x0204;
/// Register offset for FIOPAD_L37_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L37_REG0_OFFSET: u32 = 0x0208;
/// Register offset for FIOPAD_N35_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N35_REG0_OFFSET: u32 = 0x020C;
/// Register offset for FIOPAD_R51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_R51_REG0_OFFSET: u32 = 0x0210;
/// Register offset for FIOPAD_R49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_R49_REG0_OFFSET: u32 = 0x0214;
/// Register offset for FIOPAD_N51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N51_REG0_OFFSET: u32 = 0x0218;
/// Register offset for FIOPAD_N55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N55_REG0_OFFSET: u32 = 0x021C;
/// Register offset for FIOPAD_L55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L55_REG0_OFFSET: u32 = 0x0220;
/// Register offset for FIOPAD_J55_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J55_REG0_OFFSET: u32 = 0x0224;
/// Register offset for FIOPAD_J45_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J45_REG0_OFFSET: u32 = 0x0228;
/// Register offset for FIOPAD_E47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_E47_REG0_OFFSET: u32 = 0x022C;
/// Register offset for FIOPAD_G47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_G47_REG0_OFFSET: u32 = 0x0230;
/// Register offset for FIOPAD_J47_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J47_REG0_OFFSET: u32 = 0x0234;
/// Register offset for FIOPAD_J49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J49_REG0_OFFSET: u32 = 0x0238;
/// Register offset for FIOPAD_N49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N49_REG0_OFFSET: u32 = 0x023C;
/// Register offset for FIOPAD_L51_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L51_REG0_OFFSET: u32 = 0x0240;
/// Register offset for FIOPAD_L49_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_L49_REG0_OFFSET: u32 = 0x0244;
/// Register offset for FIOPAD_N53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_N53_REG0_OFFSET: u32 = 0x0248;
/// Register offset for FIOPAD_J53_REG0_OFFSET function, pull, and drive strength configuration
pub const FIOPAD_J53_REG0_OFFSET: u32 = 0x024C;

/// Beginning offset of REG0 register range
pub const FIOPAD_REG0_BEG_OFFSET: u32 = FIOPAD_AN59_REG0_OFFSET;
/// End offset of REG0 register range
pub const FIOPAD_REG0_END_OFFSET: u32 = FIOPAD_J53_REG0_OFFSET;

// register offset of iopad delay
/// Register offset for FIOPAD_AJ55_REG1_OFFSET delay configuration
pub const FIOPAD_AJ55_REG1_OFFSET: u32 = 0x1024;
/// Register offset for FIOPAD_AL55_REG1_OFFSET delay configuration
pub const FIOPAD_AL55_REG1_OFFSET: u32 = 0x1028;
/// Register offset for FIOPAD_AL53_REG1_OFFSET delay configuration
pub const FIOPAD_AL53_REG1_OFFSET: u32 = 0x102C;
/// Register offset for FIOPAD_AN51_REG1_OFFSET delay configuration
pub const FIOPAD_AN51_REG1_OFFSET: u32 = 0x1030;
/// Register offset for FIOPAD_AR51_REG1_OFFSET delay configuration
pub const FIOPAD_AR51_REG1_OFFSET: u32 = 0x1034;
/// Register offset for FIOPAD_AJ57_REG1_OFFSET delay configuration
pub const FIOPAD_AJ57_REG1_OFFSET: u32 = 0x1058;
/// Register offset for FIOPAD_AG59_REG1_OFFSET delay configuration
pub const FIOPAD_AG59_REG1_OFFSET: u32 = 0x105C;
/// Register offset for FIOPAD_AG57_REG1_OFFSET delay configuration
pub const FIOPAD_AG57_REG1_OFFSET: u32 = 0x1060;
/// Register offset for FIOPAD_AE59_REG1_OFFSET delay configuration
pub const FIOPAD_AE59_REG1_OFFSET: u32 = 0x1064;
/// Register offset for FIOPAD_BA55_REG1_OFFSET delay configuration
pub const FIOPAD_BA55_REG1_OFFSET: u32 = 0x1074;
/// Register offset for FIOPAD_BA53_REG1_OFFSET delay configuration
pub const FIOPAD_BA53_REG1_OFFSET: u32 = 0x1078;
/// Register offset for FIOPAD_AR59_REG1_OFFSET delay configuration
pub const FIOPAD_AR59_REG1_OFFSET: u32 = 0x107C;
/// Register offset for FIOPAD_AU59_REG1_OFFSET delay configuration
pub const FIOPAD_AU59_REG1_OFFSET: u32 = 0x1080;
/// Register offset for FIOPAD_A45_REG1_OFFSET delay configuration
pub const FIOPAD_A45_REG1_OFFSET: u32 = 0x10D0;
/// Register offset for FIOPAD_C45_REG1_OFFSET delay configuration
pub const FIOPAD_C45_REG1_OFFSET: u32 = 0x10D4;
/// Register offset for FIOPAD_A47_REG1_OFFSET delay configuration
pub const FIOPAD_A47_REG1_OFFSET: u32 = 0x10D8;
/// Register offset for FIOPAD_A49_REG1_OFFSET delay configuration
pub const FIOPAD_A49_REG1_OFFSET: u32 = 0x10DC;
/// Register offset for FIOPAD_C49_REG1_OFFSET delay configuration
pub const FIOPAD_C49_REG1_OFFSET: u32 = 0x10E0;
/// Register offset for FIOPAD_A51_REG1_OFFSET delay configuration
pub const FIOPAD_A51_REG1_OFFSET: u32 = 0x10E4;
/// Register offset for FIOPAD_A33_REG1_OFFSET delay configuration
pub const FIOPAD_A33_REG1_OFFSET: u32 = 0x10E8;
/// Register offset for FIOPAD_C33_REG1_OFFSET delay configuration
pub const FIOPAD_C33_REG1_OFFSET: u32 = 0x10EC;
/// Register offset for FIOPAD_C31_REG1_OFFSET delay configuration
pub const FIOPAD_C31_REG1_OFFSET: u32 = 0x10F0;
/// Register offset for FIOPAD_A31_REG1_OFFSET delay configuration
pub const FIOPAD_A31_REG1_OFFSET: u32 = 0x10F4;
/// Register offset for FIOPAD_AJ53_REG1_OFFSET delay configuration
pub const FIOPAD_AJ53_REG1_OFFSET: u32 = 0x10F8;
/// Register offset for FIOPAD_AL49_REG1_OFFSET delay configuration
pub const FIOPAD_AL49_REG1_OFFSET: u32 = 0x10FC;
/// Register offset for FIOPAD_AL47_REG1_OFFSET delay configuration
pub const FIOPAD_AL47_REG1_OFFSET: u32 = 0x1100;
/// Register offset for FIOPAD_AN49_REG1_OFFSET delay configuration
pub const FIOPAD_AN49_REG1_OFFSET: u32 = 0x1104;
/// Register offset for FIOPAD_AG51_REG1_OFFSET delay configuration
pub const FIOPAD_AG51_REG1_OFFSET: u32 = 0x1108;
/// Register offset for FIOPAD_AJ51_REG1_OFFSET delay configuration
pub const FIOPAD_AJ51_REG1_OFFSET: u32 = 0x110C;
/// Register offset for FIOPAD_AG49_REG1_OFFSET delay configuration
pub const FIOPAD_AG49_REG1_OFFSET: u32 = 0x1110;
/// Register offset for FIOPAD_AE55_REG1_OFFSET delay configuration
pub const FIOPAD_AE55_REG1_OFFSET: u32 = 0x1114;
/// Register offset for FIOPAD_AE53_REG1_OFFSET delay configuration
pub const FIOPAD_AE53_REG1_OFFSET: u32 = 0x1118;
/// Register offset for FIOPAD_AG55_REG1_OFFSET delay configuration
pub const FIOPAD_AG55_REG1_OFFSET: u32 = 0x111C;
/// Register offset for FIOPAD_AJ49_REG1_OFFSET delay configuration
pub const FIOPAD_AJ49_REG1_OFFSET: u32 = 0x1120;
/// Register offset for FIOPAD_AC55_REG1_OFFSET delay configuration
pub const FIOPAD_AC55_REG1_OFFSET: u32 = 0x1124;
/// Register offset for FIOPAD_AC53_REG1_OFFSET delay configuration
pub const FIOPAD_AC53_REG1_OFFSET: u32 = 0x1128;
/// Register offset for FIOPAD_AE51_REG1_OFFSET delay configuration
pub const FIOPAD_AE51_REG1_OFFSET: u32 = 0x112C;
/// Register offset for FIOPAD_W51_REG1_OFFSET delay configuration
pub const FIOPAD_W51_REG1_OFFSET: u32 = 0x1130;
/// Register offset for FIOPAD_W53_REG1_OFFSET delay configuration
pub const FIOPAD_W53_REG1_OFFSET: u32 = 0x1138;
/// Register offset for FIOPAD_U55_REG1_OFFSET delay configuration
pub const FIOPAD_U55_REG1_OFFSET: u32 = 0x113C;
/// Register offset for FIOPAD_U53_REG1_OFFSET delay configuration
pub const FIOPAD_U53_REG1_OFFSET: u32 = 0x1140;
/// Register offset for FIOPAD_AE49_REG1_OFFSET delay configuration
pub const FIOPAD_AE49_REG1_OFFSET: u32 = 0x1144;
/// Register offset for FIOPAD_AC49_REG1_OFFSET delay configuration
pub const FIOPAD_AC49_REG1_OFFSET: u32 = 0x1148;
/// Register offset for FIOPAD_AE47_REG1_OFFSET delay configuration
pub const FIOPAD_AE47_REG1_OFFSET: u32 = 0x114C;
/// Register offset for FIOPAD_AA47_REG1_OFFSET delay configuration
pub const FIOPAD_AA47_REG1_OFFSET: u32 = 0x1150;
/// Register offset for FIOPAD_AA49_REG1_OFFSET delay configuration
pub const FIOPAD_AA49_REG1_OFFSET: u32 = 0x1154;
/// Register offset for FIOPAD_W49_REG1_OFFSET delay configuration
pub const FIOPAD_W49_REG1_OFFSET: u32 = 0x1158;
/// Register offset for FIOPAD_AA51_REG1_OFFSET delay configuration
pub const FIOPAD_AA51_REG1_OFFSET: u32 = 0x115C;
/// Register offset for FIOPAD_U49_REG1_OFFSET delay configuration
pub const FIOPAD_U49_REG1_OFFSET: u32 = 0x1160;
/// Register offset for FIOPAD_J59_REG1_OFFSET delay configuration
pub const FIOPAD_J59_REG1_OFFSET: u32 = 0x1168;
/// Register offset for FIOPAD_L57_REG1_OFFSET delay configuration
pub const FIOPAD_L57_REG1_OFFSET: u32 = 0x116C;
/// Register offset for FIOPAD_C59_REG1_OFFSET delay configuration
pub const FIOPAD_C59_REG1_OFFSET: u32 = 0x1170;
/// Register offset for FIOPAD_E59_REG1_OFFSET delay configuration
pub const FIOPAD_E59_REG1_OFFSET: u32 = 0x1174;
/// Register offset for FIOPAD_J57_REG1_OFFSET delay configuration
pub const FIOPAD_J57_REG1_OFFSET: u32 = 0x1178;
/// Register offset for FIOPAD_L59_REG1_OFFSET delay configuration
pub const FIOPAD_L59_REG1_OFFSET: u32 = 0x117C;
/// Register offset for FIOPAD_N59_REG1_OFFSET delay configuration
pub const FIOPAD_N59_REG1_OFFSET: u32 = 0x1180;
/// Register offset for FIOPAD_E31_REG1_OFFSET delay configuration
pub const FIOPAD_E31_REG1_OFFSET: u32 = 0x118C;
/// Register offset for FIOPAD_G31_REG1_OFFSET delay configuration
pub const FIOPAD_G31_REG1_OFFSET: u32 = 0x1190;
/// Register offset for FIOPAD_N41_REG1_OFFSET delay configuration
pub const FIOPAD_N41_REG1_OFFSET: u32 = 0x1194;
/// Register offset for FIOPAD_N39_REG1_OFFSET delay configuration
pub const FIOPAD_N39_REG1_OFFSET: u32 = 0x1198;
/// Register offset for FIOPAD_J33_REG1_OFFSET delay configuration
pub const FIOPAD_J33_REG1_OFFSET: u32 = 0x119C;
/// Register offset for FIOPAD_N33_REG1_OFFSET delay configuration
pub const FIOPAD_N33_REG1_OFFSET: u32 = 0x11A0;
/// Register offset for FIOPAD_L33_REG1_OFFSET delay configuration
pub const FIOPAD_L33_REG1_OFFSET: u32 = 0x11A4;
/// Register offset for FIOPAD_N45_REG1_OFFSET delay configuration
pub const FIOPAD_N45_REG1_OFFSET: u32 = 0x11A8;
/// Register offset for FIOPAD_N43_REG1_OFFSET delay configuration
pub const FIOPAD_N43_REG1_OFFSET: u32 = 0x11AC;
/// Register offset for FIOPAD_L31_REG1_OFFSET delay configuration
pub const FIOPAD_L31_REG1_OFFSET: u32 = 0x11B0;
/// Register offset for FIOPAD_J31_REG1_OFFSET delay configuration
pub const FIOPAD_J31_REG1_OFFSET: u32 = 0x11B4;
/// Register offset for FIOPAD_J29_REG1_OFFSET delay configuration
pub const FIOPAD_J29_REG1_OFFSET: u32 = 0x11B8;
/// Register offset for FIOPAD_E29_REG1_OFFSET delay configuration
pub const FIOPAD_E29_REG1_OFFSET: u32 = 0x11BC;
/// Register offset for FIOPAD_G29_REG1_OFFSET delay configuration
pub const FIOPAD_G29_REG1_OFFSET: u32 = 0x11C0;
/// Register offset for FIOPAD_J37_REG1_OFFSET delay configuration
pub const FIOPAD_J37_REG1_OFFSET: u32 = 0x11CC;
/// Register offset for FIOPAD_J39_REG1_OFFSET delay configuration
pub const FIOPAD_J39_REG1_OFFSET: u32 = 0x11D0;
/// Register offset for FIOPAD_G41_REG1_OFFSET delay configuration
pub const FIOPAD_G41_REG1_OFFSET: u32 = 0x11D4;
/// Register offset for FIOPAD_E43_REG1_OFFSET delay configuration
pub const FIOPAD_E43_REG1_OFFSET: u32 = 0x11D8;
/// Register offset for FIOPAD_L43_REG1_OFFSET delay configuration
pub const FIOPAD_L43_REG1_OFFSET: u32 = 0x11DC;
/// Register offset for FIOPAD_C43_REG1_OFFSET delay configuration
pub const FIOPAD_C43_REG1_OFFSET: u32 = 0x11E0;
/// Register offset for FIOPAD_E41_REG1_OFFSET delay configuration
pub const FIOPAD_E41_REG1_OFFSET: u32 = 0x11E4;
/// Register offset for FIOPAD_L45_REG1_OFFSET delay configuration
pub const FIOPAD_L45_REG1_OFFSET: u32 = 0x11E8;
/// Register offset for FIOPAD_J43_REG1_OFFSET delay configuration
pub const FIOPAD_J43_REG1_OFFSET: u32 = 0x11EC;
/// Register offset for FIOPAD_J41_REG1_OFFSET delay configuration
pub const FIOPAD_J41_REG1_OFFSET: u32 = 0x11F0;
/// Register offset for FIOPAD_L39_REG1_OFFSET delay configuration
pub const FIOPAD_L39_REG1_OFFSET: u32 = 0x11F4;
/// Register offset for FIOPAD_E37_REG1_OFFSET delay configuration
pub const FIOPAD_E37_REG1_OFFSET: u32 = 0x11F8;
/// Register offset for FIOPAD_E35_REG1_OFFSET delay configuration
pub const FIOPAD_E35_REG1_OFFSET: u32 = 0x11FC;
/// Register offset for FIOPAD_G35_REG1_OFFSET delay configuration
pub const FIOPAD_G35_REG1_OFFSET: u32 = 0x1200;
/// Register offset for FIOPAD_L55_REG1_OFFSET delay configuration
pub const FIOPAD_L55_REG1_OFFSET: u32 = 0x1220;
/// Register offset for FIOPAD_J55_REG1_OFFSET delay configuration
pub const FIOPAD_J55_REG1_OFFSET: u32 = 0x1224;
/// Register offset for FIOPAD_J45_REG1_OFFSET delay configuration
pub const FIOPAD_J45_REG1_OFFSET: u32 = 0x1228;
/// Register offset for FIOPAD_E47_REG1_OFFSET delay configuration
pub const FIOPAD_E47_REG1_OFFSET: u32 = 0x122C;
/// Register offset for FIOPAD_G47_REG1_OFFSET delay configuration
pub const FIOPAD_G47_REG1_OFFSET: u32 = 0x1230;
/// Register offset for FIOPAD_J47_REG1_OFFSET delay configuration
pub const FIOPAD_J47_REG1_OFFSET: u32 = 0x1234;
/// Register offset for FIOPAD_J49_REG1_OFFSET delay configuration
pub const FIOPAD_J49_REG1_OFFSET: u32 = 0x1238;
/// Register offset for FIOPAD_N49_REG1_OFFSET delay configuration
pub const FIOPAD_N49_REG1_OFFSET: u32 = 0x123C;
/// Register offset for FIOPAD_L51_REG1_OFFSET delay configuration
pub const FIOPAD_L51_REG1_OFFSET: u32 = 0x1240;
/// Register offset for FIOPAD_L49_REG1_OFFSET delay configuration
pub const FIOPAD_L49_REG1_OFFSET: u32 = 0x1244;
/// Register offset for FIOPAD_N53_REG1_OFFSET delay configuration
pub const FIOPAD_N53_REG1_OFFSET: u32 = 0x1248;
/// Register offset for FIOPAD_J53_REG1_OFFSET delay configuration
pub const FIOPAD_J53_REG1_OFFSET: u32 = 0x124C;

/// Beginning offset of REG1 register range
pub const FIOPAD_REG1_BEG_OFFSET: u32 = FIOPAD_AJ55_REG1_OFFSET;
/// End offset of REG1 register range
pub const FIOPAD_REG1_END_OFFSET: u32 = FIOPAD_J53_REG1_OFFSET;

/// Maximum delay value for IOPAD delay tuning.
pub const FIOPAD_DELAY_MAX: u32 = 15;

/// Base address for the IOPAD controller registers.
pub const PAD_ADDRESS: u32 = 0x000_32B3_0000;
