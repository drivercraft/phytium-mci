use core::ops;

use crate::mci::{constants::*, err::MCIError};
use bitflags::bitflags;

use super::{FlagReg, Reg};

pub type MCIReg = Reg<MCIError>;

impl Clone for MCIReg {
    fn clone(&self) -> Self {
        Self::new(self.addr)
    }
}

// FSDIF_CNTRL_OFFSET x0 Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCICtrl: u32 {
        const CONTROLLER_RESET = 1 << 0; // RW Reset controller, except DMA, FIFO
        const FIFO_RESET = 1 << 1; // RW Reset FIFO, 1 is valid
        const DMA_RESET = 1 << 2; // RW Reset internal DMA, 1 is valid
        const INT_ENABLE = 1 << 4; // RW Global interrupt enable configuration, 1 to enable
        const DMA_ENABLE = 1 << 5; // RW External DMA mode enable
        const READ_WAIT = 1 << 6; // RW SDIF read wait, 1 is valid
        const SEND_IRQ_RESPONSE = 1 << 7; // RW MMC interrupt auto response configuration, 1 is valid
        const ABORT_READ_DATA = 1 << 8; // RW Read pause exception clear
        const SEND_CCSD = 1 << 9; // RW Send CCD (NOT USED)
        const SEND_AUTO_STOP_CCSD = 1 << 10; // RW Send CCD, auto STOP (NOT USED)
        const ENDIAN = 1 << 11; // RW 0: little endian, 1: big endian
        const CARD_VOLTAGE_A_MASK = 0xf << 16; // RW A voltage selection
        const CARD_VOLTAGE_B_MASK = 0xf << 20; // RW B voltage selection
        const ENABLE_OD_PULLUP = 1 << 24; // RW External open-drain output
        const USE_INTERNAL_DMAC = 1 << 25; // RW Use internal DMA
    }
}

impl FlagReg for MCICtrl {
    const REG: u32 = FSDIF_CNTRL_OFFSET;
}

// FSDIF_PWREN_OFFSET 0x4 Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCIPwrEn: u32 {
        const ENABLE = 1 << 0; // RW Card power switch, 0: off; 1: on
    }
}

impl FlagReg for MCIPwrEn {
    const REG: u32 = FSDIF_PWREN_OFFSET;
}

// FSDIF_CLKDIV_OFFSET 0x8 Register
bitflags! {
    pub struct MCIClkDiv: u32 {
        /* CLK_SAMPLE and CLK_DRV must be less than CLK_DIVIDER */
        const CLK_DIVDER_BIT0 = 1 << 0; /* Clock divider parameter setting, divider parameter = 2*CLK_DIVIDER */
        const CLK_DIVDER_BIT1 = 1 << 1;
        const CLK_DIVDER_BIT2 = 1 << 2;
        const CLK_DIVDER_BIT3 = 1 << 3;
        const CLK_DIVDER_BIT4 = 1 << 4;
        const CLK_DIVDER_BIT5 = 1 << 5;
        const CLK_DIVDER_BIT6 = 1 << 6;
        const CLK_DIVDER_BIT7 = 1 << 7;
        const CLK_DRV_BIT0 = 1 << 8; /* Output phase range setting */
        const CLK_DRV_BIT1 = 1 << 9;
        const CLK_DRV_BIT2 = 1 << 10;
        const CLK_DRV_BIT3 = 1 << 11;
        const CLK_DRV_BIT4 = 1 << 12;
        const CLK_DRV_BIT5 = 1 << 13;
        const CLK_DRV_BIT6 = 1 << 14;
        const CLK_DRV_BIT7 = 1 << 15;
        const CLK_SAMPLE_BIT0 = 1 << 16; /* Sample phase range setting */
        const CLK_SAMPLE_BIT1 = 1 << 17;
        const CLK_SAMPLE_BIT2 = 1 << 18;
        const CLK_SAMPLE_BIT3 = 1 << 19;
        const CLK_SAMPLE_BIT4 = 1 << 20;
        const CLK_SAMPLE_BIT5 = 1 << 21;
        const CLK_SAMPLE_BIT6 = 1 << 22;
        const CLK_SAMPLE_BIT7 = 1 << 23;
    }
}

impl FlagReg for MCIClkDiv {
    const REG: u32 = FSDIF_CLKDIV_OFFSET;
}

impl MCIClkDiv {
    pub fn clk_sample_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 23, 16))
    }
    pub fn clk_drv_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 15, 8))
    }
    pub fn clk_divider_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 7, 0))
    }
    pub fn clk_div(samp: u32, drv: u32, div: u32) -> Self {
        Self::clk_sample_set(samp) | Self::clk_drv_set(drv) | Self::clk_divider_set(div)
    }
    pub fn clk_divider_get(div_reg: u32) -> Self {
        MCIClkDiv::from_bits_truncate(get_reg32_bits!(div_reg, 7, 0))
    }
}

// FSDIF_CLKENA_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCIClkEn: u32 {
        const CCLK_ENABLE = 1 << 0; /* RW 0：Clock disabled；1：Clock enabled */
        const CLKENA_CCLK_LOW_POWER = 1<<16; /* RW 0x0: non-low power; 0x1: low power */
    }
}

impl FlagReg for MCIClkEn {
    const REG: u32 = FSDIF_CLKENA_OFFSET;
}

