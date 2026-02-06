//! # MCI Configuration
//!
//! This module provides configuration structures and methods for the MCI controller.
//! It handles:
//! - Device instance configuration
//! - Transfer mode selection (DMA/PIO)
//! - Timing parameter selection
//! - Card type detection (removable/non-removable)

#[cfg(all(feature = "dma", feature = "pio"))]
compile_error!("can't enable feature dma and pio at the same time!");

use core::ptr::NonNull;

use super::constants::*;
use super::mci_timing::*;
use super::regs::*;

/// MCI controller configuration.
///
/// This structure contains the configuration for an MCI controller instance.
#[derive(Debug, PartialEq, Clone)]
pub struct MCIConfig {
    /// Device instance ID (MCI0/MCI1)
    instance_id: MCIId,
    /// Device register base address
    reg: MCIReg,
    /// Device IRQ number
    irq_num: u32,
    /// Transfer mode (DMA/PIO)
    trans_mode: MCITransMode,
    /// Non-removable media flag (e.g., eMMC)
    non_removable: bool,
}

impl MCIConfig {
    /// Create a new MCI configuration with DMA transfer mode.
    ///
    /// # Features
    ///
    /// This function is only available when the `dma` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the MCI controller registers
    #[cfg(feature = "dma")]
    pub fn new(addr: NonNull<u8>) -> Self {
        Self {
            instance_id: MCIId::MCI1,
            reg: MCIReg::new(addr),
            irq_num: 105,
            trans_mode: MCITransMode::DMA,
            non_removable: false,
        }
    }

    /// Create a new MCI configuration with PIO transfer mode.
    ///
    /// # Features
    ///
    /// This function is only available when the `pio` feature is enabled.
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the MCI controller registers
    #[cfg(feature = "pio")]
    pub fn new(addr: NonNull<u8>) -> Self {
        Self {
            instance_id: MCIId::MCI0,
            reg: MCIReg::new(addr),
            irq_num: 104,
            trans_mode: MCITransMode::PIO,
            non_removable: false,
        }
    }

    /// Get the device instance default configuration.
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the MCI controller registers
    pub fn lookup_config(addr: NonNull<u8>) -> Self {
        Self::new(addr)
    }

    /// Get timing parameters for the specified clock frequency and card type.
    ///
    /// This function returns the appropriate timing configuration based on:
    /// - The target clock frequency
    /// - Whether the media is removable (SD) or non-removable (eMMC)
    ///
    /// # Arguments
    ///
    /// * `clock_freq` - Target clock frequency
    /// * `non_removable` - Whether the media is non-removable (eMMC)
    ///
    /// # Returns
    ///
    /// `Some(MCITiming)` if a valid timing configuration exists, `None` otherwise
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

    /// Create a configuration for controller restart.
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the MCI controller registers
    pub fn restart(addr: NonNull<u8>) -> Self {
        Self::new(addr)
    }

    /// Get a reference to the register accessor.
    pub fn reg(&self) -> &MCIReg {
        &self.reg
    }

    /// Get the transfer mode.
    pub fn trans_mode(&self) -> MCITransMode {
        self.trans_mode
    }

    /// Set the transfer mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - Transfer mode to set (DMA/PIO)
    pub fn trans_mode_set(&mut self, mode: MCITransMode) {
        self.trans_mode = mode;
    }

    /// Check if the media is non-removable (e.g., eMMC).
    pub fn non_removable(&self) -> bool {
        self.non_removable
    }

    /// Get the device instance ID.
    pub fn instance_id(&self) -> MCIId {
        self.instance_id
    }
}
