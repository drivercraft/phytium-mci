//! # MCI Host Device Trait
//!
//! This module defines the `MCIHostDevice` trait which abstracts the underlying
//! hardware device operations for SD/MMC card host controllers.

use core::ptr::NonNull;

use alloc::vec::Vec;

use crate::mci::MCICmdData;

use super::MCIHost;
use super::MCIHostCardIntFn;
use super::constants::*;
use super::err::*;
use super::mci_host_card_detect::MCIHostCardDetect;
use super::mci_host_transfer::MCIHostTransfer;
use super::mci_sdif::constants::SDStatus;

/// Trait defining MCI host device operations.
///
/// This trait provides an abstraction layer over the underlying hardware
/// for SD/MMC card operations. Implementations of this trait handle
/// device-specific functionality.
#[allow(unused)]
pub(crate) trait MCIHostDevice {
    /// Initialize the device.
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the device registers
    /// * `host` - Reference to the host controller
    fn init(&self, addr: NonNull<u8>, host: &MCIHost) -> MCIHostStatus;

    /// Perform low-level device initialization.
    ///
    /// # Arguments
    ///
    /// * `addr` - Base address of the device registers
    /// * `host` - Reference to the host controller
    fn do_init(&self, addr: NonNull<u8>, host: &MCIHost) -> MCIHostStatus;

    /* sdmmc host operations */
    /// Deinitialize the device.
    fn deinit(&self);

    /// Reset the device.
    fn reset(&self) -> MCIHostStatus;

    /* set sdmmc host mode and get host status */
    /// Switch the card operation voltage.
    ///
    /// # Arguments
    ///
    /// * `voltage` - Target operation voltage
    /// * `host` - Reference to the host controller
    fn switch_to_voltage(&self, voltage: MCIHostOperationVoltage, host: &MCIHost) -> MCIHostStatus;

    /// Execute sampling tuning for high-speed modes.
    ///
    /// # Arguments
    ///
    /// * `tuning_cmd` - Tuning command to execute
    /// * `rev_buf` - Buffer to receive tuning data
    /// * `block_size` - Block size for tuning transfer
    fn execute_tuning(
        &self,
        tuning_cmd: u32,
        rev_buf: &mut Vec<u32>,
        block_size: u32,
    ) -> MCIHostStatus;

    /// Enable or disable DDR mode.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable DDR mode
    /// * `nibble_pos` - Nibble position for DDR
    fn enable_ddr_mode(&self, enable: bool, nibble_pos: u32);

    /// Enable or disable HS400 mode.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable HS400 mode
    fn enable_hs400_mode(&self, enable: bool);

    /// Enable or disable strobe DLL.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable strobe DLL
    fn enable_strobe_dll(&self, enable: bool);

    /// Get signal line status.
    ///
    /// # Arguments
    ///
    /// * `signal_line` - Signal line to check
    ///
    /// # Returns
    ///
    /// `true` if the signal line is active, `false` otherwise
    fn get_signal_line_status(&self, signal_line: u32) -> bool;

    /// Convert data to little-endian format.
    ///
    /// # Arguments
    ///
    /// * `data` - Data buffer to convert
    /// * `word_size` - Size of each word in 32-bit units
    /// * `format` - Current data format (MSB/LSB first)
    /// * `host` - Reference to the host controller
    fn convert_data_to_little_endian(
        &self,
        data: &mut Vec<u32>,
        word_size: usize,
        format: MCIHostDataPacketFormat,
        host: &MCIHost,
    ) -> MCIHostStatus;

    /* card related functions */
    // TODO: MCIHostCardDetect introduced here must be a member of MCIHostDevice implementation, leaving a get_MCIHostDevice interface here
    /// Initialize card detection.
    ///
    /// # Arguments
    ///
    /// * `cd` - Card detection configuration
    fn card_detect_init(&self, cd: &MCIHostCardDetect) -> MCIHostStatus;

    /// Set card power on/off.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to power on the card
    fn card_power_set(&self, enable: bool);

    /// Force clock on/off.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to force clock on
    fn force_clock_on(&self, enable: bool);

    /// Enable or disable card interrupt.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable card interrupt
    /// * `host` - Reference to the host controller
    fn card_int_enable(&self, enable: bool, host: &MCIHost) -> MCIHostStatus;

    // TODO: Same as above
    /// Initialize card interrupt.
    ///
    /// # Arguments
    ///
    /// * `sdio_int` - Card interrupt handler function
    fn card_int_init(&self, sdio_int: &MCIHostCardIntFn) -> MCIHostStatus;

    /// Set card data bus width.
    ///
    /// # Arguments
    ///
    /// * `data_bus_width` - Bus width (1/4/8 bit)
    fn card_bus_width_set(&self, data_bus_width: MCIHostBusWdith);

    /// Poll for card detection status.
    ///
    /// # Arguments
    ///
    /// * `wait_card_status` - Status to wait for (inserted/removed)
    /// * `timeout` - Timeout in milliseconds
    /// * `host` - Reference to the host controller
    fn card_detect_status_polling(
        &self,
        wait_card_status: SDStatus,
        timeout: u32,
        host: &MCIHost,
    ) -> MCIHostStatus;

    /// Get current card detection status.
    ///
    /// # Returns
    ///
    /// Current card status (inserted/removed)
    fn card_detect_status(&self) -> SDStatus;

    /// Send card active command.
    fn card_active_send(&self);

    /// Set card clock frequency.
    ///
    /// # Arguments
    ///
    /// * `target_clock` - Target clock frequency in Hz
    /// * `host` - Reference to the host controller
    ///
    /// # Returns
    ///
    /// Actual clock frequency set
    fn card_clock_set(&self, target_clock: u32, host: &MCIHost) -> u32;

    /// Check if card is busy.
    ///
    /// # Returns
    ///
    /// `true` if card is busy, `false` otherwise
    fn card_is_busy(&self) -> bool;

    /* data transfer related functions */
    /// Pre-command processing.
    ///
    /// # Arguments
    ///
    /// * `content` - Transfer content to process
    /// * `host` - Reference to the host controller
    fn pre_command(&self, content: &mut MCIHostTransfer, host: &MCIHost) -> MCIHostStatus;

    /// Convert command information to MCI command format.
    ///
    /// # Arguments
    ///
    /// * `in_trans` - Transfer content to convert
    ///
    /// # Returns
    ///
    /// MCI command data structure
    fn covert_command_info(&self, in_trans: &mut MCIHostTransfer) -> MCICmdData;

    /// Execute command and data transfer.
    ///
    /// # Arguments
    ///
    /// * `content` - Transfer content to execute
    /// * `host` - Reference to the host controller
    fn transfer_function(&self, content: &mut MCIHostTransfer, host: &MCIHost) -> MCIHostStatus;

    /* boot related functions */
    // TODO: These will never be used
    // fn start_boot(&self, host_config: &MCIHostBootConfig, cmd: &MCIHostCmd, buffer: &mut [u8]) -> MCIHostStatus;
    // fn read_boot_data(&self, host_config: &MCIHostBootConfig, buffer: &mut [u8]) -> MCIHostStatus;
    // fn enable_boot(&self, enable: bool);
}