// FSDIF_TMOUT_OFFSET Register
bitflags! {
    pub struct MCITimeout: u32 {
        const MAX_DATA_TIMEOUT = 0xffffff; /* RW Read card timeout (in card clock units) */
        const MAX_RESP_TIMEOUT = 0xff; /* RW Response timeout (in card clock units) */
        const RESP_TIMEOUT_BIT0 = 1 << 0; /* RW Response timeout bit 0 */
        const RESP_TIMEOUT_BIT1 = 1 << 1; /* RW Response timeout bit 1 */
        const RESP_TIMEOUT_BIT2 = 1 << 2; /* RW Response timeout bit 2 */
        const RESP_TIMEOUT_BIT3 = 1 << 3; /* RW Response timeout bit 3 */
        const RESP_TIMEOUT_BIT4 = 1 << 4; /* RW Response timeout bit 4 */
        const RESP_TIMEOUT_BIT5 = 1 << 5; /* RW Response timeout bit 5 */
        const RESP_TIMEOUT_BIT6 = 1 << 6; /* RW Response timeout bit 6 */
        const RESP_TIMEOUT_BIT7 = 1 << 7; /* RW Response timeout bit 7 */
        const DATA_TIMEOUT_BIT0 = 1 << 8; /* RW Read card timeout bit 0 */
        const DATA_TIMEOUT_BIT1 = 1 << 9; /* RW Read card timeout bit 1 */
        const DATA_TIMEOUT_BIT2 = 1 << 10; /* RW Read card timeout bit 2 */
        const DATA_TIMEOUT_BIT3 = 1 << 11; /* RW Read card timeout bit 3 */
        const DATA_TIMEOUT_BIT4 = 1 << 12; /* RW Read card timeout bit 4 */
        const DATA_TIMEOUT_BIT5 = 1 << 13; /* RW Read card timeout bit 5 */
        const DATA_TIMEOUT_BIT6 = 1 << 14; /* RW Read card timeout bit 6 */
        const DATA_TIMEOUT_BIT7 = 1 << 15; /* RW Read card timeout bit 7 */
        const DATA_TIMEOUT_BIT8 = 1 << 16; /* RW Read card timeout bit 8 */
        const DATA_TIMEOUT_BIT9 = 1 << 17; /* RW Read card timeout bit 9 */
        const DATA_TIMEOUT_BIT10 = 1 << 18; /* RW Read card timeout bit 10 */
        const DATA_TIMEOUT_BIT11 = 1 << 19; /* RW Read card timeout bit 11 */
        const DATA_TIMEOUT_BIT12 = 1 << 20; /* RW Read card timeout bit 12 */
        const DATA_TIMEOUT_BIT13 = 1 << 21; /* RW Read card timeout bit 13 */
        const DATA_TIMEOUT_BIT14 = 1 << 22; /* RW Read card timeout bit 14 */
        const DATA_TIMEOUT_BIT15 = 1 << 23; /* RW Read card timeout bit 15 */
        const DATA_TIMEOUT_BIT16 = 1 << 24; /* RW Read card timeout bit 16 */
        const DATA_TIMEOUT_BIT17 = 1 << 25; /* RW Read card timeout bit 17 */
        const DATA_TIMEOUT_BIT18 = 1 << 26; /* RW Read card timeout bit 18 */
        const DATA_TIMEOUT_BIT19 = 1 << 27; /* RW Read card timeout bit 19 */
        const DATA_TIMEOUT_BIT20 = 1 << 28; /* RW Read card timeout bit 20 */
        const DATA_TIMEOUT_BIT21 = 1 << 29; /* RW Read card timeout bit 21 */
        const DATA_TIMEOUT_BIT22 = 1 << 30; /* RW Read card timeout bit 22 */
        const DATA_TIMEOUT_BIT23 = 1 << 31; /* RW Read card timeout bit 23 */
    }
}

impl FlagReg for MCITimeout {
    const REG: u32 = FSDIF_TMOUT_OFFSET;
}

impl MCITimeout {
    pub fn timeout_data(data_timeout: MCITimeout, resp_timeout: MCITimeout) -> MCITimeout {
        MCITimeout::from_bits_truncate(
            (genmask!(31, 8) & (data_timeout.bits() << 8)) | (genmask!(7, 0) & resp_timeout.bits()),
        )
    }
}

// FSDIF_CTYPE_OFFSET Register
bitflags! {
    pub struct MCICType: u32 {
        const CARD0_WIDTH1_8BIT = 1 << 16; /* 1: 8-bit mode */
        const CARD0_WIDTH2_4BIT = 1 << 0; /* 1: 4-bit mode */
        const CARD0_WIDTH2_1BIT = 0; /* 0: 1-bit mode */
    }
}

impl From<u32> for MCICType {
    fn from(val: u32) -> Self {
        match val {
            1 => Self::CARD0_WIDTH2_1BIT,
            4 => Self::CARD0_WIDTH2_4BIT,
            8 => Self::CARD0_WIDTH1_8BIT,
            _ => panic!("Invalid card width: {}", val),
        }
    }
}

impl FlagReg for MCICType {
    const REG: u32 = FSDIF_CTYPE_OFFSET;
}

// FSDIF_INT_MASK_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCIIntMask: u32 {
        const CD_BIT = 1 << 0;       /* RW Card detect (CD) */
        const RE_BIT = 1 << 1;       /* RW Response error (RE) */
        const CMD_BIT = 1 << 2;      /* RW Command done (CD) */
        const DTO_BIT = 1 << 3;      /* RW Data transfer over (DTO) */
        const TXDR_BIT = 1 << 4;     /* RW Transmit FIFO data request (TXDR) */
        const RXDR_BIT = 1 << 5;     /* RW Receive FIFO data request (RXDR) */
        const RCRC_BIT = 1 << 6;     /* RW Response CRC error (RCRC) */
        const DCRC_BIT = 1 << 7;     /* RW Data CRC error (DCRC) */
        const RTO_BIT = 1 << 8;      /* RW Response timeout (RTO) */
        const DRTO_BIT = 1 << 9;     /* RW Data read timeout (DRTO) */
        const HTO_BIT = 1 << 10;     /* RW Data starvation-by-host timeout (HTO) */
        const FRUN_BIT = 1 << 11;    /* RW FIFO underrun/overrun error (FRUN) */
        const HLE_BIT = 1 << 12;     /* RW Hardware locked write error (HLE) */
        const SBE_BCI_BIT = 1 << 13; /* RW Start-bit error (SBE) */
        const ACD_BIT = 1 << 14;     /* RW Auto command done (ACD) */
        const EBE_BIT = 1 << 15;     /* RW End-bit error (read)/Write no CRC (EBE) */
        const SDIO_BIT = 1 << 16;    /* RW SDIO interrupt for card */
        const ALL_BITS = 0x1FFFF;    /* RW All bits */
        // const INTS_CMD_MASK = 0x1546;
        // const INTS_DATA_MASK = 0x2288;
        // const INTS_DATA_MASK = DTO_BIT | DCRC_BIT | DRTO_BIT | SBE_BCI_BIT;
        const INTS_DATA_MASK = 0x2288;
        // const INTS_CMD_MASK = RE_BIT | CMD_BIT | RCRC_BIT | RTO_BIT | HTO_BIT | HLE_BIT;
        const INTS_CMD_MASK = 0x5446;

    }
}

