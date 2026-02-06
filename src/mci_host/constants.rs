//! # MCI Host Constants
//!
//! This module defines constants and enumerations for the MCI host controller including:
//! - Command types and response types
//! - Common commands (SD/MMC)
//! - Card status flags
//! - OCR register flags
//! - Host capability flags
//! - Size constants

use bitflags::bitflags;

/// Command type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCIHostCmdType {
    /// Normal command
    Normal = 0,
    /// Suspend command
    Suspend = 1,
    /// Resume command
    Resume = 2,
    /// Abort command
    Abort = 3,
    /// Empty command
    Empty = 4,
}

/// Response type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCIHostResponseType {
    /// No response
    None = 0,
    /// R1 response
    R1 = 1,
    /// R1b response (with busy)
    R1b = 2,
    /// R2 response (136-bit)
    R2 = 3,
    /// R3 response (OCR)
    R3 = 4,
    /// R4 response
    R4 = 5,
    /// R5 response
    R5 = 6,
    /// R5b response (with busy)
    R5b = 7,
    /// R6 response
    R6 = 8,
    /// R7 response
    R7 = 9,
}

/// Data packet format enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCIHostDataPacketFormat {
    /// MSB first format
    MSBFirst = 0,
    /// LSB first format
    LSBFirst = 1,
}

/// Bus width enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCIHostBusWdith {
    /// 1-bit bus width
    Bit1 = 1,
    /// 4-bit bus width
    Bit4 = 4,
    /// 8-bit bus width
    Bit8 = 8,
}

/// Common SD/MMC commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCIHostCommonCmd {
    /// CMD0: Go Idle State
    GoIdleState = 0,
    /// CMD2: All Send CID
    AllSendCid = 2,
    /// CMD4: Set DSR
    SetDsr = 4,
    /// CMD7: Select Card
    SelectCard = 7,
    /// CMD9: Send CSD
    SendCsd = 9,
    /// CMD10: Send CID
    SendCid = 10,
    /// CMD12: Stop Transmission
    StopTransmission = 12,
    /// CMD13: Send Status
    SendStatus = 13,
    /// CMD15: Go Inactive State
    GoInactiveState = 15,
    /// CMD16: Set Block Length
    SetBlockLength = 16,
    /// CMD17: Read Single Block
    ReadSingleBlock = 17,
    /// CMD18: Read Multiple Block
    ReadMultipleBlock = 18,
    /// CMD23: Set Block Count
    SetBlockCount = 23,
    /// CMD24: Write Single Block
    WriteSingleBlock = 24,
    /// CMD25: Write Multiple Block
    WriteMultipleBlock = 25,
    /// CMD27: Program CSD
    ProgramCsd = 27,
    /// CMD28: Set Write Protect
    SetWriteProtect = 28,
    /// CMD29: Clear Write Protect
    ClearWriteProtect = 29,
    /// CMD30: Send Write Protect
    SendWriteProtect = 30,
    /// CMD38: Erase
    Erase = 38,
    /// CMD42: Lock Unlock
    LockUnlock = 42,
    /// CMD55: Application Command
    ApplicationCommand = 55,
    /// CMD56: General Purpose Command
    GeneralCommand = 56,
    /// CMD58: Read OCR
    ReadOcr = 58,
}

/// SDIO command enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCISDIOCommand {
    /// CMD3: Send Relative Address
    SendRelativeAddress = 3,
    /// CMD5: Send Operation Condition
    SendOperationCondition = 5,
    /// CMD8: Send Interface Condition
    SendInterfaceCondition = 8,
    /// CMD52: Read/Write I/O Direct
    RWIODirect = 52,
    /// CMD53: Read/Write I/O Direct Extended
    RWIODirectExtended = 53,
}

/// SDIO CCCR address enumeration.
///
/// CCCR (SDIO Card Common Control Register) addresses for SDIO card configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCISDIOCCCRAddr {
    /// CCCR & SDIO version
    SDIOVer = 0x00,
    /// SD version
    SDVersion = 0x01,
    /// I/O enable register
    IOEnable = 0x02,
    /// I/O ready register
    IOReady = 0x03,
    /// I/O interrupt enable register
    IOIntEnable = 0x04,
    /// I/O interrupt pending register
    IOIntPending = 0x05,
    /// I/O abort register
    IOAbort = 0x06,
    /// Bus interface register
    BusInterface = 0x07,
    /// Card capability register
    CardCapability = 0x08,
    /// Common CIS pointer register
    CommonCISPointer = 0x09,
    /// Bus suspend register
    BusSuspend = 0x0C,
    /// Function select register
    FunctionSelect = 0x0D,
    /// Execution flag register
    ExecutionFlag = 0x0E,
    /// Ready flag register
    ReadyFlag = 0x0F,
    /// FN0 block size register (low byte)
    FN0BlockSizeLow = 0x10,
    /// FN0 block size register (high byte)
    FN0BlockSizeHigh = 0x11,
    /// Power control register
    PowerControl = 0x12,
    /// Bus speed register
    BusSpeed = 0x13,
    /// UHS-I timing support register
    UHSITimingSupport = 0x14,
    /// Driver strength register
    DriverStrength = 0x15,
    /// Interrupt extension register
    InterruptExtension = 0x16,
}

