//! # MCI Host Transfer Structures
//!
//! This module provides structures for command and data transfer operations.

use alloc::vec::Vec;

use super::constants::*;

/// Host transfer structure containing command and data.
pub(crate) struct MCIHostTransfer {
    data: Option<MCIHostData>,
    cmd: Option<MCIHostCmd>,
}

impl MCIHostTransfer {
    /// Create a new empty transfer.
    pub(crate) fn new() -> Self {
        MCIHostTransfer {
            data: None,
            cmd: None,
        }
    }

    /// Get the data portion of the transfer.
    pub(crate) fn data(&self) -> Option<&MCIHostData> {
        self.data.as_ref()
    }

    /// Set the data portion of the transfer.
    pub(crate) fn set_data(&mut self, data: Option<MCIHostData>) {
        self.data = data
    }

    /// Get the command portion of the transfer.
    pub(crate) fn cmd(&self) -> Option<&MCIHostCmd> {
        self.cmd.as_ref()
    }

    /// Set the command portion of the transfer.
    pub(crate) fn set_cmd(&mut self, cmd: Option<MCIHostCmd>) {
        self.cmd = cmd
    }

    /// Get mutable reference to the data portion.
    pub(crate) fn data_mut(&mut self) -> Option<&mut MCIHostData> {
        self.data.as_mut()
    }

    /// Get mutable reference to the command portion.
    pub(crate) fn cmd_mut(&mut self) -> Option<&mut MCIHostCmd> {
        self.cmd.as_mut()
    }
}

/// Host data transfer structure.
#[allow(unused)]
pub(crate) struct MCIHostData {
    /// Whether this is a stream transfer command
    stream_transfer: bool,
    /// Enable auto CMD12
    enable_auto_command12: bool,
    /// Enable auto CMD23
    enable_auto_command23: bool,
    /// Enable ignore error to read/write all data
    enable_ignore_error: bool,
    /// Data type (normal/tuning/boot)
    data_type: u8,
    /// Block size
    block_size: usize,
    /// Block count
    block_count: u32,
    /// Buffer for received data
    rx_data: Option<Vec<u32>>,
    /// Buffer for data to send
    tx_data: Option<Vec<u32>>,
}

#[allow(unused)]
impl MCIHostData {
    pub(crate) fn new() -> Self {
        MCIHostData {
            stream_transfer: false,
            enable_auto_command12: false,
            enable_auto_command23: false,
            enable_ignore_error: false,
            data_type: 0,
            block_size: 0,
            block_count: 0,
            rx_data: None,
            tx_data: None,
        }
    }

    pub(crate) fn stream_transfer(&self) -> bool {
        self.stream_transfer
    }

    pub(crate) fn enable_auto_command12(&self) -> bool {
        self.enable_auto_command12
    }

    pub(crate) fn enable_auto_command12_set(&mut self, enable_auto_command12: bool) {
        self.enable_auto_command12 = enable_auto_command12
    }

    pub(crate) fn enable_auto_command23(&self) -> bool {
        self.enable_auto_command23
    }

    pub(crate) fn enable_ignore_error(&self) -> bool {
        self.enable_ignore_error
    }

    pub(crate) fn data_type(&self) -> u8 {
        self.data_type
    }

    pub(crate) fn block_size(&self) -> usize {
        self.block_size
    }

    pub(crate) fn block_size_set(&mut self, block_size: usize) {
        self.block_size = block_size;
    }

    pub(crate) fn block_count(&self) -> u32 {
        self.block_count
    }

    pub(crate) fn block_count_set(&mut self, block_count: u32) {
        self.block_count = block_count;
    }

    pub(crate) fn rx_data(&self) -> Option<&Vec<u32>> {
        self.rx_data.as_ref()
    }

    pub(crate) fn rx_data_set(&mut self, rx_data: Option<Vec<u32>>) {
        self.rx_data = rx_data
    }

    pub(crate) fn rx_data_mut(&mut self) -> Option<&mut Vec<u32>> {
        self.rx_data.as_mut()
    }

    pub(crate) fn rx_data_take(&mut self) -> Option<Vec<u32>> {
        self.rx_data.take()
    }

    pub(crate) fn tx_data(&self) -> Option<&Vec<u32>> {
        self.tx_data.as_ref()
    }

    pub(crate) fn tx_data_mut(&mut self) -> Option<&mut Vec<u32>> {
        self.tx_data.as_mut()
    }

    pub(crate) fn tx_data_set(&mut self, tx_data: Option<Vec<u32>>) {
        self.tx_data = tx_data
    }

    pub(crate) fn tx_data_take(&mut self) -> Option<Vec<u32>> {
        self.tx_data.take()
    }
}

/// Host command structure.
#[allow(unused)]
pub(crate) struct MCIHostCmd {
    /// Command index
    index: u32,
    /// Command argument
    argument: u32,
    /// Command type
    cmd_type: MCIHostCmdType,
    /// Response type
    response_type: MCIHostResponseType,
    /// Response data
    response: [u32; 4],
    /// Response error flags
    response_error_flags: MCIHostCardStatusFlag,
    /// Command flags
    flags: u32,
}

#[allow(unused)]
impl MCIHostCmd {
    pub(crate) fn new() -> Self {
        MCIHostCmd {
            index: 0,
            argument: 0,
            cmd_type: MCIHostCmdType::Normal,
            response_type: MCIHostResponseType::None,
            response: [0; 4],
            response_error_flags: MCIHostCardStatusFlag::empty(),
            flags: 0,
        }
    }

    pub(crate) fn index(&self) -> u32 {
        self.index
    }

    pub(crate) fn index_set(&mut self, index: u32) {
        self.index = index
    }

    pub(crate) fn argument(&self) -> u32 {
        self.argument
    }

    pub(crate) fn argument_set(&mut self, argument: u32) {
        self.argument = argument
    }

    pub(crate) fn cmd_type(&self) -> MCIHostCmdType {
        self.cmd_type
    }

    pub(crate) fn cmd_type_set(&mut self, cmd_type: MCIHostCmdType) {
        self.cmd_type = cmd_type
    }

    pub(crate) fn response_type(&self) -> MCIHostResponseType {
        self.response_type
    }

    pub(crate) fn response_type_set(&mut self, response_type: MCIHostResponseType) {
        self.response_type = response_type
    }

    pub(crate) fn response(&self) -> &[u32; 4] {
        &self.response
    }

    pub(crate) fn response_mut(&mut self) -> &mut [u32; 4] {
        &mut self.response
    }

    pub(crate) fn response_error_flags(&self) -> MCIHostCardStatusFlag {
        self.response_error_flags
    }

    pub(crate) fn response_error_flags_set(&mut self, flags: MCIHostCardStatusFlag) {
        self.response_error_flags = flags
    }

    pub(crate) fn flags(&self) -> u32 {
        self.flags
    }
}