impl FlagReg for MCIIntMask {
    const REG: u32 = FSDIF_INT_MASK_OFFSET; // Assuming FSDIF_INT_OFFSET is the corresponding register offset
}

// FSDIF_MASKED_INTS_OFFSET Register
bitflags! {
    pub struct MCIMaskedInts: u32 {
        const CD_BIT = 1 << 0;       /* RW Card detect (CD) */
        const RE_BIT = 1 << 1;       /* RW Response error (RE) */
        const CMD_BIT = 1 << 2;      /* RW Command done (CD) */
        const DTO_BIT = 1 << 3;      /* RW Data transfer over (DTO) */
        const TXDR_BIT = 1 << 4;     /* RW Transmit FIFO data request (TXDR) */
        const RXDR_BIT = 1 << 5;     /* RW Receive FIFO data request (RXDR) */
        const RCRC_BIT = 1 << 6;     /* RW Response CRC error (RCRC) */
        const DCRC_BIT = 1 << 7;     /* RW Data CRC error (DCRC) */
        const RTO_BIT = 1 << 8;      /* RW Response timeout (RTO) */
        const DRTO_BIT = 1 << 9;     /* RW Data read timeout (DRTO) */
        const HTO_BIT = 1 << 10;     /* RW Data starvation-by-host timeout (HTO) */
        const FRUN_BIT = 1 << 11;    /* RW FIFO underrun/overrun error (FRUN) */
        const HLE_BIT = 1 << 12;     /* RW Hardware locked write error (HLE) */
        const SBE_BCI_BIT = 1 << 13; /* RW Start-bit error (SBE) */
        const ACD_BIT = 1 << 14;     /* RW Auto command done (ACD) */
        const EBE_BIT = 1 << 15;     /* RW End-bit error (read)/Write no CRC (EBE) */
        const SDIO_BIT = 1 << 16;    /* RW SDIO interrupt for card */
        const ALL_BITS = 0x1FFFF;    /* RW All bits */
        const INTS_CMD_MASK = 0x1546;
        const INTS_DATA_MASK = 0x2288;
    }
}

impl FlagReg for MCIMaskedInts {
    const REG: u32 = FSDIF_MASKED_INTS_OFFSET;
}

// FSDIF_RAW_INTS_OFFSET Register
bitflags! {
    pub struct MCIRawInts: u32 {
        const CD_BIT = 1 << 0;       /* RW Card detect (CD) */
        const RE_BIT = 1 << 1;       /* RW Response error (RE) */
        const CMD_BIT = 1 << 2;      /* RW Command done (CD) */
        const DTO_BIT = 1 << 3;      /* RW Data transfer over (DTO) */
        const TXDR_BIT = 1 << 4;     /* RW Transmit FIFO data request (TXDR) */
        const RXDR_BIT = 1 << 5;     /* RW Receive FIFO data request (RXDR) */
        const RCRC_BIT = 1 << 6;     /* RW Response CRC error (RCRC) */
        const DCRC_BIT = 1 << 7;     /* RW Data CRC error (DCRC) */
        const RTO_BIT = 1 << 8;      /* RW Response timeout (RTO) */
        const DRTO_BIT = 1 << 9;     /* RW Data read timeout (DRTO) */
        const HTO_BIT = 1 << 10;     /* RW Data starvation-by-host timeout (HTO) */
        const FRUN_BIT = 1 << 11;    /* RW FIFO underrun/overrun error (FRUN) */
        const HLE_BIT = 1 << 12;     /* RW Hardware locked write error (HLE) */
        const SBE_BCI_BIT = 1 << 13; /* RW Start-bit error (SBE) */
        const ACD_BIT = 1 << 14;     /* RW Auto command done (ACD) */
        const EBE_BIT = 1 << 15;     /* RW End-bit error (read)/Write no CRC (EBE) */
        const SDIO_BIT = 1 << 16;    /* RW SDIO interrupt for card */
        const ALL_BITS = 0x1FFFF;    /* RW All bits */
        const INTS_CMD_MASK = 0x1546;
        const INTS_DATA_MASK = 0x2288;
    }
}

impl FlagReg for MCIRawInts {
    const REG: u32 = FSDIF_RAW_INTS_OFFSET;
}

// FSDIF_CMD_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCICmd: u32 {
        const START = 1 << 31;                /* Start command */
        const USE_HOLD_REG = 1 << 29;         /* 0: Bypass HOLD register, 1: Enable HOLD register */
        const VOLT_SWITCH = 1 << 28;          /* 0: No voltage switching, 1: Voltage switching */
        const BOOT_MODE = 1 << 27;            /* 0: Mandatory boot, 1: Alternate boot */
        const DISABLE_BOOT = 1 << 26;         /* Abort boot process */
        const EXPECT_BOOT_ACK = 1 << 25;      /* 1: Expect boot ack */
        const ENABLE_BOOT = 1 << 24;          /* 1: Enable boot for mandatory */
        const UPD_CLK = 1 << 21;              /* 1: Do not send command, only update clock register value to card clock domain */
        const INIT = 1 << 15;                  /* 0: Do not send initialization sequence (80 cycles) before sending command, 1: Send */
        const STOP_ABORT = 1 << 14;           /* 1: Stop or abort command, used to stop current data transfer */
        const WAIT_PRVDATA_COMPLETE = 1 << 13; /* 1: Wait for previous data transfer to complete before sending command, 0: Send command immediately */
        const SEND_AUTO_STOP = 1 << 12;       /* 1: Send stop command when data transfer ends */
        const DAT_WRITE = 1 << 10;            /* 0: Read card, 1: Write card */
        const DAT_EXP = 1 << 9;                /* 0: Do not wait for data transfer, 1: Wait for data transfer */
        const RESP_CRC = 1 << 8;               /* 1: Check response CRC */
        const RESP_LONG = 1 << 7;              /* 0: Wait for short response from card, 1: Wait for long response from card */
        const RESP_EXP = 1 << 6;               /* 1: Wait for card response, 0: Command does not require card response */
        const CMD_INDEX_BIT5 = 1 << 5;         /* Command index bit 5 */
        const CMD_INDEX_BIT4 = 1 << 4;         /* Command index bit 4 */
        const CMD_INDEX_BIT3 = 1 << 3;         /* Command index bit 3 */
        const CMD_INDEX_BIT2 = 1 << 2;         /* Command index bit 2 */
        const CMD_INDEX_BIT1 = 1 << 1;         /* Command index bit 1 */
        const CMD_INDEX_BIT0 = 1 << 0;         /* Command index bit 0 */
    }
}

