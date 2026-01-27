//! Constants and enumerations for the MCI driver
//!
//! This module defines all the constants, enums, and bitflags used throughout
//! the MCI driver for hardware configuration and operation.

#![allow(missing_docs)]

use bitflags::bitflags;

/// MCI hardware instance identifiers
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCIId {
    /// MCI instance 0
    MCI0,
    /// MCI instance 1
    MCI1,
}

impl Default for MCIId {
    fn default() -> Self {
        Self::MCI0
    }
}

/// FIFO depth configuration for the MCI controller
///
/// The values correspond to register bit positions for different FIFO sizes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCIFifoDepth {
    /// 8-entry FIFO depth
    Depth8 = 23,
    /// 16-entry FIFO depth
    Depth16 = 24,
    /// 32-entry FIFO depth
    Depth32 = 25,
    /// 64-entry FIFO depth
    Depth64 = 26,
    /// 128-entry FIFO depth
    Depth128 = 27,
}

/// Command flags for MCI operations
///
/// These flags specify the properties and requirements of MCI commands,
/// including whether they expect responses, data transfers, or special handling.
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct MCICmdFlag: u32 {
        /// Command requires card initialization (80 clock cycles)
        const NEED_INIT = 0x1;
        /// Command expects a response from the card
        const EXP_RESP = 0x2;
        /// Command expects a long (128-bit) response
        const EXP_LONG_RESP = 0x4;
        /// Response CRC should be checked
        const NEED_RESP_CRC = 0x8;
        /// Command involves data transfer
        const EXP_DATA = 0x10;
        /// Data transfer is a write operation
        const WRITE_DATA = 0x20;
        /// Data transfer is a read operation
        const READ_DATA = 0x40;
        /// Automatically send CMD12 after data transfer
        const NEED_AUTO_STOP = 0x80;
        /// Application-specific command with data transfer
        const ADTC = 0x100;
        /// Command involves voltage switching
        const SWITCH_VOLTAGE = 0x200;
        /// Command aborts current data transfer
        const ABORT = 0x400;
        /// Auto-send CMD12 at end of transfer
        const AUTO_CMD12 = 0x800;
    }
}

/// Transfer mode enumeration
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCITransMode {
    /// DMA transfer mode
    DMA,
    /// PIO transfer mode (via FIFO read/write)
    PIO,
}

/// Interrupt type enumeration
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCIIntrType {
    /// Controller interrupt status
    GeneralIntr,
    /// DMA interrupt status
    DmaIntr,
}

/// Event type enumeration for MCI operations
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FSdifEvtType {
    /// Card detection event
    CardDetected = 0,
    /// Command transfer complete event
    CmdDone,
    /// Command with data transfer complete event
    DataDone,
    /// SDIO card custom event
    SdioIrq,
    /// Error occurred during transfer
    ErrOccured,
    /// Number of events
    NumOfEvt,
}

/// Clock speed enumeration for MCI operations
///
/// Defines the supported clock frequencies for different SD/MMC speed modes.
#[derive(Debug, PartialEq)]
pub enum MCIClkSpeed {
    /// 400 KHz - Initialization speed
    ClkSpeed400KHz = 400_000,
    /// 25 MHz - Default speed for SD cards
    ClkSpeed25Mhz = 25_000_000,
    /// 26 MHz - MMC speed
    ClkSpeed26Mhz = 26_000_000,
    /// 50 MHz - High speed for SD cards
    ClkSpeed50Mhz = 50_000_000,
    /// 52 MHz - MMC high speed
    ClkSpeed52Mhz = 52_000_000,
    /// 66 MHz - MMC higher speed
    ClkSpeed66Mhz = 66_000_000,
    /// 100 MHz - UHS-I SDR104 / MMC HS200
    ClkSpeed100Mhz = 100_000_000,
}

