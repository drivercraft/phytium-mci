use super::MCIHost;
use super::constants::SDStatus;
use crate::mci::{MCI, MCICommand, MCIConfig, constants::*, mci_data::MCIData, regs::MCIIntMask};
use crate::mci_host::{
    MCIHostCardIntFn, constants::*, err::*, mci_host_card_detect::MCIHostCardDetect,
    mci_host_config::*, mci_host_device::MCIHostDevice, mci_host_transfer::MCIHostTransfer,
    sd::constants::SdCmd,
};

use crate::mci_sleep;
use crate::tools::swap_half_word_byte_sequence_u32;
use alloc::vec::Vec;
use core::{cell::RefCell, ptr::NonNull, time::Duration};
use dma_api::Direction;
use log::*;

pub(crate) struct SDIFDev {
    hc: RefCell<MCI>, // SDIF 硬件控制器
}

impl SDIFDev {
    pub fn new(addr: NonNull<u8>) -> Self {
        Self {
            hc: MCI::new(MCIConfig::new(addr)).into(),
        }
    }
}

impl MCIHostDevice for SDIFDev {
    fn init(&self, addr: NonNull<u8>, host: &MCIHost) -> MCIHostStatus {
        self.do_init(addr, host)
    }

    fn do_init(&self, addr: NonNull<u8>, host: &MCIHost) -> MCIHostStatus {
        let mci_config = MCIConfig::lookup_config(addr);

        *self.hc.borrow_mut() = MCI::new(MCIConfig::lookup_config(addr));

        let restart_mci = MCI::new_restart(MCIConfig::restart(addr));
        restart_mci
            .restart()
            .unwrap_or_else(|e| error!("restart failed: {:?}", e));

        if self.hc.borrow_mut().config_init(&mci_config).is_err() {
            info!("Sdio ctrl init failed.");
            return Err(MCIHostError::Fail);
        }

        if host.config.enable_irq {
            // TODO
        }

        #[cfg(feature = "dma")]
        {
            if self.hc.borrow_mut().init_dma().is_err() {
                error!("idma list set failed!");
                return Err(MCIHostError::Fail);
            }
        }

        Ok(())
    }

    fn deinit(&self) {
        // TODO：FSDIFHOST_RevokeIrq
        let _ = self.hc.borrow_mut().config_deinit();
        info!("Sdio ctrl deinited !!!")
    }

    fn reset(&self) -> MCIHostStatus {
        match self.hc.borrow_mut().restart() {
            Ok(_) => Ok(()),
            Err(_) => Err(MCIHostError::Fail),
        }
    }

    fn switch_to_voltage(&self, voltage: MCIHostOperationVoltage, host: &MCIHost) -> MCIHostStatus {
        match voltage {
            MCIHostOperationVoltage::Voltage300V => {
                host.curr_voltage.set(voltage);
                self.hc.borrow_mut().voltage_1_8v_set(false);
                info!("Switch to 3.0V");
            }
            MCIHostOperationVoltage::Voltage330V => {
                host.curr_voltage.set(voltage);
                self.hc.borrow_mut().voltage_1_8v_set(false);
                info!("Switch to 3.0V");
            }
            MCIHostOperationVoltage::Voltage180V => {
                host.curr_voltage.set(voltage);
                self.hc.borrow_mut().voltage_1_8v_set(true);
                info!("Switch to 1.8V");
            }
            _ => {
                info!("Invalid target voltage !!!");
            }
        }
        Ok(())
    }

    fn execute_tuning(
        &self,
        _tuning_cmd: u32,
        _rev_buf: &mut Vec<u32>,
        _block_size: u32,
    ) -> MCIHostStatus {
        Ok(())
    }

    fn enable_ddr_mode(&self, _enable: bool, _nibble_pos: u32) {
        // TODO：暂时还没有实现
    }

    fn enable_hs400_mode(&self, _enable: bool) {
        info!("Enable HS400 mode Not Implemented !!!");
    }

    fn enable_strobe_dll(&self, _enable: bool) {
        info!("Enable Strobe DLL Not Implemented !!!");
    }