impl FlagReg for MCICmd {
    const REG: u32 = FSDIF_CMD_OFFSET; // Assuming FSDIF_CMD_OFFSET is the corresponding register offset
}

impl MCICmd {
    pub fn index_set(x: u32) -> Self {
        Self::from_bits_truncate(set_reg32_bits!(x, 5, 0))
    }

    pub fn index_get(&self) -> u32 {
        self.bits() & genmask!(5, 0)
    }
}

// FSDIF_STATUS_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCIStatus: u32 {
        const FIFO_RX = 1 << 0;     /* RO, Reach FIFO_RX mark */
        const FIFO_TX = 1 << 1;     /* RO, Reach FIFO_TX mark */
        const FIFO_EMPTY = 1 << 2;  /* RO, FIFO empty */
        const FIFO_FULL = 1 << 3;   /* RO, FIFO full */
        const CMD_FSM_BIT0 = 1 << 4; /* RO CMD FSM state */
        const CMD_FSM_BIT1 = 1 << 5; /* RO CMD FSM state */
        const CMD_FSM_BIT2 = 1 << 6; /* RO CMD FSM state */
        const CMD_FSM_BIT3 = 1 << 7; /* RO CMD FSM state */
        const DATA3_STATUS = 1 << 8; /* RO DATA[3] card present detection, 1: present */
        const DATA_BUSY = 1 << 9;   /* RO 1: Card busy */
        const DATA_STATE_MC_BUSY = 1 << 10;  /* RO DATA TX|RX FSM busy  */
        const RESP_INDEX_BIT0 = 1 << 11; /* RO Response index bit 0 */
        const RESP_INDEX_BIT1 = 1 << 12; /* RO Response index bit 1 */
        const RESP_INDEX_BIT2 = 1 << 13; /* RO Response index bit 2 */
        const RESP_INDEX_BIT3 = 1 << 14; /* RO Response index bit 3 */
        const RESP_INDEX_BIT4 = 1 << 15; /* RO Response index bit 4 */
        const RESP_INDEX_BIT5 = 1 << 16; /* RO Response index bit 5 */
        const FIFO_CNT_BIT0 = 1 << 17;   /* RO FIFO data count bit 0 */
        const FIFO_CNT_BIT1 = 1 << 18;   /* RO FIFO data count bit 1 */
        const FIFO_CNT_BIT2 = 1 << 19;   /* RO FIFO data count bit 2 */
        const FIFO_CNT_BIT3 = 1 << 20;   /* RO FIFO data count bit 3 */
        const FIFO_CNT_BIT4 = 1 << 21;   /* RO FIFO data count bit 4 */
        const FIFO_CNT_BIT5 = 1 << 22;   /* RO FIFO data count bit 5 */
        const FIFO_CNT_BIT6 = 1 << 23;   /* RO FIFO data count bit 6 */
        const FIFO_CNT_BIT7 = 1 << 24;   /* RO FIFO data count bit 7 */
        const FIFO_CNT_BIT8 = 1 << 25;   /* RO FIFO data count bit 8 */
        const FIFO_CNT_BIT9 = 1 << 26;   /* RO FIFO data count bit 9 */
        const FIFO_CNT_BIT10 = 1 << 27;  /* RO FIFO data count bit 10 */
        const FIFO_CNT_BIT11 = 1 << 28;  /* RO FIFO data count bit 11 */
        const FIFO_CNT_BIT12 = 1 << 29;  /* RO FIFO data count bit 12 */
        const DMA_ACK = 1 << 30;    /* RO DMA acknowledge */
        const DMA_REQ = 1 << 31;    /* RO DMA request */
    }
}

impl FlagReg for MCIStatus {
    const REG: u32 = FSDIF_STATUS_OFFSET;
}

