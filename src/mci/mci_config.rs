//! Configuration and timing for MCI operations
//!
//! This module provides configuration structures and timing parameters
//! for different SD/MMC operating modes and clock frequencies.

#[cfg(all(feature = "dma", feature = "pio"))]
compile_error!("can't enable feature dma and pio at the same time!");

use core::ptr::NonNull;

use super::consts::*;
use super::mci_timing::*;
use super::regs::*;

/// MCI configuration structure
///
/// Contains all configuration parameters for an MCI instance including
/// transfer mode, base address, and timing settings.
#[derive(Debug, PartialEq, Clone)]
pub struct MCIConfig {
    /// Device instance identifier
    instance_id: MCIId,
    /// Device register base address
    reg: MCIReg,
    /// Device IRQ number
    irq_num: u32,
    /// Transfer mode (PIO or DMA)
    trans_mode: MCITransMode,
    /// Whether media is non-removable (e.g., eMMC)
    non_removable: bool,
}

impl MCIConfig {
    /// Creates a new MCI configuration
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the MCI controller registers
    ///
    /// # Returns
    ///
    /// A new `MCIConfig` instance with default settings.
    /// The transfer mode is automatically selected based on the `dma` feature flag.
    pub fn new(addr: NonNull<u8>) -> Self {
        let mut config = Self {
            instance_id: MCIId::MCI0,
            reg: MCIReg::new(addr),
            irq_num: 72,
            trans_mode: MCITransMode::DMA,
            non_removable: false,
        };

        if cfg!(feature = "pio") {
            config.trans_mode = MCITransMode::PIO;
        }

        config
    }

    fn clear_irq(&self) {
        let raw_ints = self.reg.read_reg::<MCIRawInts>();
        let dmac_status = self.reg.read_reg::<MCIDMACStatus>();
        self.reg.write_reg(raw_ints);
        self.reg.write_reg(dmac_status);
    }

    /// Gets the device instance default configuration
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the MCI controller registers
    pub fn lookup_config(addr: NonNull<u8>) -> Self {
        Self::new(addr)
    }

    /// Gets timing parameters for the specified clock frequency
    ///
    /// # Arguments
    ///
    /// * `clock_freq` - Target clock frequency
    /// * `non_removable` - Whether the media is non-removable (e.g., eMMC)
    ///
    /// # Returns
    ///
    /// Returns `Some(MCITiming)` if a valid timing configuration exists,
    /// or `None` if the frequency is not supported.
    pub fn get_tuning(clock_freq: MCIClkSpeed, non_removable: bool) -> Option<MCITiming> {
        if clock_freq == MCIClkSpeed::ClkSpeed400KHz {
            return Some(MMC_SD_400K_HZ);
        }
        match (non_removable, clock_freq) {
            (true, MCIClkSpeed::ClkSpeed26Mhz) => Some(MMC_26MHZ),
            (true, MCIClkSpeed::ClkSpeed52Mhz) => Some(MMC_52MHZ),
            (true, MCIClkSpeed::ClkSpeed66Mhz) => Some(MMC_66MHZ),
            (true, MCIClkSpeed::ClkSpeed100Mhz) => Some(MMC_100MHZ),
            (false, MCIClkSpeed::ClkSpeed25Mhz) => Some(SD_25MHZ),
            (false, MCIClkSpeed::ClkSpeed50Mhz) => Some(SD_50MHZ),
            (false, MCIClkSpeed::ClkSpeed100Mhz) => Some(SD_100MHZ),
            _ => None,
        }
    }

    /// Creates a configuration for restarting the MCI controller
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the MCI controller registers
    pub fn restart(addr: NonNull<u8>) -> Self {
        Self::new(addr)
    }

    /// Returns a reference to the MCI register accessor
    pub fn reg(&self) -> &MCIReg {
        &self.reg
    }

    /// Returns the transfer mode (DMA or PIO)
    pub fn trans_mode(&self) -> MCITransMode {
        self.trans_mode
    }

    /// Sets the transfer mode
    ///
    /// # Arguments
    ///
    /// * `mode` - Transfer mode to set (DMA or PIO)
    pub fn trans_mode_set(&mut self, mode: MCITransMode) {
        self.trans_mode = mode;
    }

    /// Returns whether the media is non-removable (e.g., eMMC)
    pub fn non_removable(&self) -> bool {
        self.non_removable
    }

    /// Returns the MCI instance identifier
    pub fn instance_id(&self) -> MCIId {
        self.instance_id
    }

    /// Returns the IRQ number for this MCI instance
    pub fn irq_num(&self) -> u32 {
        self.irq_num
    }
}
