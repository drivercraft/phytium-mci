//! MMC/SD Host Controller Protocol Layer.
//!
//! This module implements the SD/MMC host controller protocol, providing the
//! abstraction layer between the hardware controller and the card-specific
//! drivers. It handles command issuing, data transfers, and card state management.
//!
//! # Overview
//!
//! The `MCIHost` provides:
//! - **Command management** - Sending commands and receiving responses
//! - **Data transfer** - Block read/write operations with DMA or PIO
//! - **Card state** - Tracking card voltage, bus width, and clock frequency
//! - **Card detection** - Automatic card insertion/removal detection
//! - **Capability reporting** - Host controller capabilities and limits
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │  Card Driver (SdCard, EmmcCard)    │
//! └──────────────┬──────────────────────┘
//!                │
//! ┌──────────────▼──────────────────────┐
//! │         MCIHost                     │
//! │  - Command sequencing               │
//! │  - Card state management            │
//! │  - Transfer coordination            │
//! └──────────────┬──────────────────────┘
//!                │
//! ┌──────────────▼──────────────────────┐
//! │         SDIFDev                     │
//! │  - Hardware register access         │
//! │  - DMA/PIO control                  │
//! │  - Interrupt handling               │
//! └─────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use phytium_mci::mci_host::{MCIHost, MCIHostConfig};
//! use core::ptr::NonNull;
//!
//! // Create host configuration
//! let config = MCIHostConfig::new();
//!
//! // Create host instance
//! let mut host = MCIHost::new(device, config);
//!
//! // Initialize the host
//! host.init(NonNull::new_unchecked(0x2800_1000 as *mut u8))?;
//! ```

#[allow(unused)]
mod constants;
pub mod err;
mod mci_card_base;
mod mci_host_card_detect;
mod mci_host_config;
mod mci_host_transfer;
pub mod mci_sdif;
pub mod sd;

use core::{cell::Cell, ptr::NonNull};

use alloc::{boxed::Box, rc::Rc};

use constants::*;
use err::{MCIHostError, MCIHostStatus};
use mci_host_card_detect::MCIHostCardDetect;
use mci_host_config::MCIHostConfig;
use mci_host_transfer::{MCIHostCmd, MCIHostTransfer};
use mci_sdif::sdif_device::SDIFDev;

type MCIHostCardIntFn = fn();

/// MMC/SD Host Controller.
///
/// `MCIHost` represents an SD/MMC host controller instance. It manages the
/// communication between the host and the SD/MMC card, handling commands,
/// data transfers, and card state.
///
/// # Card State
///
/// The host tracks the following card state:
/// - `curr_voltage` - Current operating voltage (3.3V or 1.8V)
/// - `curr_bus_width` - Current bus width (1, 4, or 8 bits)
/// - `curr_clock_freq` - Current clock frequency in Hz
///
/// # Capabilities
///
/// The host reports its capabilities through the `capability` field:
/// - Supported voltage ranges
/// - Supported bus widths
/// - High-speed mode support
/// - DMA support
/// - UHS-I mode support
///
/// # Card Detection
///
/// Optional card detection is available through the `cd` field when
/// supported by the hardware.
#[allow(unused)]
pub struct MCIHost {
    pub(crate) dev: Box<SDIFDev>,
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
    //todo uint8_t tuningType not yet ported
}

#[allow(unused)]
impl MCIHost {
    pub(crate) fn new(dev: Box<SDIFDev>, config: MCIHostConfig) -> Self {
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

    pub(crate) fn init(&mut self, addr: NonNull<u8>) -> MCIHostStatus {
        self.dev.init(addr, self)
    }
}