// FSDIF_FIFOTH_OFFSET Register
bitflags! {
    pub struct MCIFifoTh: u32 {
        const DMA_TRANS_MASK = genmask!(30, 28); /* Burst size for multiple transfers */
        const RX_WMARK_MASK = genmask!(27, 16);  /* FIFO threshold when receiving data to card */
        const TX_WMARK_MASK = genmask!(11, 0);   /* FIFO threshold when transmitting data to card */
        const TX_WMARK_BIT0 = 1 << 0;            /* TX_WMARK bit 0 */
        const TX_WMARK_BIT1 = 1 << 1;            /* TX_WMARK bit 1 */
        const TX_WMARK_BIT2 = 1 << 2;            /* TX_WMARK bit 2 */
        const TX_WMARK_BIT3 = 1 << 3;            /* TX_WMARK bit 3 */
        const TX_WMARK_BIT4 = 1 << 4;            /* TX_WMARK bit 4 */
        const TX_WMARK_BIT5 = 1 << 5;            /* TX_WMARK bit 5 */
        const TX_WMARK_BIT6 = 1 << 6;            /* TX_WMARK bit 6 */
        const TX_WMARK_BIT7 = 1 << 7;            /* TX_WMARK bit 7 */
        const TX_WMARK_BIT8 = 1 << 8;            /* TX_WMARK bit 8 */
        const TX_WMARK_BIT9 = 1 << 9;            /* TX_WMARK bit 9 */
        const TX_WMARK_BIT10 = 1 << 10;          /* TX_WMARK bit 10 */
        const TX_WMARK_BIT11 = 1 << 11;          /* TX_WMARK bit 11 */
        const RX_WMARK_BIT0 = 1 << 16;           /* RX_WMARK bit 0 */
        const RX_WMARK_BIT1 = 1 << 17;           /* RX_WMARK bit 1 */
        const RX_WMARK_BIT2 = 1 << 18;           /* RX_WMARK bit 2 */
        const RX_WMARK_BIT3 = 1 << 19;           /* RX_WMARK bit 3 */
        const RX_WMARK_BIT4 = 1 << 20;           /* RX_WMARK bit 4 */
        const RX_WMARK_BIT5 = 1 << 21;           /* RX_WMARK bit 5 */
        const RX_WMARK_BIT6 = 1 << 22;           /* RX_WMARK bit 6 */
        const RX_WMARK_BIT7 = 1 << 23;           /* RX_WMARK bit 7 */
        const RX_WMARK_BIT8 = 1 << 24;           /* RX_WMARK bit 8 */
        const RX_WMARK_BIT9 = 1 << 25;           /* RX_WMARK bit 9 */
        const RX_WMARK_BIT10 = 1 << 26;          /* RX_WMARK bit 10 */
        const RX_WMARK_BIT11 = 1 << 27;          /* RX_WMARK bit 11 */
        const DMA_TRANS_BIT0 = 1 << 28;          /* DMA */
        const DMA_TRANS_BIT1 = 1 << 29;          /* DMA */
        const DMA_TRANS_BIT2 = 1 << 30;          /* DMA */
    }
}

impl MCIFifoTh {
    pub const RX_WMARK: u32 = 0x7;
    pub const TX_WMARK: u32 = 0x100;
    /*
    trans_size: Burst size of multiple transaction;
    rx_wmark: FIFO threshold watermark level when receiving data to card.
    tx_wmark: FIFO threshold watermark level when transmitting data to card
    */
    pub fn fifoth(trans_size: MCIFifoThDMATransSize, rx_wmark: u32, tx_wmark: u32) -> Self {
        let trans_size: u32 = trans_size.into();
        (MCIFifoTh::DMA_TRANS_MASK & (trans_size << 28))
            | (MCIFifoTh::RX_WMARK_MASK & (rx_wmark << 16))
            | (MCIFifoTh::TX_WMARK_MASK & tx_wmark)
    }
}

BitsOpsForU32!(MCIFifoTh);

impl FlagReg for MCIFifoTh {
    const REG: u32 = FSDIF_FIFOTH_OFFSET;
}

pub enum MCIFifoThDMATransSize {
    DMATrans1 = 0b000,
    DMATrans4 = 0b001,
    DMATrans8 = 0b010,
    DMATrans16 = 0b011,
    DMATrans32 = 0b100,
    DMATrans64 = 0b101,
    DMATrans128 = 0b110,
    DMATrans256 = 0b111,
}

impl From<MCIFifoThDMATransSize> for u32 {
    fn from(val: MCIFifoThDMATransSize) -> Self {
        val as u32
    }
}

// FSDIF_CARD_DETECT_OFFSET Register
bitflags! {
    pub struct MCICardDetect: u32 {
        const DETECTED = 1 << 0; /* 1: Card not present; 0: Card present */
    }
}

impl FlagReg for MCICardDetect {
    const REG: u32 = FSDIF_CARD_DETECT_OFFSET; // Assuming FSDIF_CARD_DETECT_OFFSET is the corresponding register offset
}

// FSDIF_CARD_WRTPRT_OFFSET Register
bitflags! {
    pub struct MCICardWrtp: u32 {
        const WRITE_PROTECTED = 1 << 0; /* 1: Write protected; 0: No write protection */
    }
}

impl FlagReg for MCICardWrtp {
    const REG: u32 = FSDIF_CARD_WRTPRT_OFFSET; // Assuming FSDIF_CARD_WRTPRT_OFFSET is the corresponding register offset
}

// FSDIF_CKSTS_OFFSET Register
bitflags! {
    pub struct MCIClkSts: u32 {
        const READY = 1 << 0; /* CIU clock ready */
    }
}

impl FlagReg for MCIClkSts {
    const REG: u32 = FSDIF_CKSTS_OFFSET; // Assuming FSDIF_CKSTS_OFFSET is the corresponding register offset
}

// FSDIF_UHS_REG_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCIUhsReg: u32 {
        const VOLT_180 = 1 << 0; /* RW External voltage regulator interface voltage 0: 3.3v, 1: 1.8v */
        const DDR = 1 << 16;     /* RW DDR mode */
    }
}

impl FlagReg for MCIUhsReg {
    const REG: u32 = FSDIF_UHS_REG_OFFSET; // Assuming FSDIF_UHS_REG_OFFSET is the corresponding register offset
}

// FSDIF_CARD_RESET_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCICardReset: u32 {
        const ENABLE = 1 << 0; /* RW 1: Running; 0: Reset */
    }
}

impl FlagReg for MCICardReset {
    const REG: u32 = FSDIF_CARD_RESET_OFFSET; // Assuming FSDIF_CARD_RESET_OFFSET is the corresponding register offset
}

// FSDIF_BUS_MODE_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCIBusMode: u32 {
        const SWR = 1 << 0; /* RW Soft reset, reset idma internal registers */
        const FB = 1 << 1;  /* RW Fixed burst */
        const DE = 1 << 7;  /* RW idma enable */
        const PBL_BIT0 = 1 << 8; /* R0 Transfer burst length */
        const PBL_BIT1 = 1 << 9; /* R0 Transfer burst length */
        const PBL_BIT2 = 1 << 10; /* R0 Transfer burst length */
    }
}