    fn get_signal_line_status(&self, _signal_line: u32) -> bool {
        !self.hc.borrow().check_if_card_busy()
    }

    fn convert_data_to_little_endian(
        &self,
        data: &mut Vec<u32>,
        word_size: usize,
        format: MCIHostDataPacketFormat,
        host: &MCIHost,
    ) -> MCIHostStatus {
        for val in data.iter_mut().take(word_size) {
            *val = match (host.config.endian_mode, format) {
                (MCIHostEndianMode::Little, MCIHostDataPacketFormat::MSBFirst)
                | (MCIHostEndianMode::Big, MCIHostDataPacketFormat::LSBFirst) => val.swap_bytes(),
                (MCIHostEndianMode::HalfWordBig, _) => swap_half_word_byte_sequence_u32(*val),
                _ => *val,
            };
        }
        Ok(())
    }

    fn card_detect_init(&self, _cd: &MCIHostCardDetect) -> MCIHostStatus {
        Ok(())
    }

    fn card_power_set(&self, _enable: bool) {}

    fn card_int_enable(&self, enable: bool, host: &MCIHost) -> MCIHostStatus {
        if MCIHostCardType::SDIO == host.config.card_type {
            self.hc.borrow().interrupt_mask_set(
                MCIInterruptType::GeneralIntr,
                MCIIntMask::SDIO_BIT.bits(),
                enable,
            );
        }
        Ok(())
    }

    fn card_int_init(&self, _sdio_int: &MCIHostCardIntFn) -> MCIHostStatus {
        Ok(())
    }

    fn card_bus_width_set(&self, data_bus_width: MCIHostBusWdith) {
        match data_bus_width {
            MCIHostBusWdith::Bit1 => {
                self.hc.borrow().bus_width_set(data_bus_width as u32);
                info!("Set bus width to 1 bit");
            }
            MCIHostBusWdith::Bit4 => {
                self.hc.borrow().bus_width_set(data_bus_width as u32);
                info!("Set bus width to 4 bit");
            }
            MCIHostBusWdith::Bit8 => {
                self.hc.borrow().bus_width_set(data_bus_width as u32);
                info!("Set bus width to 8 bit");
            }
        }
    }

    fn card_detect_status_polling(
        &self,
        wait_card_status: SDStatus,
        _timeout: u32,
        host: &MCIHost,
    ) -> MCIHostStatus {
        let cd = host.card_detect.as_ref().ok_or(MCIHostError::NoData)?;

        let mut retry_times: usize = 100;

        /* Wait card inserted. */
        loop {
            let is_card_inserted = self.card_detect_status() == SDStatus::Inserted;
            mci_sleep(Duration::from_millis(cd.cd_debounce_ms as u64));
            if wait_card_status == SDStatus::Inserted && is_card_inserted {
                break;
            }

            if wait_card_status == SDStatus::Removed && !is_card_inserted {
                break;
            }

            if retry_times == 0 {
                info!("Wait card insert timeout !!!");
                return Err(MCIHostError::Timeout);
            }

            retry_times -= 1;
        }
        Ok(())
    }

    fn card_detect_status(&self) -> SDStatus {
        if self.hc.borrow().check_if_card_exist() {
            SDStatus::Inserted
        } else {
            SDStatus::Removed
        }
    }

    fn card_active_send(&self) {}

    fn card_clock_set(&self, target_clock: u32, host: &MCIHost) -> u32 {
        // 如果当前时钟频率已经是目标频率，则直接返回
        if host.curr_clock_freq.get() == target_clock {
            return host.curr_clock_freq.get();
        }
        // 尝试设置时钟频率
        if self.hc.borrow_mut().clk_freq_set(target_clock).is_ok() {
            info!("BUS CLOCK: {}", target_clock);
            // 更新实例的时钟频率
            host.curr_clock_freq.set(target_clock);
        } else {
            info!("Failed to update clock");
        }

        host.curr_clock_freq.get()
    }

    fn force_clock_on(&self, enable: bool) {
        self.hc.borrow().clock_set(enable);
    }

    fn card_is_busy(&self) -> bool {
        self.hc.borrow().check_if_card_busy()
    }

