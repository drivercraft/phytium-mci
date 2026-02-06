//! # MCI Host Configuration
//!
//! This module provides configuration structures for the MCI host controller.
//!
//! ## Configuration Options
//!
//! - Host type selection (SDMMC/SDIF)
//! - Card type selection (SD/MMC/SDIO)
//! - DMA/PIO transfer mode
//! - Endianness configuration
//! - Clock frequency settings
//! - UHS card support

#[cfg(all(feature = "dma", feature = "pio"))]
compile_error!("can't enable feature dma and pio at the same time!");

use crate::mci::constants::MCIId;

use super::sd::constants::{SD_BLOCK_SIZE, SD_CLOCK_50MHZ, SD_MAX_RW_BLK};

/// MCI host controller configuration.
#[allow(unused)]
pub struct MCIHostConfig {
    /// Host ID (MCI0/MCI1)
    pub(crate) host_id: MCIId,
    /// Host type (SDMMC/SDIF)
    pub(crate) host_type: MCIHostType,
    /// Card type (SD/MMC/SDIO)
    pub(crate) card_type: MCIHostCardType,
    /// Whether to enable interrupts
    pub(crate) enable_irq: bool,
    /// Whether to enable DMA
    pub(crate) enable_dma: bool,
    /// Endianness mode
    pub(crate) endian_mode: MCIHostEndianMode,
    /// Maximum transfer size
    pub(crate) max_trans_size: usize,
    /// Default block size
    pub(crate) def_block_size: usize,
    /// Card clock frequency
    pub(crate) card_clock: u32,
    /// Whether the card is UHS-compliant
    pub(crate) is_uhs_card: bool,
    /* for SDIO card, to support card customized interrupt handling */
    // TODO This functionality is not implemented yet
}

#[allow(unused)]
impl MCIHostConfig {
    /// Create a new host configuration with DMA mode enabled.
    ///
    /// # Features
    ///
    /// This function is only available when the `dma` feature is enabled.
    #[cfg(feature = "dma")]
    pub fn new() -> Self {
        Self {
            host_id: MCIId::MCI1,
            host_type: MCIHostType::SDIF,
            card_type: MCIHostCardType::MicroSD,
            enable_irq: false, // TODO Will be set to true after IRQ-related features are implemented
            enable_dma: true,
            endian_mode: MCIHostEndianMode::Little,
            max_trans_size: SD_MAX_RW_BLK * SD_BLOCK_SIZE,
            def_block_size: SD_BLOCK_SIZE,
            card_clock: SD_CLOCK_50MHZ,
            is_uhs_card: false, // TODO Need to test whether UHS mode is supported
        }
    }

    /// Create a new host configuration with PIO mode enabled.
    ///
    /// # Features
    ///
    /// This function is only available when the `pio` feature is enabled.
    #[cfg(feature = "pio")]
    pub fn new() -> Self {
        Self {
            host_id: MCIId::MCI0,
            host_type: MCIHostType::SDIF,
            card_type: MCIHostCardType::MicroSD,
            enable_irq: false, // TODO Will be set to true after IRQ-related features are implemented
            enable_dma: false,
            endian_mode: MCIHostEndianMode::Little,
            max_trans_size: SD_MAX_RW_BLK * SD_BLOCK_SIZE,
            def_block_size: SD_BLOCK_SIZE,
            card_clock: SD_CLOCK_50MHZ,
            is_uhs_card: false,
        }
    }
}

/// Host type enumeration.
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostType {
    /// SDMMC host type
    SDMMC,
    /// SDIF host type
    SDIF,
}

/// Card type enumeration.
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostCardType {
    /// Standard SD card
    StandardSD,
    /// Micro SD card
    MicroSD,
    /// eMMC card
    EMMC,
    /// SDIO card
    SDIO,
}

/// Endianness mode enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostEndianMode {
    /// Big endian mode
    Big = 0,
    /// Half-word big endian mode
    HalfWordBig = 1,
    /// Little endian mode
    Little = 2,
}