impl FlagReg for MCIBusMode {
    const REG: u32 = FSDIF_BUS_MODE_OFFSET; // Assuming FSDIF_BUS_MODE_OFFSET is the corresponding register offset
}

// FSDIF_DMAC_STATUS_OFFSET Register
bitflags! {
    pub struct MCIDMACStatus: u32 {
        const TI = 1 << 0;  /* RW Transmit interrupt. Indicates linked list data transmission complete */
        const RI = 1 << 1;  /* RW Receive interrupt. Indicates linked list data reception complete */
        const FBE = 1 << 2; /* RW Fatal bus error interrupt */
        const DU_BIT0 = 1 << 3;  /* RW Descriptor unavailable interrupt */
        const DU_BIT1 = 1 << 4;  /* RW Descriptor unavailable interrupt */
        const CES = 1 << 5; /* RW Card error summary */
        const NIS = 1 << 8; /* RW Normal interrupt summary */
        const AIS = 1 << 9; /* RW Abnormal interrupt summary */
        const EB_BIT0 = 1 << 10;
        const EB_BIT1 = 1 << 11;
        const EB_BIT2 = 1 << 12;
        const FSM_BIT0 = 1 << 13;
        const FSM_BIT1 = 1 << 14;
        const FSM_BIT2 = 1 << 15;
        const FSM_BIT3 = 1 << 16;
        const FSM_BIT4 = 1 << 17;
        const FSM_BIT5 = 1 << 18;
        const FSM_BIT6 = 1 << 19;
        const FSM_BIT7 = 1 << 20;
        const FSM_BIT8 = 1 << 21;
        const FSM_BIT9 = 1 << 22;
        const FSM_BIT10 = 1 << 23;
        const FSM_BIT11 = 1 << 24;
        const FSM_BIT12 = 1 << 25;
        const FSM_BIT13 = 1 << 26;
        const FSM_BIT14 = 1 << 27;
        const FSM_BIT15 = 1 << 28;
        const FSM_BIT16 = 1 << 29;
        const FSM_BIT17 = 1 << 30;
        const FSM_BIT18 = 1 << 31;
        const ALL_BITS = 0x3ff;
        const STATUS_EB_TX = 0b001;
        const STATUS_EB_RX = 0b010;
    }
}

impl FlagReg for MCIDMACStatus {
    const REG: u32 = FSDIF_DMAC_STATUS_OFFSET; // Assuming FSDIF_DMAC_STATUS_OFFSET is the corresponding register offset
}

// FSDIF_DMAC_INT_EN_OFFSET Register
bitflags! {
    pub struct MCIDMACIntEn: u32 {
        const TI = 1 << 0;  /* RW Transmit complete interrupt enable */
        const RI = 1 << 1;  /* RW Receive complete interrupt enable */
        const FBE = 1 << 2; /* RW Bus error interrupt enable */
        const DU = 1 << 4;  /* RW Descriptor unavailable interrupt enable */
        const CES = 1 << 5; /* RW Card error interrupt enable */
        const NIS = 1 << 8; /* RW Normal interrupt summary enable */
        const AIS = 1 << 9; /* RW Abnormal interrupt summary enable */
        const ALL_BITS = 0x3ff;
        const INTS_MASK = MCIDMACIntEn::FBE.bits() | MCIDMACIntEn::DU.bits() | MCIDMACIntEn::NIS.bits() | MCIDMACIntEn::AIS.bits();
    }
}

impl FlagReg for MCIDMACIntEn {
    const REG: u32 = FSDIF_DMAC_INT_EN_OFFSET; // Assuming FSDIF_DMAC_INT_EN_OFFSET is the corresponding register offset
}

// FSDIF_CARD_THRCTL_OFFSET Register
bitflags! {
    pub struct MCICardThrctl: u32 {
        const CARDRD = 1 << 0;   /* RW Read card threshold enable */
        const BUSY_CLR = 1 << 1; /* RW busy clear interrupt */
        const CARDWR = 1 << 2;   /* RO Write card threshold enable */
        const FIFO_DEPTH_BIT0 = 1 << 16; /* RW FIFO depth */
        const FIFO_DEPTH_BIT1 = 1 << 17; /* RW FIFO depth */
        const FIFO_DEPTH_BIT2 = 1 << 18; /* RW FIFO depth */
        const FIFO_DEPTH_BIT3 = 1 << 19; /* RW FIFO depth */
        const FIFO_DEPTH_BIT4 = 1 << 20; /* RW FIFO depth */
        const FIFO_DEPTH_BIT5 = 1 << 21; /* RW FIFO depth */
        const FIFO_DEPTH_BIT6 = 1 << 22; /* RW FIFO depth */
        const FIFO_DEPTH_BIT7 = 1 << 23; /* RW FIFO depth */
        const FIFO_DEPTH_BIT8 = 1 << 24; /* RW FIFO depth */
        const FIFO_DEPTH_BIT9 = 1 << 25; /* RW FIFO depth */
        const FIFO_DEPTH_BIT10 = 1 << 26; /* RW FIFO depth */
        const FIFO_DEPTH_BIT11 = 1 << 27; /* RW FIFO depth */
        const FIFO_DEPTH_BIT12 = 1 << 28; /* RW FIFO depth */
    }
}

impl FlagReg for MCICardThrctl {
    const REG: u32 = FSDIF_CARD_THRCTL_OFFSET; // Assuming FSDIF_CARD_THRCTL_OFFSET is the corresponding register offset
}

impl From<MCIFifoDepth> for MCICardThrctl {
    fn from(value: MCIFifoDepth) -> Self {
        let value: u32 = value as u32;
        let value: u32 = 1 << value;
        MCICardThrctl::from_bits_truncate(value)
    }
}

