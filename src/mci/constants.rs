//! # MCI Controller Constants
//!
//! This module defines constants and enumerations for the MCI controller including:
//! - Register offsets
//! - Command flags
//! - Transfer modes
//! - Interrupt types
//! - Clock speeds
//! - DMA descriptors

use core::arch::asm;

use bitflags::bitflags;

/// MCI controller identifier.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCIId {
    MCI0,
    MCI1,
}

/// FIFO depth configuration.
///
/// The value represents the bit position in the register.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCIFifoDepth {
    Depth8 = 23,
    Depth16 = 24,
    Depth32 = 25,
    Depth64 = 26,
    Depth128 = 27,
}

bitflags! {
    /// Command flags for MCI operations.
    #[derive(Debug, Clone, Copy)]
    pub struct MCICmdFlag: u32 {
        /// Command needs initialization
        const NEED_INIT = 0x1;
        /// Expect response
        const EXP_RESP = 0x2;
        /// Expect long response (128-bit)
        const EXP_LONG_RESP = 0x4;
        /// Response needs CRC check
        const NEED_RESP_CRC = 0x8;
        /// Expect data transfer
        const EXP_DATA = 0x10;
        /// Data write operation
        const WRITE_DATA = 0x20;
        /// Data read operation
        const READ_DATA = 0x40;
        /// Need auto stop command
        const NEED_AUTO_STOP = 0x80;
        /// Application specific command
        const ADTC = 0x100;
        /// Voltage switch command
        const SWITCH_VOLTAGE = 0x200;
        /// Abort command
        const ABORT = 0x400;
        /// Auto CMD12 enabled
        const AUTO_CMD12 = 0x800;
    }
}

/// Transfer mode enumeration.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCITransMode {
    /// DMA transfer mode
    DMA,
    /// PIO transfer mode (via FIFO read/write)
    PIO,
}

/// Interrupt type enumeration.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MCIIntrType {
    /// Controller interrupt status
    GeneralIntr,
    /// DMA interrupt status
    DmaIntr,
}

/// Event type enumeration.
#[derive(Debug, PartialEq)]
pub enum FsDifEvtType {
    /// Card detection event
    CardDetected = 0,
    /// Command transfer complete event
    CmdDone,
    /// Data transfer complete event
    DataDone,
    /// SDIO card custom event
    SdioIrq,
    /// Error occurred during transfer
    ErrOccured,
    /// Number of event types
    NumOfEvt,
}

