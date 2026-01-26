//! DMA transfer implementation for MCI operations
//!
//! This module provides DMA (Direct Memory Access) transfer capabilities
//! for high-performance data transfers with SD/MMC cards.

use core::ptr::NonNull;

use alloc::vec::Vec;
use log::*;

use crate::flush;

use super::MCI;
use super::consts::*;
use super::err::*;
use super::mci_data::MCIData;
use super::regs::*;

/// DMA descriptor for chained DMA transfers
///
/// Each descriptor describes a single DMA transfer, including buffer address,
/// length, and control flags. Descriptors can be chained together for multi-block
/// transfers.
#[derive(Default)]
pub struct FSdifIDmaDesc {
    /// Descriptor attributes and control flags
    pub attribute: u32,
    /// Reserved field
    pub non1: u32,
    /// Buffer length in bytes
    pub len: u32,
    /// Reserved field
    pub non2: u32,
    /// Buffer address low 32 bits
    pub addr_lo: u32,
    /// Buffer address high 32 bits
    pub addr_hi: u32,
    /// Next descriptor address low 32 bits
    pub desc_lo: u32,
    /// Next descriptor address high 32 bits
    pub desc_hi: u32,
}

/// DMA descriptor list for chained transfers
///
/// Manages a linked list of DMA descriptors for efficient multi-block transfers.
pub struct FSdifIDmaDescList {
    /// Pointer to the first descriptor in the chain
    pub first_desc: *mut FSdifIDmaDesc,
    /// Physical address of the first descriptor
    pub first_desc_dma: usize,
    /// Number of descriptors in the list
    pub desc_num: u32,
    /// Bytes transferred by a single descriptor
    pub desc_trans_sz: u32,
}

impl Default for FSdifIDmaDescList {
    fn default() -> Self {
        Self::new()
    }
}

impl FSdifIDmaDescList {
    /// Creates a new empty DMA descriptor list
    pub fn new() -> Self {
        FSdifIDmaDescList {
            first_desc: core::ptr::null_mut(),
            first_desc_dma: 0,
            desc_num: 0,
            desc_trans_sz: 0,
        }
    }
}

//* DMA related functions */
impl MCI {
    /// Sets DMA interrupt enable bits
    ///
    /// Enables receive, transmit, and fatal bus error interrupts for DMA transfers.
    pub fn dma_int_set(&mut self) {
        self.config
            .reg()
            .modify_reg(|reg| MCIDMACIntEn::RI | MCIDMACIntEn::TI | MCIDMACIntEn::FBE | reg);
    }

    /// Dumps DMA descriptor information for debugging
    ///
    /// # Arguments
    ///
    /// * `desc_in_use` - Number of descriptors currently in use
    pub fn dump_dma_descriptor(&self, desc_in_use: u32) {
        debug!("{} dma desc in use!", desc_in_use);
        if !self.desc_list.first_desc.is_null() {
            for i in 0..desc_in_use {
                unsafe {
                    let cur_desc = &*self.desc_list.first_desc.add(i as usize);
                    debug!("descriptor no {} @{:p}", i, cur_desc);
                    debug!("\tattribute: 0x{:x}", cur_desc.attribute);
                    debug!("\tnon1: 0x{:x}", cur_desc.non1);
                    debug!("\tlen: 0x{:x}", cur_desc.len);
                    debug!("\tnon2: 0x{:x}", cur_desc.non2);
                    debug!("\taddr_lo: 0x{:x}", cur_desc.addr_lo);
                    debug!("\taddr_hi: 0x{:x}", cur_desc.addr_hi);
                    debug!("\tdesc_lo: 0x{:x}", cur_desc.desc_lo);
                    debug!("\tdesc_hi: 0x{:x}", cur_desc.desc_hi);
                }
            }
        }
    }