// FSDIF_CLK_SRC_OFFSET Register
bitflags! {
    #[derive(Clone, Copy)]
    pub struct MCIClkSrc: u32 {
        const UHS_EXT_MMC_VOLT = 1 << 0;         /* RW 1.2V power supply selection */
        const UHS_EXT_CLK_ENA = 1 << 1;          /* RW External clock, CIU clock enable */
        const UHS_EXT_CLK_MUX = 1 << 31;         /* RW External clock selection */
        const UHS_CLK_DIV_MASK = genmask!(14, 8); /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_DIV_BIT0 = 1 << 8;         /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_DIV_BIT1 = 1 << 9;         /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_DIV_BIT2 = 1 << 10;        /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_DIV_BIT3 = 1 << 11;        /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_DIV_BIT4 = 1 << 12;        /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_DIV_BIT5 = 1 << 13;        /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_DIV_BIT6 = 1 << 14;        /* RW Division coefficient, ciu_f = clk_div_ctrl + 1, min=1*/
        const UHS_CLK_SAMP_MASK = genmask!(22, 16); /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_SAMP_BIT0 = 1 << 16;         /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_SAMP_BIT1 = 1 << 17;         /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_SAMP_BIT2 = 1 << 18;         /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_SAMP_BIT3 = 1 << 19;         /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_SAMP_BIT4 = 1 << 20;         /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_SAMP_BIT5 = 1 << 21;         /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_SAMP_BIT6 = 1 << 22;         /* RW Sample phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_MASK = genmask!(30, 24); /* RW Output phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_BIT0 = 1 << 24;         /* RW Output phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_BIT1 = 1 << 25;         /* RW Output phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_BIT2 = 1 << 26;         /* RW Output phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_BIT3 = 1 << 27;         /* RW Output phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_BIT4 = 1 << 28;         /* RW Output phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_BIT5 = 1 << 29;         /* RW Output phase parameter, relative to ciu clock phase point */
        const UHS_CLK_DRV_BIT6 = 1 << 30;         /* RW Output phase parameter, relative to ciu clock phase point */
    }
}

impl FlagReg for MCIClkSrc {
    const REG: u32 = FSDIF_CLK_SRC_OFFSET; // Assuming FSDIF_CLK_SRC_OFFSET is the corresponding register offset
}

impl MCIClkSrc {
    pub fn uhs_clk_div(x: u32) -> Self {
        Self::UHS_CLK_DIV_MASK & Self::from_bits_truncate(x << 8)
    }

    pub fn uhs_clk_samp(x: u32) -> Self {
        Self::UHS_CLK_SAMP_MASK & Self::from_bits_truncate(x << 16)
    }

    pub fn uhs_clk_drv(x: u32) -> Self {
        Self::UHS_CLK_DRV_MASK & Self::from_bits_truncate(x << 24)
    }

    pub fn uhs_reg(drv_phase: u32, samp_phase: u32, clk_div: u32) -> Self {
        Self::uhs_clk_div(clk_div) | Self::uhs_clk_samp(samp_phase) | Self::uhs_clk_drv(drv_phase)
    }
}

// FSDIF_EMMC_DDR_REG_OFFSET Register
bitflags! {
    pub struct MCIEmmcDdrReg: u32 {
        const CYCLE = 1 << 0; /* RW 1: start bit less than one cycle, 0: start bit is one cycle */
    }
}

impl FlagReg for MCIEmmcDdrReg {
    const REG: u32 = FSDIF_EMMC_DDR_REG_OFFSET; // Assuming FSDIF_EMMC_DDR_REG_OFFSET is the corresponding register offset
}

// FSDIF_DESC_LIST_ADDRH_OFFSET Register
bitflags! {
    pub struct MCIDescListAddrH: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}

impl FlagReg for MCIDescListAddrH {
    const REG: u32 = FSDIF_DESC_LIST_ADDRH_OFFSET; // Assuming FSDIF_DESC_LIST_ADDRH_OFFSET is the corresponding register offset
}

// FSDIF_DESC_LIST_ADDRL_OFFSET Register
bitflags! {
    pub struct MCIDescListAddrL: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}

impl FlagReg for MCIDescListAddrL {
    const REG: u32 = FSDIF_DESC_LIST_ADDRL_OFFSET; // Assuming FSDIF_DESC_LIST_ADDRL_OFFSET is the corresponding register offset
}