/// Clock speed enumeration.
///
/// Values are in Hz.
#[derive(Debug, PartialEq)]
pub enum MCIClkSpeed {
    ClkSpeed400KHz = 400_000,
    ClkSpeed25Mhz = 25_000_000,
    /// MMC specific speed
    ClkSpeed26Mhz = 26_000_000,
    ClkSpeed50Mhz = 50_000_000,
    /// MMC specific speed
    ClkSpeed52Mhz = 52_000_000,
    /// MMC specific speed
    ClkSpeed66Mhz = 66_000_000,
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

/// Data Synchronization Barrier.
///
/// # Safety
///
/// This function uses inline assembly and must be called safely within an unsafe context.
/// The caller must ensure that calling this function does not violate memory safety.
#[inline(always)]
pub unsafe fn dsb() {
    unsafe {
        core::arch::asm!("dsb sy");
        core::arch::asm!("isb sy");
    }
}

/// Flushes the data cache for the given address range.
///
/// # Safety
///
/// The caller must ensure that:
/// - The address points to valid memory
/// - The size does not cause the range to overflow
/// - The memory region is appropriate for cache flushing
#[inline(always)]
pub unsafe fn flush(addr: *const u8, size: usize) {
    unsafe {
        let mut addr = addr as usize;
        let end = addr + size;
        while addr < end {
            asm!("dc civac, {0}", in(reg) addr, options(nostack, preserves_flags));
            addr += 64;
        }
        dsb();
    }
}

/// Invalidates the data cache for the given address range.
///
/// # Safety
///
/// The caller must ensure that:
/// - The address points to valid memory
/// - The size does not cause the range to overflow
/// - The memory region is appropriate for cache invalidation
#[inline(always)]
pub unsafe fn invalidate(addr: *const u8, size: usize) {
    unsafe {
        const CACHE_LINE_SIZE: usize = 64;

        let start_addr = (addr as usize) & !(CACHE_LINE_SIZE - 1);
        let end_addr = (addr as usize + size + CACHE_LINE_SIZE - 1) & !(CACHE_LINE_SIZE - 1);

        let mut current_addr = start_addr;
        while current_addr < end_addr {
            asm!("dc ivac, {0}", in(reg) current_addr, options(nostack, preserves_flags));
            current_addr += CACHE_LINE_SIZE;
        }

        asm!("dsb sy");
        asm!("isb");
    }
}

/** @name Register Map
 *
 * Register offsets from the base address of an SD device.
 * @{
 */
/// Controller configuration register
pub const FSDIF_CNTRL_OFFSET: u32 = 0x00;
/// Power enable register
pub const FSDIF_PWREN_OFFSET: u32 = 0x04;
/// Clock divider register
pub const FSDIF_CLKDIV_OFFSET: u32 = 0x08;
/// Clock enable register
pub const FSDIF_CLKENA_OFFSET: u32 = 0x10;
/// Timeout register
pub const FSDIF_TMOUT_OFFSET: u32 = 0x14;
/// Card type register
pub const FSDIF_CTYPE_OFFSET: u32 = 0x18;
/// Block size register
pub const FSDIF_BLK_SIZ_OFFSET: u32 = 0x1C;
/// Byte count register
pub const FSDIF_BYT_CNT_OFFSET: u32 = 0x20;
/// Interrupt mask register
pub const FSDIF_INT_MASK_OFFSET: u32 = 0x24;
/// Command argument register
pub const FSDIF_CMD_ARG_OFFSET: u32 = 0x28;
/// Command register
pub const FSDIF_CMD_OFFSET: u32 = 0x2C;
/// Response register 0
pub const FSDIF_RESP0_OFFSET: u32 = 0x30;
/// Response register 1
pub const FSDIF_RESP1_OFFSET: u32 = 0x34;
/// Response register 2
pub const FSDIF_RESP2_OFFSET: u32 = 0x38;
/// Response register 3
pub const FSDIF_RESP3_OFFSET: u32 = 0x3C;
/// Masked interrupt status register
pub const FSDIF_MASKED_INTS_OFFSET: u32 = 0x40;
/// Raw interrupt status register
pub const FSDIF_RAW_INTS_OFFSET: u32 = 0x44;
/// Status register
pub const FSDIF_STATUS_OFFSET: u32 = 0x48;
/// FIFO threshold watermark register
pub const FSDIF_FIFOTH_OFFSET: u32 = 0x4C;
/// Card detect register
pub const FSDIF_CARD_DETECT_OFFSET: u32 = 0x50;
/// Card write protect register
pub const FSDIF_CARD_WRTPRT_OFFSET: u32 = 0x54;
/// CIU ready status
pub const FSDIF_CKSTS_OFFSET: u32 = 0x58;
/// Transferred CIU card byte count register
pub const FSDIF_TRAN_CARD_CNT_OFFSET: u32 = 0x5C;
/// Transferred host to FIFO byte count register
pub const FSDIF_TRAN_FIFO_CNT_OFFSET: u32 = 0x60;
/// Debounce count register
pub const FSDIF_DEBNCE_OFFSET: u32 = 0x64;
/// User ID register
pub const FSDIF_UID_OFFSET: u32 = 0x68;
/// Controller version ID register
pub const FSDIF_VID_OFFSET: u32 = 0x6C;
/// Hardware configuration register
pub const FSDIF_HWCONF_OFFSET: u32 = 0x70;
/// UHS-I register
pub const FSDIF_UHS_REG_OFFSET: u32 = 0x74;
/// Card reset register
pub const FSDIF_CARD_RESET_OFFSET: u32 = 0x78;
/// Bus mode register
pub const FSDIF_BUS_MODE_OFFSET: u32 = 0x80;
/// Descriptor list low base address register
pub const FSDIF_DESC_LIST_ADDRL_OFFSET: u32 = 0x88;
/// Descriptor list high base address register
pub const FSDIF_DESC_LIST_ADDRH_OFFSET: u32 = 0x8C;
/// Internal DMAC status register
pub const FSDIF_DMAC_STATUS_OFFSET: u32 = 0x90;
/// Internal DMAC interrupt enable register
pub const FSDIF_DMAC_INT_EN_OFFSET: u32 = 0x94;
/// Current host descriptor low address register
pub const FSDIF_CUR_DESC_ADDRL_OFFSET: u32 = 0x98;
/// Current host descriptor high address register
pub const FSDIF_CUR_DESC_ADDRH_OFFSET: u32 = 0x9C;
/// Current buffer low address register
pub const FSDIF_CUR_BUF_ADDRL_OFFSET: u32 = 0xA0;
/// Current buffer high address register
pub const FSDIF_CUR_BUF_ADDRH_OFFSET: u32 = 0xA4;
/// Card threshold control register
pub const FSDIF_CARD_THRCTL_OFFSET: u32 = 0x100;
/// UHS register extension
pub const FSDIF_CLK_SRC_OFFSET: u32 = 0x108;
/// EMMC DDR register
pub const FSDIF_EMMC_DDR_REG_OFFSET: u32 = 0x10C;
/// Enable phase shift register
pub const FSDIF_ENABLE_SHIFT_OFFSET: u32 = 0x110;
/// Data FIFO access
pub const FSDIF_DATA_OFFSET: u32 = 0x200;

/// Timeout for retry operations
pub const RETRIES_TIMEOUT: usize = 50000;
/// Delay in microseconds
pub const FSDIF_DELAY_US: u32 = 5;
/// Maximum FIFO count
pub const MCI_MAX_FIFO_CNT: u32 = 0x800;

/// Maximum command retries
pub const FSL_SDMMC_MAX_CMD_RETRIES: u32 = 10;

/// FSDIF0 instance ID
pub const FSDIF0_ID: u32 = 0;
/// FSDIF1 instance ID
pub const FSDIF1_ID: u32 = 1;

/// Component ready flag
pub const FT_COMPONENT_IS_READY: u32 = 0x11111111;

// DMA-related
/// Internal descriptor doesn't trigger TI/RI interrupt
pub const FSDIF_IDMAC_DES0_DIC: u32 = 1 << 1;
/// Last descriptor of data
pub const FSDIF_IDMAC_DES0_LD: u32 = 1 << 2;
/// First descriptor of data
pub const FSDIF_IDMAC_DES0_FD: u32 = 1 << 3;
/// Chain to next descriptor address
pub const FSDIF_IDMAC_DES0_CH: u32 = 1 << 4;
/// Chain has reached last descriptor
pub const FSDIF_IDMAC_DES0_ER: u32 = 1 << 5;
/// RINTSTS register error summary
pub const FSDIF_IDMAC_DES0_CES: u32 = 1 << 30;
/// Descriptor owned by DMA, cleared to 0 after transfer complete
pub const FSDIF_IDMAC_DES0_OWN: u32 = 1 << 31;
/// Maximum bytes per descriptor in chained mode
pub const FSDIF_IDMAC_MAX_BUF_SIZE: u32 = 0x1000;
