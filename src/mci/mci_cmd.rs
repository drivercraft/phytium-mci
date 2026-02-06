use core::time::Duration;

use log::*;

use crate::sleep;

use super::MCI;
use super::constants::*;
use super::err::*;
use super::mci_cmddata::MCICmdData;
use super::regs::*;

impl MCI {
    pub(crate) fn private_cmd_send(&self, cmd: MCICmd, arg: u32) -> MCIResult {
        let reg = self.config.reg();

        reg.retry_for(
            |reg: MCIStatus| !reg.contains(MCIStatus::DATA_BUSY),
            Some(RETRIES_TIMEOUT),
        )?;
        reg.write_reg(MCICmdArg::from_bits_truncate(arg));

        unsafe { dsb() }; /* drain writebuffer */

        let cmd_reg = MCICmd::START | cmd;

        reg.write_reg(cmd_reg);
        reg.retry_for(
            |reg: MCICmd| !reg.contains(MCICmd::START),
            Some(RETRIES_TIMEOUT),
        )?;

        Ok(())
    }

    pub(crate) fn private_cmd11_send(&self, cmd: MCICmd) -> MCIResult {
        let reg = self.config.reg();
        unsafe { dsb() }; /* drain writebuffer */
        reg.write_reg(MCICmd::START | cmd);
        reg.retry_for(
            |reg| (MCICmd::START & reg).bits() == 0,
            Some(RETRIES_TIMEOUT),
        )?;
        Ok(())
    }

    pub(crate) fn cmd_transfer(&self, cmd_data: &MCICmdData) -> MCIResult {
        let mut raw_cmd = MCICmd::empty();
        let reg = self.config.reg();

        if self.curr_timing.use_hold() {
            raw_cmd |= MCICmd::USE_HOLD_REG;
        }

        let flag = cmd_data.flag();
        if flag.contains(MCICmdFlag::ABORT) {
            raw_cmd |= MCICmd::STOP_ABORT;
        }
        /* Command requires card initialization, such as CMD-0 */
        if flag.contains(MCICmdFlag::NEED_INIT) {
            raw_cmd |= MCICmd::INIT;
        }
        /* Command involves voltage switching */
        if flag.contains(MCICmdFlag::SWITCH_VOLTAGE) {
            raw_cmd |= MCICmd::VOLT_SWITCH;
        }
        /* Command transfer process accompanied by data transfer */
        if flag.contains(MCICmdFlag::EXP_DATA) {
            raw_cmd |= MCICmd::DAT_EXP;
            if flag.contains(MCICmdFlag::WRITE_DATA) {
                raw_cmd |= MCICmd::DAT_WRITE;
            }
        }
        /* Command requires CRC check */
        if flag.contains(MCICmdFlag::NEED_RESP_CRC) {
            raw_cmd |= MCICmd::RESP_CRC;
        }
        /* Command requires response */
        if flag.contains(MCICmdFlag::EXP_RESP) {
            raw_cmd |= MCICmd::RESP_EXP;
            if flag.contains(MCICmdFlag::EXP_LONG_RESP) {
                raw_cmd |= MCICmd::RESP_LONG;
            }
        }
        raw_cmd |= MCICmd::from_bits_truncate(set_reg32_bits!(cmd_data.cmdidx(), 5, 0));
        debug!(
            "============[{}-{}]@0x{:x} begin ============",
            {
                if self.prev_cmd == Self::EXT_APP_CMD {
                    "ACMD"
                } else {
                    "CMD"
                }
            },
            cmd_data.cmdidx(),
            reg.addr.as_ptr() as usize
        );
        debug!("    cmd: 0x{:x}", raw_cmd.bits());
        debug!("    arg: 0x{:x}", cmd_data.cmdarg());
        /* enable related interrupt */
        self.interrupt_mask_set(
            MCIIntrType::GeneralIntr,
            MCIIntMask::INTS_CMD_MASK.bits(),
            true,
        );
        self.private_cmd_send(raw_cmd, cmd_data.cmdarg())?;
        Ok(())
    }

    pub(crate) fn cmd_response_get(&mut self, cmd_data: &mut MCICmdData) -> MCIResult {
        #[cfg(feature = "pio")]
        let read = cmd_data.flag().contains(MCICmdFlag::READ_DATA);

        if !self.is_ready {
            error!("device is not yet initialized!!!");
            return Err(MCIError::NotInit);
        }

        #[cfg(feature = "pio")]
        if let Some(data) = cmd_data.get_mut_data()
            && read
            && MCITransMode::PIO == self.config.trans_mode()
        {
            self.pio_read_data(data)?;
        }

        /* check response of cmd */
        let flag = *cmd_data.flag();
        let reg = self.config.reg();
        if flag.contains(MCICmdFlag::EXP_RESP) {
            let response = cmd_data.get_mut_response();
            if flag.contains(MCICmdFlag::EXP_LONG_RESP) {
                response[0] = reg.read_reg::<MCIResp0>().bits();
                response[1] = reg.read_reg::<MCIResp1>().bits();
                response[2] = reg.read_reg::<MCIResp2>().bits();
                response[3] = reg.read_reg::<MCIResp3>().bits();
                trace!(
                    "    resp: 0x{:x}-0x{:x}-0x{:x}-0x{:x}",
                    response[0], response[1], response[2], response[3]
                );
            } else {
                response[0] = reg.read_reg::<MCIResp0>().bits();
                response[1] = 0;
                response[2] = 0;
                response[3] = 0;
                trace!("    resp: 0x{:x}", response[0]);
            }
        }

        cmd_data.success_set(true); /* cmd / data transfer finished successful */

        sleep(Duration::from_micros(50)); // Allow some time for the command to settle

        debug!(
            "============[{}-{}]@0x{:x} end ============",
            {
                if self.prev_cmd == Self::EXT_APP_CMD {
                    "ACMD"
                } else {
                    "CMD"
                }
            },
            cmd_data.cmdidx(),
            reg.addr.as_ptr() as usize
        );

        trace!("cmd response get done ...");

        /* disable related interrupt */
        self.interrupt_mask_set(
            MCIIntrType::GeneralIntr,
            (MCIIntMask::INTS_CMD_MASK | MCIIntMask::INTS_DATA_MASK).bits(),
            false,
        );

        self.interrupt_mask_set(MCIIntrType::DmaIntr, MCIDMACIntEn::INTS_MASK.bits(), false);
        trace!("cmd send done ...");

        self.prev_cmd = cmd_data.cmdidx();

        Ok(())
    }
}
