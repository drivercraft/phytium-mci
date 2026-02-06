//! # MCI Host Module
//!
//! This module provides the host controller abstraction layer for SD/MMC card operations.
//! It implements higher-level card management functions on top of the low-level MCI controller.
//!
//! ## Components
//!
//! - **MCIHost**: Main host controller structure managing card operations
//! - **mci_sdif**: SDIF device implementation
//! - **sd**: SD card specific operations and data structures
//!
//! ## Functionality
//!
//! - Card initialization and detection
//! - Command execution (standard and application-specific)
//! - Data transfer operations
//! - Card capability queries
//! - Voltage and bus width configuration

#[allow(unused)]
mod constants;
pub mod err;
mod mci_card_base;
mod mci_host_card_detect;
mod mci_host_config;
mod mci_host_device;
mod mci_host_transfer;
pub mod mci_sdif;
pub mod sd;

use core::{cell::Cell, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use constants::*;
use err::{MCIHostError, MCIHostStatus};
use log::error;
use mci_host_card_detect::MCIHostCardDetect;
use mci_host_config::MCIHostConfig;
use mci_host_device::MCIHostDevice;
use mci_host_transfer::{MCIHostCmd, MCIHostTransfer};

type MCIHostCardIntFn = fn();

/// MCI Host controller.
///
/// This structure manages the host controller for SD/MMC card operations.
/// It provides higher-level card management functions including:
/// - Card detection and initialization
/// - Command execution
/// - Data transfer management
/// - Voltage and bus width configuration
///
/// # Fields
///
/// - `dev`: The underlying device implementation
/// - `config`: Host configuration
/// - `curr_voltage`: Current operation voltage
/// - `curr_bus_width`: Current bus width (1/4/8 bit)
/// - `curr_clock_freq`: Current clock frequency
/// - `source_clock_hz`: Source clock frequency in Hz
/// - `capability`: Host capabilities
/// - `max_block_count`: Maximum block count for transfer
/// - `max_block_size`: Maximum block size for transfer
/// - `tuning_type`: Tuning type for high-speed modes
/// - `cd`: Optional card detection handler
/// - `card_int`: Card interrupt handler
#[allow(unused)]
pub struct MCIHost {
    pub(crate) dev: Box<dyn MCIHostDevice>,
    pub(crate) config: MCIHostConfig,
    pub(crate) curr_voltage: Cell<MCIHostOperationVoltage>,
    pub(crate) curr_bus_width: u32,
    pub(crate) curr_clock_freq: Cell<u32>,

    pub(crate) source_clock_hz: u32,
    pub(crate) capability: MCIHostCapability,
    pub(crate) max_block_count: Cell<u32>,
    pub(crate) max_block_size: u32,
    pub(crate) tuning_type: u8,

    pub(crate) cd: Option<Rc<MCIHostCardDetect>>, // Card detection
    pub(crate) card_int: MCIHostCardIntFn,
    //? Here uint8_t tuningType, sdmmc_osa_event_t hostEvent, sdmmc_osa_mutex_t lock are not ported
}

#[allow(unused)]
impl MCIHost {
    pub(crate) fn new(dev: Box<dyn MCIHostDevice>, config: MCIHostConfig) -> Self {
        MCIHost {
            dev,
            config,
            curr_voltage: Cell::new(MCIHostOperationVoltage::None),
            curr_bus_width: 0,
            curr_clock_freq: Cell::new(0),
            source_clock_hz: 0,
            capability: MCIHostCapability::empty(),
            max_block_count: Cell::new(0),
            max_block_size: 0,
            tuning_type: 0,
            cd: None,
            card_int: || {},
        }
    }

    pub(crate) fn card_select(&self, relative_address: u32, is_selected: bool) -> MCIHostStatus {
        let mut command = MCIHostCmd::new();

        command.index_set(MCIHostCommonCmd::SelectCard as u32);
        if is_selected {
            command.argument_set(relative_address << 16);
            command.response_type_set(MCIHostResponseType::R1);
        } else {
            command.argument_set(0);
            command.response_type_set(MCIHostResponseType::None);
        }

        let mut content = MCIHostTransfer::new();
        content.set_cmd(Some(command));

        let err = self.dev.transfer_function(&mut content, self);

        let command = content.cmd().unwrap();
        let response = command.response();

        if err.is_err() || response[0] & MCIHostCardStatusFlag::ALL_ERROR_FLAG.bits() != 0 {
            return Err(MCIHostError::TransferFailed);
        }

        Ok(())
    }

    pub(crate) fn application_command_send(&self, relative_address: u32) -> MCIHostStatus {
        let mut command = MCIHostCmd::new();

        command.index_set(MCIHostCommonCmd::ApplicationCommand as u32);
        command.argument_set(relative_address << 16);
        command.response_type_set(MCIHostResponseType::R1);

        let mut content = MCIHostTransfer::new();
        content.set_cmd(Some(command));

        let err = self.dev.transfer_function(&mut content, self);

        let command = content.cmd().unwrap();
        let response = command.response();

        if err.is_err() || response[0] & MCIHostCardStatusFlag::ALL_ERROR_FLAG.bits() != 0 {
            return Err(MCIHostError::TransferFailed);
        }

        if response[0] & MCIHostCardStatusFlag::APPLICATION_COMMAND.bits() == 0 {
            return Err(MCIHostError::CardNotSupport);
        }

        Ok(())
    }

    pub(crate) fn block_count_set(&self, block_count: u32) -> MCIHostStatus {
        let mut command = MCIHostCmd::new();

        command.index_set(MCIHostCommonCmd::SetBlockCount as u32);
        command.argument_set(block_count);
        command.response_type_set(MCIHostResponseType::R1);

        let mut content = MCIHostTransfer::new();
        content.set_cmd(Some(command));

        let err = self.dev.transfer_function(&mut content, self);

        let command = content.cmd().unwrap();
        let response = command.response();

        if err.is_err() || response[0] & MCIHostCardStatusFlag::ALL_ERROR_FLAG.bits() != 0 {
            return Err(MCIHostError::TransferFailed);
        }

        Ok(())
    }

    pub(crate) fn go_idle(&self) -> MCIHostStatus {
        error!("cmd0 starts");
        let mut command = MCIHostCmd::new();

        command.index_set(MCIHostCommonCmd::GoIdleState as u32);

        let mut content = MCIHostTransfer::new();
        content.set_cmd(Some(command));

        let err = self.dev.transfer_function(&mut content, self);

        if err.is_err() {
            return Err(MCIHostError::TransferFailed);
        }

        Ok(())
    }

    pub(crate) fn block_size_set(&self, block_size: u32) -> MCIHostStatus {
        let mut command = MCIHostCmd::new();

        command.index_set(MCIHostCommonCmd::SetBlockLength as u32);
        command.argument_set(block_size);
        command.response_type_set(MCIHostResponseType::R1);

        let mut content = MCIHostTransfer::new();
        content.set_cmd(Some(command));

        let err = self.dev.transfer_function(&mut content, self);

        let command = content.cmd().unwrap();
        let response = command.response();

        if err.is_err() || response[0] & MCIHostCardStatusFlag::ALL_ERROR_FLAG.bits() != 0 {
            return Err(MCIHostError::TransferFailed);
        }

        Ok(())
    }

    pub(crate) fn card_inactive_set(&self) -> MCIHostStatus {
        let mut command = MCIHostCmd::new();

        command.index_set(MCIHostCommonCmd::GoInactiveState as u32);
        command.argument_set(0);
        command.response_type_set(MCIHostResponseType::None);

        let mut content = MCIHostTransfer::new();
        content.set_cmd(Some(command));

        let err = self.dev.transfer_function(&mut content, self);

        if err.is_err() {
            return Err(MCIHostError::TransferFailed);
        }

        Ok(())
    }

    /// Initialize the host controller.
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the device registers
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub(crate) fn init(&mut self, addr: NonNull<u8>) -> MCIHostStatus {
        self.dev.init(addr, self)
    }
}

#[allow(unused)]
impl MCIHost {
    // TODO Wrap dev operations
}
