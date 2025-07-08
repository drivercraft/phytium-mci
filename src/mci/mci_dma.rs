use alloc::vec::Vec;
use dma_api::DVec;
use dma_api::Direction;
use log::*;

use crate::mci::MCICommand;

use super::MCI;
use super::constants::*;
use super::err::*;
use super::mci_data::MCIData;
use super::regs::*;

#[derive(Default, Clone, Copy, Debug)]
pub struct FSdifIDmaDesc {
    pub attribute: u32,
    pub non1: u32,
    pub len: u32,
    pub non2: u32,
    pub cur_addr_lo: u32,
    pub cur_addr_hi: u32,
    pub cur_desc_lo: u32,
    pub cur_desc_hi: u32,
}

impl FSdifIDmaDesc {
    pub fn new() -> Self {
        FSdifIDmaDesc::default()
    }
}

pub struct FSdifIDmaDescList {
    pub descriptor: Option<DVec<FSdifIDmaDesc>>,
    pub desc_num: u32,
    pub desc_trans_sz: u32,
}

impl FSdifIDmaDescList {
    pub fn new() -> Self {
        FSdifIDmaDescList {
            descriptor: None,
            desc_num: 0,
            desc_trans_sz: FSDIF_IDMAC_MAX_BUF_SIZE,
        }
    }

    pub fn desc_num(&self) -> u32 {
        self.desc_num
    }

    /// get the physical address of the first descriptor.
    pub fn first_desc_dma(&self) -> usize {
        self.descriptor
            .as_ref()
            .map(|dvec| dvec.bus_addr() as usize)
            .unwrap_or(0)
    }

    /// get the number of descriptors
    pub fn len(&self) -> usize {
        self.descriptor.as_ref().map(|dvec| dvec.len()).unwrap_or(0)
    }

    /// Check if the descriptor list is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a mutable reference to the descriptor at the specified index
    pub fn descriptor_mut(&mut self, index: usize) -> Option<FSdifIDmaDesc> {
        if let Some(dvec) = &self.descriptor {
            dvec.get(index)
        } else {
            None
        }
    }

    /// Get the descriptor at the specified index
    pub fn descriptor(&self, index: usize) -> Option<FSdifIDmaDesc> {
        if let Some(dvec) = &self.descriptor {
            dvec.get(index)
        } else {
            None
        }
    }

    /// Set the descriptor at the specified index
    pub fn set_descriptor(&mut self, index: usize, desc: FSdifIDmaDesc) -> MCIResult {
        if let Some(dvec) = &mut self.descriptor {
            if index < dvec.len() {
                dvec.set(index, desc);
                Ok(())
            } else {
                Err(MCIError::InvalidParam)
            }
        } else {
            Err(MCIError::NotInit)
        }
    }

    /// Ensure that the descriptor list has enough capacity for the required number of descriptors
    pub fn ensure_capacity(&mut self, required_desc: u32) -> MCIResult {
        let required_len = required_desc as usize;

        match &self.descriptor {
            None => {
                // Allocate a new DVec with the required number of descriptors
                debug!("Creating new DVec with {required_desc} descriptors");

                let dvec = DVec::zeros(
                    required_len,
                    core::mem::align_of::<FSdifIDmaDesc>(),
                    Direction::Bidirectional,
                )
                .ok_or(MCIError::BadMalloc)?;

                self.descriptor = Some(dvec);
                self.desc_num = required_desc;

                debug!("DVec created at phys_addr: 0x{:x}", self.first_desc_dma());
            }
            Some(dvec) => {
                if dvec.len() < required_len {
                    debug!(
                        "Expanding DVec from {} to {} descriptors",
                        dvec.len(),
                        required_desc
                    );

                    let new_dvec = DVec::zeros(
                        required_len,
                        core::mem::align_of::<FSdifIDmaDesc>(),
                        Direction::Bidirectional,
                    )
                    .ok_or(MCIError::BadMalloc)?;

                    self.descriptor = Some(new_dvec);
                    self.desc_num = required_desc;

                    debug!(
                        "DVec reallocated at phys_addr: 0x{:x}",
                        self.first_desc_dma()
                    );
                }
            }
        }

        Ok(())
    }