    fn pre_command(&self, content: &mut MCIHostTransfer, host: &MCIHost) -> MCIHostStatus {
        let cmd = match content.cmd() {
            Some(cmd) => cmd,
            None => return Err(MCIHostError::NoData),
        };

        let data = match content.data() {
            Some(data) => data,
            None => return Ok(()),
        };

        if cmd.index() == MCIHostCommonCmd::ReadMultipleBlock as u32
            || cmd.index() == MCIHostCommonCmd::WriteMultipleBlock as u32
        {
            let block_count = data.block_count();

            if block_count > 1 {
                return host.block_count_set(block_count);
            }
        }
        Ok(())
    }

    fn convert_command_info(&self, in_trans: &mut MCIHostTransfer) -> MCICommand {
        let in_cmd = match in_trans.cmd() {
            Some(cmd) => cmd,
            None => panic!("Not Inited intrans"),
        };

        let index = in_cmd.index();
        let arg: u32 = in_cmd.argument();
        let mut flag = MCICmdFlag::empty();

        if index == MCIHostCommonCmd::GoIdleState as u32 {
            flag |= MCICmdFlag::NEED_INIT;
        }

        if index == MCIHostCommonCmd::GoInactiveState as u32
            || (index == MCISDIOCommand::RWIODirect as u32
                && (arg >> 9 & 0x1FFFF) == MCISDIOCCCRAddr::IOAbort as u32)
        {
            flag |= MCICmdFlag::ABORT;
        }

        let response_type = in_cmd.response_type();

        if response_type != MCIHostResponseType::None {
            flag |= MCICmdFlag::EXP_RESP;
            if response_type == MCIHostResponseType::R2 {
                /* need 136 bits long response */
                flag |= MCICmdFlag::EXP_LONG_RESP;
            }

            if response_type != MCIHostResponseType::R3 && response_type != MCIHostResponseType::R4
            {
                /* most cmds need CRC */
                flag |= MCICmdFlag::NEED_RESP_CRC;
            }
        }

        if index == SdCmd::VoltageSwitch as u32 {
            /* CMD11 need switch voltage */
            flag |= MCICmdFlag::SWITCH_VOLTAGE;
        }

        let out_data = if let Some(in_data) = in_trans.data_mut() {
            let mut out_data = MCIData::new();

            flag |= MCICmdFlag::EXP_DATA;

            #[cfg(feature = "pio")]
            let (buf, _direction) = if let Some(rx_data) = in_data.rx_data_take() {
                // Handle receive data
                flag |= MCICmdFlag::READ_DATA;
                (rx_data, Direction::FromDevice)
            } else if let Some(tx_data) = in_data.tx_data_take() {
                // Handle transmit data
                flag |= MCICmdFlag::WRITE_DATA;
                (tx_data, Direction::ToDevice)
            } else {
                // Neither rx_data nor tx_data is available
                panic!("Transaction data initialized but contains neither rx_data nor tx_data");
            };

            #[cfg(feature = "dma")]
            let (buf, _direction) = if let Some(rx_data) = in_data.rx_data_take() {
                // Handle receive data
                flag |= MCICmdFlag::READ_DATA;
                (rx_data, Direction::FromDevice)
            } else if let Some(tx_data) = in_data.tx_data_take() {
                // Handle transmit data
                flag |= MCICmdFlag::WRITE_DATA;
                (tx_data, Direction::ToDevice)
            } else {
                // Neither rx_data nor tx_data is available
                panic!("Transaction data initialized but contains neither rx_data nor tx_data");
            };

            out_data.blksz_set(in_data.block_size() as u32);
            out_data.blkcnt_set(in_data.block_count());
            out_data.datalen_set(in_data.block_size() as u32 * in_data.block_count());

            #[cfg(feature = "dma")]
            {
                out_data.buf_dma_set(Some(buf));
                debug!(
                    "buf PA: 0x{:x}, blksz: {}, datalen: {}",
                    out_data.buf_dma().unwrap().bus_addr(),
                    out_data.blksz(),
                    out_data.datalen()
                );
            }

            #[cfg(feature = "pio")]
            {
                out_data.buf_set(Some(buf));
                debug!(
                    "buf PA: 0x{:x?}, blksz: {}, datalen: {}",
                    out_data.buf().unwrap().as_ptr() as usize,
                    out_data.blksz(),
                    out_data.datalen()
                );
            }

            Some(out_data)
        } else {
            None
        };

        let mut out_trans = MCICommand::new();

        out_trans.cmdidx_set(index);
        out_trans.cmdarg_set(arg);
        out_trans.set_data(out_data);
        out_trans.flag_set(flag);

        unsafe {
            dsb();
        }

        out_trans
    }