bitflags! {
    /// Card status flags in SD card response.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MCIHostCardStatusFlag: u32 {
        /// Out of range status bit
        const OUT_OF_RANGE = 1 << 31;
        /// Address error status bit
        const ADDRESS_ERROR = 1 << 30;
        /// Block length error status bit
        const BLOCK_LENGTH_ERROR = 1 << 29;
        /// Erase sequence error status bit
        const ERASE_SEQUENCE_ERROR = 1 << 28;
        /// Erase parameter error status bit
        const ERASE_PARAMETER_ERROR = 1 << 27;
        /// Write protection violation status bit
        const WRITE_PROTECT_VIOLATION = 1 << 26;
        /// Card locked status bit
        const CARD_IS_LOCKED = 1 << 25;
        /// Lock/unlock error status bit
        const LOCK_UNLOCK_FAILED = 1 << 24;
        /// CRC error status bit
        const COMMAND_CRC_ERROR = 1 << 23;
        /// Illegal command status bit
        const ILLEGAL_COMMAND = 1 << 22;
        /// Card ECC error status bit
        const CARD_ECC_FAILED = 1 << 21;
        /// Internal card controller error status bit
        const CARD_CONTROLLER_ERROR = 1 << 20;
        /// A general or an unknown error status bit
        const ERROR = 1 << 19;
        /// CID/CSD overwrite status bit
        const CID_CSD_OVERWRITE = 1 << 16;
        /// Write protection erase skip status bit
        const WRITE_PROTECT_ERASE_SKIP = 1 << 15;
        /// Card ECC disabled status bit
        const CARD_ECC_DISABLED = 1 << 14;
        /// Erase reset status bit
        const ERASE_RESET = 1 << 13;
        /// Ready for data status bit
        const READY_FOR_DATA = 1 << 8;
        /// Switch error status bit
        const SWITCH_ERROR = 1 << 7;
        /// Application command enabled status bit
        const APPLICATION_COMMAND = 1 << 5;
        /// Error in the sequence of authentication process
        const AUTHENTICATION_SEQUENCE_ERROR = 1 << 3;
        /// All error status bits
        const ALL_ERROR_FLAG = 0xFFF90008;
    }
}

/// Card current state enumeration (internal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostCurrentState {
    Idle = 0,
    Ready = 1,
    Identification = 2,
    Standby = 3,
    Transfer = 4,
    Data = 5,
    Receive = 6,
    Programming = 7,
    Disconnect = 8,
}

impl MCIHostCurrentState {
    pub(crate) fn current_state(state: u32) -> Self {
        let state = (state & 0x00001E00) >> 9;
        match state {
            0 => MCIHostCurrentState::Idle,
            1 => MCIHostCurrentState::Ready,
            2 => MCIHostCurrentState::Identification,
            3 => MCIHostCurrentState::Standby,
            4 => MCIHostCurrentState::Transfer,
            5 => MCIHostCurrentState::Data,
            6 => MCIHostCurrentState::Receive,
            7 => MCIHostCurrentState::Programming,
            8 => MCIHostCurrentState::Disconnect,
            _ => MCIHostCurrentState::Idle,
        }
    }
}

/// Operation voltage enumeration (internal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostOperationVoltage {
    None = 0,
    /// 3.3V operation
    Voltage330V = 1,
    /// 3.0V operation
    Voltage300V = 2,
    /// 1.8V operation
    Voltage180V = 3,
}

/// Card detection type enumeration (internal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostDetectCardType {
    /// Detection via GPIO card detect pin
    ByGpioCD,
    /// Detection via host controller card detect
    ByHostCD,
    /// Detection via DATA3 pin
    ByHostDATA3,
}

/// Default clock frequency for card initialization (400 kHz)
pub(crate) const MCI_HOST_CLOCK_400KHZ: u32 = 400_000;
/// Maximum number of command retries
pub(crate) const MCI_HOST_MAX_CMD_RETRIES: u32 = 10;
/// Default block size (512 bytes)
pub(crate) const MCI_HOST_DEFAULT_BLOCK_SIZE: u32 = 512;
/// Maximum block length (4096 bytes)
pub(crate) const MCI_HOST_MAX_BLOCK_LENGTH: u32 = 4096;