impl From<u32> for MCIClkSpeed {
    fn from(value: u32) -> Self {
        match value {
            400_000 => MCIClkSpeed::ClkSpeed400KHz,
            25_000_000 => MCIClkSpeed::ClkSpeed25Mhz,
            26_000_000 => MCIClkSpeed::ClkSpeed26Mhz,
            50_000_000 => MCIClkSpeed::ClkSpeed50Mhz,
            52_000_000 => MCIClkSpeed::ClkSpeed52Mhz,
            66_000_000 => MCIClkSpeed::ClkSpeed66Mhz,
            100_000_000 => MCIClkSpeed::ClkSpeed100Mhz,
            _ => panic!("Invalid clock speed"),
        }
    }
}

/// Register Map
///
/// Register offsets from the base address of an SD device.
/// @{ */
/// Controller configuration register offset
pub const FSDIF_CNTRL_OFFSET: u32 = 0x00;
/// Power enable register offset
pub const FSDIF_PWREN_OFFSET: u32 = 0x04;
/// Clock divider register offset
pub const FSDIF_CLKDIV_OFFSET: u32 = 0x08;
/// Clock enable register offset
pub const FSDIF_CLKENA_OFFSET: u32 = 0x10;
/// Timeout register offset
pub const FSDIF_TMOUT_OFFSET: u32 = 0x14;
/// Card type register offset
pub const FSDIF_CTYPE_OFFSET: u32 = 0x18;
/// Block size register offset
pub const FSDIF_BLK_SIZ_OFFSET: u32 = 0x1C;
/// Byte count register offset
pub const FSDIF_BYT_CNT_OFFSET: u32 = 0x20;
/// Interrupt mask register offset
pub const FSDIF_INT_MASK_OFFSET: u32 = 0x24;
/// Command argument register offset
pub const FSDIF_CMD_ARG_OFFSET: u32 = 0x28;
/// Command register offset
pub const FSDIF_CMD_OFFSET: u32 = 0x2C;
/// Response register 0 offset
pub const FSDIF_RESP0_OFFSET: u32 = 0x30;
/// Response register 1 offset
pub const FSDIF_RESP1_OFFSET: u32 = 0x34;
/// Response register 2 offset
pub const FSDIF_RESP2_OFFSET: u32 = 0x38;
/// Response register 3 offset
pub const FSDIF_RESP3_OFFSET: u32 = 0x3C;
/// Masked interrupt status register offset
pub const FSDIF_MASKED_INTS_OFFSET: u32 = 0x40;
/// Raw interrupt status register offset
pub const FSDIF_RAW_INTS_OFFSET: u32 = 0x44;
/// Status register offset
pub const FSDIF_STATUS_OFFSET: u32 = 0x48;
/// FIFO threshold watermark register offset
pub const FSDIF_FIFOTH_OFFSET: u32 = 0x4C;
/// Card detect register offset
pub const FSDIF_CARD_DETECT_OFFSET: u32 = 0x50;
/// Card write protect register offset
pub const FSDIF_CARD_WRTPRT_OFFSET: u32 = 0x54;
/// CIU ready register offset
pub const FSDIF_CKSTS_OFFSET: u32 = 0x58;
/// Transferred CIU card byte count register offset
pub const FSDIF_TRAN_CARD_CNT_OFFSET: u32 = 0x5C;
/// Transferred host to FIFO byte count register offset
pub const FSDIF_TRAN_FIFO_CNT_OFFSET: u32 = 0x60;
/// Debounce count register offset
pub const FSDIF_DEBNCE_OFFSET: u32 = 0x64;
/// User ID register offset
pub const FSDIF_UID_OFFSET: u32 = 0x68;
/// Controller version ID register offset
pub const FSDIF_VID_OFFSET: u32 = 0x6C;
/// Hardware configuration register offset
pub const FSDIF_HWCONF_OFFSET: u32 = 0x70;
/// UHS-I register offset
pub const FSDIF_UHS_REG_OFFSET: u32 = 0x74;
/// Card reset register offset
pub const FSDIF_CARD_RESET_OFFSET: u32 = 0x78;
/// Bus mode register offset
pub const FSDIF_BUS_MODE_OFFSET: u32 = 0x80;
/// Descriptor list low base address register offset
pub const FSDIF_DESC_LIST_ADDRL_OFFSET: u32 = 0x88;
/// Descriptor list high base address register offset
pub const FSDIF_DESC_LIST_ADDRH_OFFSET: u32 = 0x8C;
/// Internal DMAC status register offset
pub const FSDIF_DMAC_STATUS_OFFSET: u32 = 0x90;
/// Internal DMAC interrupt enable register offset
pub const FSDIF_DMAC_INT_EN_OFFSET: u32 = 0x94;
/// Current host descriptor low address register offset
pub const FSDIF_CUR_DESC_ADDRL_OFFSET: u32 = 0x98;
/// Current host descriptor high address register offset
pub const FSDIF_CUR_DESC_ADDRH_OFFSET: u32 = 0x9C;
/// Current buffer low address register offset
pub const FSDIF_CUR_BUF_ADDRL_OFFSET: u32 = 0xA0;
/// Current buffer high address register offset
pub const FSDIF_CUR_BUF_ADDRH_OFFSET: u32 = 0xA4;
/// Card threshold control register offset
pub const FSDIF_CARD_THRCTL_OFFSET: u32 = 0x100;
/// UHS register extension offset
pub const FSDIF_CLK_SRC_OFFSET: u32 = 0x108;
/// EMMC DDR register offset
pub const FSDIF_EMMC_DDR_REG_OFFSET: u32 = 0x10C;
/// Enable phase shift register offset
pub const FSDIF_ENABLE_SHIFT_OFFSET: u32 = 0x110;
/// Data FIFO access offset
pub const FSDIF_DATA_OFFSET: u32 = 0x200;