    fn transfer_function(&self, content: &mut MCIHostTransfer, host: &MCIHost) -> MCIHostStatus {
        self.pre_command(content, host)?;

        let mut cmd_data = self.convert_command_info(content);
        let is_read_transfer = cmd_data.flag().contains(MCICmdFlag::READ_DATA);
        let is_write_transfer = cmd_data.flag().contains(MCICmdFlag::WRITE_DATA);

        let transfer_type = match (is_read_transfer, is_write_transfer) {
            (true, false) => "READ",
            (false, true) => "WRITE",
            (true, true) => "READ_WRITE",
            (false, false) => "NO_DATA",
        };

        debug!("Starting transfer ({})", transfer_type);

        #[cfg(feature = "dma")]
        {
            if let Err(e) = self.hc.borrow_mut().dma_transfer(&mut cmd_data) {
                error!("DMA transfer failed: {:?}", e);
                return Err(MCIHostError::NoData);
            }

            if let Err(e) = self.hc.borrow_mut().poll_wait_dma_end(&mut cmd_data) {
                error!("DMA wait failed: {:?}", e);
                return Err(MCIHostError::NoData);
            }
        }

        #[cfg(feature = "pio")]
        {
            if let Err(e) = self.hc.borrow_mut().pio_transfer(&mut cmd_data) {
                error!("PIO transfer failed: {:?}", e);
                return Err(MCIHostError::NoData);
            }

            if let Err(e) = self.hc.borrow_mut().poll_wait_pio_end(&mut cmd_data) {
                error!("PIO wait failed: {:?}", e);
                return Err(MCIHostError::NoData);
            }
        }

        debug!(
            "MCI transfer completed - cmd: 0x{:x?}, arg: 0x{:x?}, flags: {:x?}",
            cmd_data.cmdidx(),
            cmd_data.cmdarg(),
            cmd_data.flag(),
        );

        if let Err(_) = self.hc.borrow_mut().cmd_response_get(&mut cmd_data) {
            info!("Transfer cmd and data failed !!!");
            return Err(MCIHostError::Timeout);
        }

        // TODO: 这里的 CLONE 会降低驱动速度,需要解决这个性能问题 可能Take出来直接用更好
        #[cfg(feature = "pio")]
        if let Some(_) = content.data() {
            let data = cmd_data.get_data().unwrap();
            unsafe {
                invalidate(
                    data.buf().unwrap().as_ptr() as *const u8,
                    data.buf().unwrap().len() * 4,
                );
            }
            if let Some(rx_data) = data.buf() {
                if let Some(in_data) = content.data_mut() {
                    in_data.rx_data_set(Some(rx_data.clone()));
                }
            }
        }

        #[cfg(feature = "dma")]
        if let Some(_) = content.data() {
            let data = cmd_data.get_mut_data().unwrap();
            unsafe {
                invalidate(
                    data.buf_dma().unwrap().as_ptr() as *const u8,
                    data.buf_dma().unwrap().len() * 4,
                );
            }
            if let Some(rx_data) = data.take_buf_dma() {
                if let Some(in_data) = content.data_mut() {
                    in_data.rx_data_set(Some(rx_data));
                }
            }
        }

        if let Some(cmd) = content.cmd_mut() {
            if cmd.response_type() != MCIHostResponseType::None {
                cmd.response_mut()
                    .copy_from_slice(&cmd_data.get_response()[..4]);
            }
        }

        Ok(())
    }
}