    /// Clear all descriptors in the list
    pub fn clear_descriptors(&mut self) {
        if let Some(dvec) = &mut self.descriptor {
            for i in 0..dvec.len() {
                dvec.set(i, FSdifIDmaDesc::default());
            }
        }
    }
}

/* DMA 相关的函数 */
impl MCI {
    pub fn dma_int_set(&mut self) {
        self.config
            .reg()
            .modify_reg(|reg| MCIDMACIntEn::RI | MCIDMACIntEn::TI | MCIDMACIntEn::FBE | reg);
    }

    pub fn dump_dma_descriptor(&self, desc_in_use: u32) {
        debug!("{desc_in_use} dma desc in use!");
        debug!(
            "Descriptor array physical address: 0x{:x}",
            self.desc_list.first_desc_dma()
        );

        if !self.desc_list.is_empty() {
            let max_dump = desc_in_use.min(self.desc_list.len() as u32);
            for i in 0..max_dump {
                if let Some(desc) = self.desc_list.descriptor(i as usize) {
                    let data_addr = ((desc.cur_addr_hi as u64) << 32) | (desc.cur_addr_lo as u64);
                    let next_desc = ((desc.cur_desc_hi as u64) << 32) | (desc.cur_desc_lo as u64);

                    debug!("\tattribute: 0x{:x}", desc.attribute);
                    debug!("\tnon1: 0x{:x}", desc.non1);
                    debug!("\tlen: 0x{:x}", desc.len);
                    debug!("\tnon2: 0x{:x}", desc.non2);
                    debug!("\tdata_addr: 0x{data_addr:x}");
                    debug!("\tnext_desc: 0x{next_desc:x}");

                    // interpret the attribute flags
                    let mut flags = Vec::new();
                    if desc.attribute & FSDIF_IDMAC_DES0_FD != 0 {
                        flags.push("FD");
                    }
                    if desc.attribute & FSDIF_IDMAC_DES0_LD != 0 {
                        flags.push("LD");
                    }
                    if desc.attribute & FSDIF_IDMAC_DES0_ER != 0 {
                        flags.push("ER");
                    }
                    if desc.attribute & FSDIF_IDMAC_DES0_CH != 0 {
                        flags.push("CH");
                    }
                    if desc.attribute & FSDIF_IDMAC_DES0_OWN != 0 {
                        flags.push("OWN");
                    }
                    debug!("\tflags: [{}]", flags.join(", "));
                }
            }
        }
        debug!("dump ok");
    }

    /// Start command and data transfer in DMA mode
    pub fn dma_transfer(&mut self, cmd_data: &mut MCICommand) -> MCIResult {
        cmd_data.success_set(false);

        if !self.is_ready {
            error!("Device is not yet initialized!");
            return Err(MCIError::NotInit);
        }

        if self.config.trans_mode() != MCITransMode::DMA {
            error!("Device is not configured in DMA transfer mode!");
            return Err(MCIError::InvalidState);
        }

        // for removable media, check if card exists
        if !self.config.non_removable() && !self.check_if_card_exist() {
            error!("card is not detected !!!");
            return Err(MCIError::NoCard);
        }

        // wait previous command finished and card not busy
        self.poll_wait_busy_card()?;

        // clear raw interrupt status
        self.config
            .reg()
            .write_reg(MCIRawInts::from_bits_truncate(0xFFFFE));

        /* reset fifo and DMA before transfer */
        self.ctrl_reset(MCICtrl::FIFO_RESET | MCICtrl::DMA_RESET)?;

        // enable use of DMA
        self.config
            .reg()
            .modify_reg(|reg| MCICtrl::USE_INTERNAL_DMAC | reg);
        self.config.reg().modify_reg(|reg| MCIBusMode::DE | reg);

        // transfer data
        if cmd_data.get_data().is_some() {
            self.dma_transfer_data(cmd_data.get_data().unwrap())?;
        }

        // transfer command
        self.cmd_transfer(cmd_data)?;

        Ok(())
    }

