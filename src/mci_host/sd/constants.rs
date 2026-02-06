//! # SD Card Constants
//!
//! This module defines constants and enumerations specific to SD card operations.
//!
//! ## Contents
//!
//! - Timing modes (SDR12, SDR25, SDR50, SDR104, DDR50)
//! - Driver strength types
//! - Current limits
//! - SD commands and application commands
//! - Card capability flags

use bitflags::bitflags;

/// SD bus timing mode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdTimingMode {
    /// SDR12 default mode (25 MHz at 3.3V, 12.5 MHz at 1.8V)
    SDR12DefaultMode = 0,
    /// SDR25 high speed mode (50 MHz)
    SDR25HighSpeedMode = 1,
    /// SDR50 mode (100 MHz)
    SDR50Mode = 2,
    /// SDR104 mode (208 MHz)
    SDR104Mode = 3,
    /// DDR50 mode (50 MHz)
    DDR50Mode = 4,
}

/// SD driver strength type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdDriverStrength {
    /// Type B (default)
    TypeB = 0,
    /// Type A
    TypeA = 1,
    /// Type C
    TypeC = 2,
    /// Type D
    TypeD = 3,
}

/// SD maximum current limit enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdMaxCurrent {
    /// 200 mA
    Limit200mA = 0,
    /// 400 mA
    Limit400mA = 1,
    /// 600 mA
    Limit600mA = 2,
    /// 800 mA
    Limit800mA = 3,
}

/// SD I/O voltage control type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdIoVoltageCtrlType {
    /// Voltage switching not supported
    NotSupport = 0,
    /// Voltage switching controlled by host
    ByHost = 1,
    /// Voltage switching controlled via GPIO
    ByGpio = 2,
}

/// SD command enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SdCmd {
    /// CMD3: Send Relative Address
    SendRelativeAddress = 3,
    /// CMD6: Switch Function
    Switch = 6,
    /// CMD8: Send Interface Condition
    SendInterfaceCondition = 8,
    /// CMD11: Voltage Switch
    VoltageSwitch = 11,
    /// CMD20: Speed Class Control
    SpeedClassControl = 20,
    /// CMD19: Send Tuning Block
    SendTuningBlock = 19,
    /// CMD32: Erase Write Block Start
    EraseWriteBlockStart = 32,
    /// CMD33: Erase Write Block End
    EraseWriteBlockEnd = 33,
}

/// SD application command enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdAppCmd {
    /// ACMD6: Set Bus Width
    SetBusWdith = 6,
    /// ACMD13: Send SD Status
    Status = 13,
    /// ACMD22: Send Number Of Written Blocks
    SendNumberWriteBlocks = 22,
    /// ACMD23: Set Write Block Erase Count
    SetWriteBlockEraseCount = 23,
    /// ACMD41: Send Operation Condition
    SendOperationCondition = 41,
    /// ACMD42: Set/Clear Card Detect
    SetClearCardDetect = 42,
    /// ACMD51: Send SCR
    SendScr = 51,
}

/// SD switch mode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdSwitchMode {
    /// Check mode
    Check = 0,
    /// Set mode
    Set = 1,
}

/// SD function group enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdGroupNum {
    /// Timing mode group
    TimingMode = 0,
    /// Command system group
    CommandSystem = 1,
    /// Driver strength group
    DriverStrength = 2,
    /// Current limit group
    CurrentLimit = 3,
}

/// SD timing function enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdTimingFuncNum {
    /// SDR12 default
    SDR12Default = 0,
    /// SDR25 high speed
    SDR25HighSpeed = 1,
    /// SDR50
    SDR50 = 2,
    /// SDR104
    SDR104 = 3,
    /// DDR50
    DDR50 = 4,
}

bitflags! {
    /// SD card flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SdCardFlag: u32 {
        /// Support high capacity
        const SupportHighCapacity = 1 << 1;
        /// Support 4-bit data width
        const Support4BitWidth = 1 << 2;
        /// Card is SDHC
        const SupportSdhc = 1 << 3;
        /// Card is SDXC
        const SupportSdxc = 1 << 4;
        /// Card supports 1.8v voltage
        const SupportVoltage180v = 1 << 5;
        /// Card supports CMD23 (SET_BLOCK_COUNT)
        const SupportSetBlockCountCmd = 1 << 6;
        /// Card supports speed class control
        const SupportSpeedClassControlCmd = 1 << 7;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SdSpecificationVersion {
    Version1_0 = 1 << 0,
    Version1_1 = 1 << 1,
    Version2_0 = 1 << 2,
    Version3_0 = 1 << 3,
}

pub(crate) const SD_POWER_ON_DELAY_MS: u32 = 400;
pub(crate) const SD_POWER_OFF_DELAY_MS: u32 = 100;

pub(crate) const SD_CLOCK_25MHZ: u32 = 25_000_000;
pub(crate) const SD_CLOCK_50MHZ: u32 = 50_000_000;
pub(crate) const SD_CLOCK_100MHZ: u32 = 100_000_000;
pub(crate) const SD_CLOCK_208MHZ: u32 = 208_000_000;

pub(crate) const SD_CMD13_RETRY_TIMES: u32 = 10;
pub(crate) const SD_PRODUCT_NAME_BYTES: usize = 5;
pub(crate) const SD_MAX_RW_BLK: usize = 1024;
pub(crate) const SD_BLOCK_SIZE: usize = 512;

pub(crate) const SD_CARD_ACCESS_WAIT_IDLE_TIMEOUT: u32 = 600;
