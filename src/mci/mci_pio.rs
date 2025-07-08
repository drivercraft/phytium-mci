use crate::mci::MCICommand;

use super::MCI;
use super::constants::*;
use super::err::*;
use super::mci_data::MCIData;
use super::regs::*;
use log::*;

impl MCI {
    pub(crate) fn pio_write_data(&self, data: &MCIData) -> MCIResult {
        let reg = self.config.reg();
        let wr_times = (data.datalen() / 4) as usize; /* u8 --> u32 */
        let buf = if let Some(buf) = data.buf() {
            buf
        } else {
            return Err(MCIError::NotInit);
        };

        /* write fifo data */
        reg.write_reg(MCICmd::DAT_WRITE);
        for item in buf.iter().take(wr_times) {
            reg.write_reg(MCIDataReg::from_bits_truncate(*item));
        }
        Ok(())
    }

    pub(crate) fn pio_read_data(&self, data: &mut MCIData) -> MCIResult {
        let reg = self.config.reg();
        let datalen = data.datalen();
        let rd_times = (datalen / 4) as usize; /* u8 --> u32 */
        let buf = if let Some(buf) = data.buf_mut() {
            buf
        } else {
            return Err(MCIError::NotInit);
        };

        buf.clear();

        if datalen > MCI_MAX_FIFO_CNT {
            error!(
                "Fifo do not support writing more than 0x{:x}.",
                MCI_MAX_FIFO_CNT
            );
            return Err(MCIError::NotSupport);
        }

        for _ in 0..rd_times {
            buf.push(reg.read_reg::<MCIDataReg>().bits());
        }

        Ok(())
    }

    /// Start command and data transfer in PIO mode
    pub fn pio_transfer(&self, cmd_data: &mut MCICommand) -> MCIResult {
        let read = cmd_data.flag().contains(MCICmdFlag::READ_DATA);
        let reg = self.config.reg();

        cmd_data.success_set(false);

        if !self.is_ready {
            error!("device is not yet initialized!!!");
            return Err(MCIError::NotInit);
        }
        if self.config.trans_mode() != MCITransMode::PIO {
            error!("device is not configure in PIO transfer mode.");
            return Err(MCIError::InvalidState);
        }

        /* for removable media, check if card exists */
        if !self.config.non_removable() && !self.check_if_card_exist() {
            error!("card is not detected !!!");
            return Err(MCIError::NoCard);
        }

        /* wait previous command finished and card not busy */
        self.poll_wait_busy_card()?;

        /* reset fifo and not use DMA */
        reg.clear_reg(MCICtrl::USE_INTERNAL_DMAC);
        self.ctrl_reset(MCICtrl::FIFO_RESET)?;
        reg.clear_reg(MCIBusMode::DE);

        /* transfer data */
        if let Some(data) = cmd_data.get_mut_data() {
            /* while in PIO mode, max data transferred is 0x800 */
            if data.datalen() > MCI_MAX_FIFO_CNT {
                error!(
                    "Fifo do not support writing more than 0x{:x}.",
                    MCI_MAX_FIFO_CNT
                );
                return Err(MCIError::NotSupport);
            }

            /* set transfer data length and block size */
            self.trans_bytes_set(data.datalen());
            self.blksize_set(data.blksz());

            /* if need to write, write to fifo before send command */
            if !read {
                /* invalide buffer for data to write */
                unsafe { dsb() };
                self.pio_write_data(data)?;
            }
        }

        self.cmd_transfer(cmd_data)?;

        Ok(())
    }

    /// Wait PIO transfer finished by poll
    pub fn poll_wait_pio_end(&mut self, cmd_data: &mut MCICommand) -> MCIResult {
        let read = cmd_data.flag().contains(MCICmdFlag::READ_DATA);
        let reg = self.config.reg();

        if !self.is_ready {
            error!("device is not yet initialized!!!");
            return Err(MCIError::NotInit);
        }

        if MCITransMode::PIO != self.config.trans_mode() {
            error!("device is not configure in PIO transfer mode.");
            return Err(MCIError::InvalidState);
        }

        trace!("wait for PIO cmd to finish ...");
        if let Err(err) = reg.retry_for(
            |reg: MCIRawInts| {
                let result = reg.contains(MCIRawInts::CMD_BIT);
                MCI::relax_handler();
                result
            },
            Some(RETRIES_TIMEOUT),
        ) {
            error!(
                "wait cmd done timeout, raw ints: 0x{:x}",
                self.raw_status_get()
            );
            return Err(err);
        }

        /* if need to read data, read fifo after send command */
        if cmd_data.get_data().is_some() && read {
            trace!("wait for PIO data to read ...");
            if let Err(err) = reg.retry_for(
                |reg| {
                    MCI::relax_handler();
                    (MCIRawInts::DTO_BIT & reg).bits() != 0
                },
                Some(RETRIES_TIMEOUT),
            ) {
                self.raw_status_clear();
                return Err(err);
            }

            /* clear status to ack */
            self.raw_status_clear();
            trace!(
                "card cnt: 0x{:x}, fifo cnt: 0x{:x}",
                reg.read_reg::<MCITranCardCnt>(),
                reg.read_reg::<MCITranFifoCnt>()
            );
        }

        /* clear status to ack cmd done */
        self.raw_status_clear();
        Ok(())
    }
}
