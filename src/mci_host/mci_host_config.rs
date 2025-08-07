use super::sd::constants::{SD_BLOCK_SIZE, SD_CLOCK_50MHZ, SD_MAX_RW_BLK};
use crate::mci::consts::MCIId;

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct MCIHostConfig {
    pub(crate) host_id: MCIId,             // 主机 ID
    pub(crate) host_type: MCIHostType,     // 主机类型
    pub(crate) card_type: MCIHostCardType, // 卡类型
    pub(crate) enable_irq: bool,           // 是否启用中断
    // pub(crate) enable_dma: bool, // 是否启用 DMA (是否有必要继续存在这个字段？)
    pub(crate) endian_mode: MCIHostEndianMode, // 字节序模式
    pub(crate) max_trans_size: usize,          // 最大传输大小
    pub(crate) def_block_size: usize,          // 默认块大小
    pub(crate) card_clock: u32,                // 卡时钟频率
    pub(crate) is_uhs_card: bool,              // 是否为 UHS 卡
                                               /* for SDIO card, to support card customized interrupt handling */
                                               // TODO：暂时没实现这部分功能
}

#[allow(unused)]
impl MCIHostConfig {
    #[cfg(feature = "pio")]
    pub fn new() -> Self {
        let enable_irq = if cfg!(feature = "irq") {
            true
        } else {
            false // 默认不启用中断
        };

        Self {
            host_id: MCIId::MCI0,
            host_type: MCIHostType::SDIF,
            card_type: MCIHostCardType::MicroSD,
            enable_irq,
            endian_mode: MCIHostEndianMode::Little,
            max_trans_size: SD_MAX_RW_BLK * SD_BLOCK_SIZE,
            def_block_size: SD_BLOCK_SIZE,
            card_clock: SD_CLOCK_50MHZ,
            is_uhs_card: false,
        }
    }

    #[cfg(feature = "dma")]
    pub fn new() -> Self {
        let enable_irq = if cfg!(feature = "irq") {
            true
        } else {
            false // 默认不启用中断
        };

        Self {
            host_id: MCIId::MCI1,
            host_type: MCIHostType::SDIF,
            card_type: MCIHostCardType::MicroSD,
            enable_irq,
            endian_mode: MCIHostEndianMode::Little,
            max_trans_size: SD_MAX_RW_BLK * SD_BLOCK_SIZE,
            def_block_size: SD_BLOCK_SIZE,
            card_clock: SD_CLOCK_50MHZ,
            is_uhs_card: false, // TODO：需要测试能不能支持UHS模式
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostType {
    SDMMC,
    SDIF,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostCardType {
    StandardSD,
    MicroSD,
    EMMC,
    SDIO,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MCIHostEndianMode {
    Big = 0,         /* Big endian mode */
    HalfWordBig = 1, /* Half word big endian mode */
    Little = 2,      /* Little endian mode */
}