    /// start DMA transfers for data
    pub(crate) fn dma_transfer_data(&mut self, data: &MCIData) -> MCIResult {
        self.interrupt_mask_set(
            MCIInterruptType::GeneralIntr,
            MCIIntMask::INTS_DATA_MASK.bits(),
            true,
        );
        self.interrupt_mask_set(
            MCIInterruptType::DmaIntr,
            MCIDMACIntEn::INTS_MASK.bits(),
            true,
        );

        self.setup_dma_descriptor(data)?;

        let data_len = data.blkcnt() * data.blksz();
        debug!(
            "Descriptor count: {}, trans bytes: {}, block size: {}, desc_phys: 0x{:x}, data_phys: 0x{:x}",
            self.desc_list.len(),
            data_len,
            data.blksz(),
            self.desc_list.first_desc_dma(),
            data.buf_dma().map(|d| d.bus_addr()).unwrap_or(0)
        );

        self.descriptor_set(self.desc_list.first_desc_dma());
        self.trans_bytes_set(data_len);
        self.blksize_set(data.blksz());

        Ok(())
    }

    /// Setup DMA descriptor for data transfer
    pub(crate) fn setup_dma_descriptor(&mut self, data: &MCIData) -> MCIResult {
        // 一个desc可以传输的块数
        let desc_blocks = self.desc_list.desc_trans_sz / data.blksz();
        let mut remain_blocks = data.blkcnt();
        let buf_dma = data.buf_dma().unwrap();
        let mut buf_addr = buf_dma.bus_addr() as usize;

        let data_len = data.blkcnt() * data.blksz();

        // calculate the number of descriptors needed
        let desc_num = if data_len <= self.desc_list.desc_trans_sz {
            1
        } else {
            let count = data_len / self.desc_list.desc_trans_sz;
            if data_len % self.desc_list.desc_trans_sz == 0 {
                count
            } else {
                count + 1
            }
        };

        debug!(
            "DMA transfer: data_buf=0x{:x}, use {} desc",
            buf_addr, desc_num
        );

        // need to ensure the descriptor list has enough capacity
        self.desc_list.ensure_capacity(desc_num)?;

        debug!(
            "DMA descriptor allocated: desc_phys=0x{:x}",
            self.desc_list.first_desc_dma()
        );

        // need to clear the descriptors before setup
        for i in 0..desc_num as usize {
            self.desc_list.set_descriptor(i, FSdifIDmaDesc::default())?;
        }

        // set up each descriptor
        for i in 0..desc_num {
            let trans_blocks = if remain_blocks <= desc_blocks {
                remain_blocks
            } else {
                desc_blocks
            };

            let is_first = i == 0;
            let is_last = desc_num - 1 == i;

            let mut desc = FSdifIDmaDesc::default();

            // set property of descriptor entry
            desc.attribute = FSDIF_IDMAC_DES0_CH | FSDIF_IDMAC_DES0_OWN;
            if is_first {
                desc.attribute |= FSDIF_IDMAC_DES0_FD;
            }
            if is_last {
                desc.attribute |= FSDIF_IDMAC_DES0_LD | FSDIF_IDMAC_DES0_ER;
            }

            // set data length in transfer
            desc.non1 = 0u32;
            desc.len = trans_blocks * data.blksz();

            // set data buffer for transfer
            if buf_addr % data.blksz() as usize != 0 {
                error!(
                    "Data buffer 0x{:x} do not align to {}!",
                    buf_addr,
                    data.blksz()
                );
                return Err(MCIError::DmaBufUnalign);
            }

            // set data buffer address
            if cfg!(target_arch = "aarch64") {
                desc.cur_addr_hi = ((buf_addr >> 32) & 0xFFFF_FFFF) as u32;
                desc.cur_addr_lo = (buf_addr & 0xFFFF_FFFF) as u32;
            } else {
                desc.cur_addr_hi = 0;
                desc.cur_addr_lo = (buf_addr & 0xFFFF_FFFF) as u32;
            }

            // set address of next descriptor entry, NULL for last entry
            let next_desc_addr = if is_last {
                0
            } else {
                self.desc_list.first_desc_dma()
                    + (i + 1) as usize * core::mem::size_of::<FSdifIDmaDesc>()
            };

            if next_desc_addr != 0 && next_desc_addr % core::mem::size_of::<FSdifIDmaDesc>() != 0 {
                error!("DMA descriptor 0x{next_desc_addr:x} do not align!");
                return Err(MCIError::DmaBufUnalign);
            }

            if cfg!(target_arch = "aarch64") {
                desc.cur_desc_hi = ((next_desc_addr >> 32) & 0xFFFF_FFFF) as u32;
                desc.cur_desc_lo = (next_desc_addr & 0xFFFF_FFFF) as u32;
            } else {
                desc.cur_desc_hi = 0;
                desc.cur_desc_lo = (next_desc_addr & 0xFFFF_FFFF) as u32;
            }

            // set the descriptor in the list
            self.desc_list.set_descriptor(i as usize, desc)?;

            buf_addr += desc.len as usize;
            remain_blocks -= trans_blocks;

            debug!(
                "Desc[{}]: len={}, data_addr=0x{:x}, next_desc=0x{:x}, is_last={}",
                i,
                desc.len,
                ((desc.cur_addr_hi as u64) << 32) | (desc.cur_addr_lo as u64),
                next_desc_addr,
                is_last
            );
        }

        unsafe {
            core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
            #[cfg(target_arch = "aarch64")]
            core::arch::asm!("dsb sy");
        }

        self.dump_dma_descriptor(desc_num);
        debug!(
            "DMA descriptor setup completed: {} descriptors, desc_phys=0x{:x}, data_phys=0x{:x}",
            desc_num,
            self.desc_list.first_desc_dma(),
            data.buf_dma().unwrap().bus_addr()
        );

        Ok(())
    }

