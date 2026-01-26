#[cfg(all(feature = "dma", feature = "pio"))]
compile_error!("can't enable feature dma and pio at the same time!");
#[cfg(all(feature = "irq", feature = "poll"))]
compile_error!("can't enable feature irq and poll at the same time!");

use crate::mci::consts::MCIId;

use super::sd::consts::{SD_BLOCK_SIZE, SD_CLOCK_50MHZ, SD_MAX_RW_BLK};

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct MCIHostConfig {
    pub(crate) host_id: MCIId,                 // Host ID
    pub(crate) host_type: MCIHostType,         // Host type
    pub(crate) card_type: MCIHostCardType,     // Card type
    pub(crate) enable_irq: bool,               // Whether to enable interrupt
    pub(crate) enable_dma: bool,               // Whether to enable DMA
    pub(crate) endian_mode: MCIHostEndianMode, // Endianness mode
    pub(crate) max_trans_size: usize,          // Maximum transfer size
    pub(crate) def_block_size: usize,          // Default block size
    pub(crate) card_clock: u32,                // Card clock frequency
    pub(crate) is_uhs_card: bool,              // Whether it is a UHS card
                                               /* for SDIO card, to support card customized interrupt handling */ // todo Not implemented yet
                                               // todo timeTuner
}

#[allow(unused)]
impl MCIHostConfig {
    /// Creates a new host configuration with default values.
    ///
    /// The default configuration uses:
    /// - DMA transfer mode (if `dma` feature enabled)
    /// - Polling mode (if `poll` feature enabled)
    /// - 50 MHz card clock
    /// - 512 byte block size
    /// - Little endian mode
    pub fn new() -> Self {
        let mut config = Self {
            host_id: MCIId::MCI1,
            host_type: MCIHostType::SDIF,
            card_type: MCIHostCardType::MicroSD,
            enable_irq: false,
            enable_dma: true,
            endian_mode: MCIHostEndianMode::Little,
            max_trans_size: SD_MAX_RW_BLK * SD_BLOCK_SIZE,
            def_block_size: SD_BLOCK_SIZE,
            card_clock: SD_CLOCK_50MHZ,
            is_uhs_card: false,
        };

        if cfg!(feature = "dma") {
            config.host_id = MCIId::MCI1;
            config.enable_dma = true;
        } else if cfg!(feature = "pio") {
            config.host_id = MCIId::MCI0;
            config.enable_dma = false;
        }

        if cfg!(feature = "irq") {
            config.enable_irq = true;
        } else if cfg!(feature = "poll") {
            config.enable_irq = false;
        }

        config
    }
}

#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostType {
    /// Standard SD/MMC host controller
    SDMMC,
    /// Phytium SDIF host controller
    SDIF,
}

#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostCardType {
    /// Standard SD card (full-size)
    StandardSD,
    /// MicroSD card
    MicroSD,
    /// eMMC (embedded MMC)
    EMMC,
    /// SDIO card with I/O functionality
    SDIO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostEndianMode {
    Big = 0,         /* Big endian mode */
    HalfWordBig = 1, /* Half word big endian mode */
    Little = 2,      /* Little endian mode */
}