bitflags! {
    /// OCR register flags in SD card
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct MCIHostOCR: u32 {
        /// Power up busy status (bit 31)
        const POWER_UP_BUSY_FLAG = 1 << 31;

        /// Card/Host capacity status (bit 30)
        const HOST_CAPACITY_SUPPORT_FLAG = 1 << 30;
        /// Card capacity status (bit 30, same as HOST_CAPACITY_SUPPORT_FLAG)
        const CARD_CAPACITY_SUPPORT_FLAG = 1 << 30;

        /// Switch to 1.8V request (bit 24)
        const SWITCH_18_REQUEST_FLAG = 1 << 24;
        /// Switch to 1.8V accepted (bit 24, same as SWITCH_18_REQUEST_FLAG)
        const SWITCH_18_ACCEPT_FLAG = 1 << 24;

        /// VDD 2.7-2.8V (bit 15)
        const VDD_27_28 = 1 << 15;
        /// VDD 2.8-2.9V (bit 16)
        const VDD_28_29 = 1 << 16;
        /// VDD 2.9-3.0V (bit 17)
        const VDD_29_30 = 1 << 17;
        /// VDD 3.0-3.1V (bit 18)
        const VDD_30_31 = 1 << 18;
        /// VDD 3.1-3.2V (bit 19)
        const VDD_31_32 = 1 << 19;
        /// VDD 3.2-3.3V (bit 20)
        const VDD_32_33 = 1 << 20;
        /// VDD 3.3-3.4V (bit 21)
        const VDD_33_34 = 1 << 21;
        /// VDD 3.4-3.5V (bit 22)
        const VDD_34_35 = 1 << 22;
        /// VDD 3.5-3.6V (bit 23)
        const VDD_35_36 = 1 << 23;
    }
}

bitflags! {
    /// SDMMC host capability flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct MCIHostCapability: u32 {
        /// High speed capability
        const HIGH_SPEED = 1 << 0;
        /// Suspend resume capability
        const SUSPEND_RESUME = 1 << 1;
        /// 3.3V capability
        const VOLTAGE_3V3 = 1 << 2;
        /// 3.0V capability
        const VOLTAGE_3V0 = 1 << 3;
        /// 1.8V capability
        const VOLTAGE_1V8 = 1 << 4;
        /// 1.2V capability
        const VOLTAGE_1V2 = 1 << 5;
        /// 4-bit data width capability
        const BIT4_DATA_WIDTH = 1 << 6;
        /// 8-bit data width capability
        const BIT8_DATA_WIDTH = 1 << 7;
        /// DDR mode capability
        const DDR_MODE = 1 << 8;
        /// Data3 detect card capability
        const DETECT_CARD_BY_DATA3 = 1 << 9;
        /// CD detect card capability
        const DETECT_CARD_BY_CD = 1 << 10;
        /// Auto command 12 capability
        const AUTO_CMD12 = 1 << 11;
        /// SDR104 capability
        const SDR104 = 1 << 12;
        /// SDR50 capability
        const SDR50 = 1 << 13;
        /// HS200 capability
        const HS200 = 1 << 14;
        /// HS400 capability
        const HS400 = 1 << 15;
        /// Driver Type C capability
        const DRIVER_TYPE_C = 1 << 16;
        /// Set current limit capability
        const SET_CURRENT = 1 << 17;
    }
}

bitflags! {
    /// Extended host capability flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct MCIHostCapabilityExt: u32 {
        /// 8-bit data width capability
        const BIT8_WIDTH = 1;
    }
}

// Size constants

pub const SZ_1: u64 = 0x00000001;
pub const SZ_2: u64 = 0x00000002;
pub const SZ_4: u64 = 0x00000004;
pub const SZ_8: u64 = 0x00000008;
pub const SZ_16: u64 = 0x00000010;
pub const SZ_32: u64 = 0x00000020;
pub const SZ_64: u64 = 0x00000040;
pub const SZ_128: u64 = 0x00000080;
pub const SZ_256: u64 = 0x00000100;
pub const SZ_512: u64 = 0x00000200;

pub const SZ_1K: u64 = 0x00000400;
pub const SZ_2K: u64 = 0x00000800;
pub const SZ_4K: u64 = 0x00001000;
pub const SZ_8K: u64 = 0x00002000;
pub const SZ_16K: u64 = 0x00004000;
pub const SZ_32K: u64 = 0x00008000;
pub const SZ_64K: u64 = 0x00010000;
pub const SZ_128K: u64 = 0x00020000;
pub const SZ_256K: u64 = 0x00040000;
pub const SZ_512K: u64 = 0x00080000;

pub const SZ_1M: u64 = 0x00100000;
pub const SZ_2M: u64 = 0x00200000;
pub const SZ_4M: u64 = 0x00400000;
pub const SZ_8M: u64 = 0x00800000;
pub const SZ_16M: u64 = 0x01000000;
pub const SZ_32M: u64 = 0x02000000;
pub const SZ_64M: u64 = 0x04000000;
pub const SZ_128M: u64 = 0x08000000;
pub const SZ_256M: u64 = 0x10000000;
pub const SZ_512M: u64 = 0x20000000;

pub const SZ_1G: u64 = 0x40000000;
pub const SZ_2G: u64 = 0x80000000;
pub const SZ_3G: u64 = 0xC0000000;
pub const SZ_4G: u64 = 0x100000000;
pub const SZ_8G: u64 = 0x200000000;