    pub fn init_dma(&mut self) -> MCIResult {
        if !self.is_ready {
            error!("Device is not yet initialized!");
            return Err(MCIError::NotInit);
        }

        if self.config.trans_mode() != MCITransMode::DMA {
            error!("Device is not configured in DMA transfer mode!");
            return Err(MCIError::InvalidState);
        }

        self.desc_list = FSdifIDmaDescList::new();

        debug!("DMA initialization completed");
        Ok(())
    }

    /// Wait DMA transfer finished by poll
    pub fn poll_wait_dma_end(&mut self, cmd_data: &mut MCICommand) -> MCIResult {
        let wait_bits = if cmd_data.get_data().is_none() {
            MCIIntMask::CMD_BIT.bits()
        } else {
            MCIIntMask::CMD_BIT.bits() | MCIIntMask::DTO_BIT.bits()
        };
        let mut reg_val;

        if !self.is_ready {
            error!("Device is not yet initialized!");
            return Err(MCIError::NotInit);
        }

        if self.config.trans_mode() != MCITransMode::DMA {
            error!("Device is not configured in DMA transfer mode!");
            return Err(MCIError::InvalidState);
        }

        /* wait command done or data timeout */
        let mut delay = RETRIES_TIMEOUT;
        loop {
            reg_val = self.config.reg().read_reg::<MCIRawInts>().bits();

            if delay % 100 == 0 {
                debug!(
                    "Polling dma end, reg_val = 0x{:x}, delay: {}, wait_bits: {}, result: {}",
                    reg_val,
                    delay,
                    wait_bits,
                    wait_bits & reg_val == wait_bits
                );
            }

            if wait_bits & reg_val == wait_bits || delay == 0 {
                break;
            }

            delay -= 1;
        }

        /* clear status to ack data done */
        self.raw_status_clear();

        if wait_bits & reg_val != wait_bits && delay == 0 {
            error!("Wait command done timeout, raw ints: 0x{reg_val:x}!");
            return Err(MCIError::CmdTimeout);
        }

        if cmd_data.get_data().is_some() {
            let read = cmd_data.flag().contains(MCICmdFlag::READ_DATA);
            if !read {
                unsafe {
                    dsb();
                }
            }
        }

        Ok(())
    }
}