// FSDIF_DATA_OFFSET Register
bitflags! {
    pub struct MCIDataReg: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIDataReg {
    const REG: u32 = FSDIF_DATA_OFFSET; // Assuming FSDIF_DATA_OFFSET is the corresponding register offset
}

// FSDIF_BYT_CNT_OFFSET Register
bitflags! {
    pub struct MCIBytCnt: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIBytCnt {
    const REG: u32 = FSDIF_BYT_CNT_OFFSET; // Assuming FSDIF_BYT_CNT_OFFSET is the corresponding register offset
}

// FSDIF_BLK_SIZ_OFFSET Register
bitflags! {
    pub struct MCIBlkSiz: u32 {
        const BIT0 = 1 << 0; /* RW Block size bit 0 */
        const BIT1 = 1 << 1; /* RW Block size bit 1 */
        const BIT2 = 1 << 2; /* RW Block size bit 2 */
        const BIT3 = 1 << 3; /* RW Block size bit 3 */
        const BIT4 = 1 << 4; /* RW Block size bit 4 */
        const BIT5 = 1 << 5; /* RW Block size bit 5 */
        const BIT6 = 1 << 6; /* RW Block size bit 6 */
        const BIT7 = 1 << 7; /* RW Block size bit 7 */
        const BIT8 = 1 << 8; /* RW Block size bit 8 */
        const BIT9 = 1 << 9; /* RW Block size bit 9 */
        const BIT10 = 1 << 10; /* RW Block size bit 10 */
        const BIT11 = 1 << 11; /* RW Block size bit 11 */
        const BIT12 = 1 << 12; /* RW Block size bit 12 */
        const BIT13 = 1 << 13; /* RW Block size bit 13 */
        const BIT14 = 1 << 14; /* RW Block size bit 14 */
        const BIT15 = 1 << 15; /* RW Block size bit 15 */
        const BIT16 = 1 << 16; /* RW Block size bit 16 */
        const BIT17 = 1 << 17; /* RW Block size bit 17 */
        const BIT18 = 1 << 18; /* RW Block size bit 18 */
        const BIT19 = 1 << 19; /* RW Block size bit 19 */
        const BIT20 = 1 << 20; /* RW Block size bit 20 */
        const BIT21 = 1 << 21; /* RW Block size bit 21 */
        const BIT22 = 1 << 22; /* RW Block size bit 22 */
        const BIT23 = 1 << 23; /* RW Block size bit 23 */
        const BIT24 = 1 << 24; /* RW Block size bit 24 */
        const BIT25 = 1 << 25; /* RW Block size bit 25 */
        const BIT26 = 1 << 26; /* RW Block size bit 26 */
        const BIT27 = 1 << 27; /* RW Block size bit 27 */
        const BIT28 = 1 << 28; /* RW Block size bit 28 */
        const BIT29 = 1 << 29; /* RW Block size bit 29 */
        const BIT30 = 1 << 30; /* RW Block size bit 30 */
        const BIT31 = 1 << 31; /* RW Block size bit 31 */
        const ALL_BITS = 0xFFFFFFFF;
    }
}
impl FlagReg for MCIBlkSiz {
    const REG: u32 = FSDIF_BLK_SIZ_OFFSET; // Assuming FSDIF_BLK_SIZ_OFFSET is the corresponding register offset
}

// FSDIF_TRAN_CARD_CNT_OFFSET Register
bitflags! {
    pub struct MCITranCardCnt:u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCITranCardCnt {
    const REG: u32 = FSDIF_TRAN_CARD_CNT_OFFSET;
}

// FSDIF_TRAN_FIFO_CNT_OFFSET Register
bitflags! {
    pub struct MCITranFifoCnt:u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCITranFifoCnt {
    const REG: u32 = FSDIF_TRAN_FIFO_CNT_OFFSET;
}

// FSDIF_RESP0_OFFSET Register
bitflags! {
    pub struct MCIResp0:u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIResp0 {
    const REG: u32 = FSDIF_RESP0_OFFSET;
}

// FSDIF_RESP1_OFFSET Register
bitflags! {
    pub struct MCIResp1:u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIResp1 {
    const REG: u32 = FSDIF_RESP1_OFFSET;
}

// FSDIF_RESP2_OFFSET Register
bitflags! {
    pub struct MCIResp2:u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIResp2 {
    const REG: u32 = FSDIF_RESP2_OFFSET;
}

// FSDIF_RESP3_OFFSET Register
bitflags! {
    pub struct MCIResp3:u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIResp3 {
    const REG: u32 = FSDIF_RESP3_OFFSET;
}

// FSDIF_CMD_ARG_OFFSET Register
bitflags! {
    pub struct MCICmdArg: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}

impl FlagReg for MCICmdArg {
    const REG: u32 = FSDIF_CMD_ARG_OFFSET;
}

// FSDIF_DEBNCE_OFFSET Register
bitflags! {
    pub struct MCIDebnce: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIDebnce {
    const REG: u32 = FSDIF_DEBNCE_OFFSET;
}

// FSDIF_UID_OFFSET Register
bitflags! {
    pub struct MCIUid: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIUid {
    const REG: u32 = FSDIF_UID_OFFSET;
}

// FSDIF_VID_OFFSET Register
bitflags! {
    pub struct MCIVid: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIVid {
    const REG: u32 = FSDIF_VID_OFFSET;
}

// FSDIF_HWCONF_OFFSET Register
bitflags! {
    pub struct MCIHwconf: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIHwconf {
    const REG: u32 = FSDIF_HWCONF_OFFSET;
}
// FSDIF_CUR_DESC_ADDRL_OFFSET Register
bitflags! {
    pub struct MCICurDescAddrL: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCICurDescAddrL {
    const REG: u32 = FSDIF_CUR_DESC_ADDRL_OFFSET;
}
// FSDIF_CUR_DESC_ADDRH_OFFSET Register
bitflags! {
    pub struct MCIDescAddrH: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIDescAddrH {
    const REG: u32 = FSDIF_CUR_DESC_ADDRH_OFFSET;
}
// FSDIF_CUR_BUF_ADDRL_OFFSET Register
bitflags! {
    pub struct MCICurBufAddrL: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCICurBufAddrL {
    const REG: u32 = FSDIF_CUR_BUF_ADDRL_OFFSET;
}
// FSDIF_CUR_BUF_ADDRH_OFFSET Register
bitflags! {
    pub struct MCIBufAddrH: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIBufAddrH {
    const REG: u32 = FSDIF_CUR_BUF_ADDRH_OFFSET;
}
// FSDIF_ENABLE_SHIFT_OFFSET Register
bitflags! {
    pub struct MCIEnableShift: u32 {
        const BIT0 = 1 << 0;
        const BIT1 = 1 << 1;
        const BIT2 = 1 << 2;
        const BIT3 = 1 << 3;
        const BIT4 = 1 << 4;
        const BIT5 = 1 << 5;
        const BIT6 = 1 << 6;
        const BIT7 = 1 << 7;
        const BIT8 = 1 << 8;
        const BIT9 = 1 << 9;
        const BIT10 = 1 << 10;
        const BIT11 = 1 << 11;
        const BIT12 = 1 << 12;
        const BIT13 = 1 << 13;
        const BIT14 = 1 << 14;
        const BIT15 = 1 << 15;
        const BIT16 = 1 << 16;
        const BIT17 = 1 << 17;
        const BIT18 = 1 << 18;
        const BIT19 = 1 << 19;
        const BIT20 = 1 << 20;
        const BIT21 = 1 << 21;
        const BIT22 = 1 << 22;
        const BIT23 = 1 << 23;
        const BIT24 = 1 << 24;
        const BIT25 = 1 << 25;
        const BIT26 = 1 << 26;
        const BIT27 = 1 << 27;
        const BIT28 = 1 << 28;
        const BIT29 = 1 << 29;
        const BIT30 = 1 << 30;
        const BIT31 = 1 << 31;
    }
}
impl FlagReg for MCIEnableShift {
    const REG: u32 = FSDIF_ENABLE_SHIFT_OFFSET;
}