    /// setup DMA descriptor list before do transcation
    pub(crate) fn setup_dma_descriptor(&mut self, data: &MCIData) -> MCIResult {
        let desc_list = &self.desc_list;
        // Number of blocks that can be transferred by one descriptor
        let desc_blocks = desc_list.desc_trans_sz / data.blksz();
        let mut remain_blocks = data.blkcnt();
        let mut buf_addr = data.buf_dma();
        let mut trans_blocks: u32; // Blocks transferred in this loop
        let mut is_first;
        let mut is_last;

        let mut desc_num = 1u32;
        let data_len = data.blkcnt() * data.blksz();
        // Calculate how many descriptors are needed for transfer
        if data_len > desc_list.desc_trans_sz {
            desc_num = data_len / desc_list.desc_trans_sz;
            desc_num += if data_len.is_multiple_of(desc_list.desc_trans_sz) {
                0
            } else {
                1
            };
        }

        if desc_num > desc_list.desc_num {
            error!(
                "Transfer descriptor are not enough! desc need: {}, desc available: {}",
                desc_num, desc_list.desc_num
            );
            return Err(MCIError::ShortBuf);
        }

        info!(
            "DMA transfer 0x{:x} use {} desc, total {} available",
            data.buf_dma(),
            desc_num,
            desc_list.desc_num
        );

        // setup DMA descriptor list, so that we just need to update buffer address in each transcation
        let total_size = desc_list.desc_num as usize * core::mem::size_of::<FSdifIDmaDesc>();
        unsafe {
            core::ptr::write_bytes(desc_list.first_desc as *mut u8, 0, total_size);
        }

        for i in 0..desc_num {
            trans_blocks = if remain_blocks <= desc_blocks {
                remain_blocks
            } else {
                desc_blocks
            };
            unsafe {
                let cur_desc = self.desc_list.first_desc.add(i as usize);
                let mut next_desc_addr = desc_list.first_desc_dma
                    + (i + 1) as usize * core::mem::size_of::<FSdifIDmaDesc>();

                is_first = i == 0;
                is_last = desc_num - 1 == i;

                // set properity of descriptor entry
                (*cur_desc).attribute = FSDIF_IDMAC_DES0_CH | FSDIF_IDMAC_DES0_OWN;
                if is_first {
                    (*cur_desc).attribute |= FSDIF_IDMAC_DES0_FD;
                }
                if is_last {
                    (*cur_desc).attribute |= FSDIF_IDMAC_DES0_LD | FSDIF_IDMAC_DES0_ER;
                }

                // set data length in transfer
                (*cur_desc).non1 = 0u32;
                (*cur_desc).len = trans_blocks * data.blksz();

                // set data buffer for transfer
                if !buf_addr.is_multiple_of(data.blksz() as usize) {
                    error!(
                        "Data buffer 0x{:x} do not align to {}!",
                        buf_addr,
                        data.blksz()
                    );
                    return Err(MCIError::DmaBufUnalign);
                }

                if cfg!(target_arch = "aarch64") {
                    (*cur_desc).addr_hi = ((buf_addr >> 32) & 0xFFFF_FFFF) as u32;
                    (*cur_desc).addr_lo = (buf_addr & 0xFFFF_FFFF) as u32;
                } else {
                    (*cur_desc).addr_hi = 0;
                    (*cur_desc).addr_lo = (buf_addr & 0xFFFF_FFFF) as u32;
                }

                // set address of next descriptor entry, NULL for last entry
                next_desc_addr = if is_last { 0 } else { next_desc_addr };
                if !next_desc_addr.is_multiple_of(core::mem::size_of::<FSdifIDmaDesc>()) {
                    // make sure descriptor aligned and not cross page boundary
                    error!("DMA descriptor 0x{:x} do not align!", next_desc_addr);
                    return Err(MCIError::DmaBufUnalign);
                }

                if cfg!(target_arch = "aarch64") {
                    (*cur_desc).desc_hi = ((next_desc_addr >> 32) & 0xFFFF_FFFF) as u32;
                    (*cur_desc).desc_lo = (next_desc_addr & 0xFFFF_FFFF) as u32;
                } else {
                    (*cur_desc).desc_hi = 0;
                    (*cur_desc).desc_lo = (next_desc_addr & 0xFFFF_FFFF) as u32;
                }

                buf_addr += (*cur_desc).len as usize;
                remain_blocks -= trans_blocks;
            }
        }

        flush(
            NonNull::new(desc_list.first_desc).unwrap().cast(),
            desc_num as _,
        );
        self.dump_dma_descriptor(desc_num);
        debug!("set dma desc ok");

        Ok(())
    }

    /// start DMA transfers for data
    pub(crate) fn dma_transfer_data(&mut self, data: &MCIData) -> MCIResult {
        self.interrupt_mask_set(
            MCIIntrType::GeneralIntr,
            MCIIntMask::INTS_DATA_MASK.bits(),
            true,
        );
        self.interrupt_mask_set(MCIIntrType::DmaIntr, MCIDMACIntEn::INTS_MASK.bits(), true);

        self.setup_dma_descriptor(data)?;

        let data_len = data.blkcnt() * data.blksz();
        info!(
            "Descriptor@{:p}, trans bytes: {}, block size: {}",
            self.desc_list.first_desc,
            data_len,
            data.blksz()
        );

        self.descriptor_set(self.desc_list.first_desc_dma);
        self.trans_bytes_set(data_len);
        self.blksize_set(data.blksz());

        Ok(())
    }
}