/// Timeout for retries in polling loops
pub const RETRIES_TIMEOUT: usize = 50000;
/// Command timeout in milliseconds
pub const COMMAND_TIMEOUT: u32 = 5000;
/// Delay in microseconds
pub const FSDIF_DELAY_US: u32 = 5;
/// Maximum FIFO count for PIO transfers
pub const MCI_MAX_FIFO_CNT: u32 = 0x800;

/// Maximum number of command retries
pub const FSL_SDMMC_MAX_CMD_RETRIES: u32 = 10;

/// FSDIF instance 0 ID
pub const FSDIF0_ID: u32 = 0;
/// FSDIF instance 1 ID
pub const FSDIF1_ID: u32 = 1;

/// Component ready magic value
pub const FT_COMPONENT_IS_READY: u32 = 0x11111111;

// DMA related constants
/// Disable interrupt on completion for this descriptor
pub const FSDIF_IDMAC_DES0_DIC: u32 = 1 << 1;
/// Last descriptor flag
pub const FSDIF_IDMAC_DES0_LD: u32 = 1 << 2;
/// First descriptor flag
pub const FSDIF_IDMAC_DES0_FD: u32 = 1 << 3;
/// Chain to next descriptor flag
pub const FSDIF_IDMAC_DES0_CH: u32 = 1 << 4;
/// End of chain flag
pub const FSDIF_IDMAC_DES0_ER: u32 = 1 << 5;
/// Card error summary in RINTSTS register
pub const FSDIF_IDMAC_DES0_CES: u32 = 1 << 30;
/// Descriptor owned by DMA, cleared to 0 after transfer
pub const FSDIF_IDMAC_DES0_OWN: u32 = 1 << 31;
/// Maximum bytes per descriptor in chained mode
pub const FSDIF_IDMAC_MAX_BUF_SIZE: u32 = 0x1000;

// Interrupt related constants
/// Number of interrupt events
pub const FSDIF_NUM_OF_EVT: usize = 5;
/// A register used by event_handler
pub const TEMP_REGISTER_OFFSET: u32 = 0xFD0;
