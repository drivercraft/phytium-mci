pub struct MCITiming {
    use_hold: bool,
    clk_div: u32,
    clk_src: u32,
    shift: u32,
    #[allow(unused)]
    pad_delay: MCIPadDelay, //* 用于调整IO的延时 */
}

impl Default for MCITiming {
    fn default() -> Self {
        Self::new()
    }
}

impl MCITiming {
    pub fn new() -> Self {
        MCITiming {
            use_hold: false,
            clk_div: 0,
            clk_src: 0,
            shift: 0,
            pad_delay: MCIPadDelay::None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum MCIPadDelay {
    Set,
    Unset,
    None,
}

impl MCITiming {
    pub(crate) fn clk_src(&self) -> u32 {
        self.clk_src
    }

    pub(crate) fn clk_div(&self) -> u32 {
        self.clk_div
    }

    pub(crate) fn use_hold(&self) -> bool {
        self.use_hold
    }

    pub(crate) fn shift(&self) -> u32 {
        self.shift
    }
}

pub const MMC_SD_400K_HZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x7e7dfa,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: MCIPadDelay::Unset,
};

pub const SD_25MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: MCIPadDelay::Unset,
};

pub const SD_50MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

pub const SD_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

pub const MMC_26MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

pub const MMC_52MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x030204,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};

pub const MMC_66MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::None,
};

pub const MMC_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: MCIPadDelay::Set,
};
