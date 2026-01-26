//! Error types for MCI host operations
//!
//! This module defines comprehensive error types for SD/MMC host operations,
//! covering command failures, data transfer errors, and card-specific issues.

#[allow(unused)]
/// Errors that can occur during MCI host operations
///
/// These errors cover all aspects of SD/MMC card operations including
/// command execution, data transfer, card initialization, and configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCIHostError {
    /// General operation failure
    Fail,
    /// Read-only operation attempted on read-only media
    ReadOnly,
    /// Parameter out of valid range
    OutOfRange,
    /// Invalid argument provided
    InvalidArgument,
    /// Operation timed out
    Timeout,
    /// No transfer in progress to wait for
    NoTransferInProgress,
    /// Device busy
    Busy,
    /// No data available
    NoData,
    /// Feature not yet supported
    NotSupportYet,
    /// Command/data transfer failed
    TransferFailed,
    /// Setting card block size failed
    SetCardBlockSizeFailed,
    /// Host doesn't support this operation
    HostNotSupport,
    /// Card doesn't support this operation
    CardNotSupport,
    /// All Send CID command failed
    AllSendCidFailed,
    /// Send Relative Address command failed
    SendRelativeAddressFailed,
    /// Send CSD command failed
    SendCsdFailed,
    /// Select Card command failed
    SelectCardFailed,
    /// Send SCR command failed
    SendScrFailed,
    /// Set data bus width failed
    SetDataBusWidthFailed,
    /// Go Idle State command failed
    GoIdleFailed,
    /// Handshake Operation Condition failed
    HandShakeOperationConditionFailed,
    /// Send application command failed
    SendApplicationCommandFailed,
    /// Switch function command failed
    SwitchFailed,
    /// Stop Transmission command failed
    StopTransmissionFailed,
    /// Wait for write complete failed
    WaitWriteCompleteFailed,
    /// Set block count failed
    SetBlockCountFailed,
    /// Set relative address failed
    SetRelativeAddressFailed,
    /// Switch to high-speed timing failed
    SwitchBusTimingFailed,
    /// Send Extended CSD failed
    SendExtendedCsdFailed,
    /// Configure boot partition failed
    ConfigureBootFailed,
    /// Configure Extended CSD failed
    ConfigureExtendedCsdFailed,
    /// Enable high capacity erase failed
    EnableHighCapacityEraseFailed,
    /// Send test pattern failed
    SendTestPatternFailed,
    /// Receive test pattern failed
    ReceiveTestPatternFailed,
    /// SDIO response error
    SdioResponseError,
    /// SDIO invalid argument response error
    SdioInvalidArgument,
    /// SDIO send operation condition failed
    SdioSendOperationConditionFail,
    /// Invalid voltage for operation
    InvalidVoltage,
    /// SDIO switch to high speed failed
    SdioSwitchHighSpeedFail,
    /// SDIO read CIS failed
    SdioReadCISFail,
    /// Invalid SDIO card
    SdioInvalidCard,
    /// HS200/HS400 tuning failed
    TuningFail,
    /// Voltage switch failed
    SwitchVoltageFail,
    /// Voltage switch to 1.8V failed, 3.3V succeeded
    SwitchVoltage18VFail33VSuccess,
    /// Retuning requested by card
    ReTuningRequest,
    /// Set driver strength failed
    SetDriverStrengthFail,
    /// Set power class failed
    SetPowerClassFail,
    /// Host controller not ready
    HostNotReady,
    /// Card detection failed
    CardDetectFailed,
    /// AU size not properly configured
    AuSizeNotSetProperly,
    /// Polling for card idle status failed
    PollingCardIdleFailed,
    /// Deselect card failed
    DeselectCardFailed,
    /// Card status is idle
    CardStatusIdle,
    /// Card status is busy
    CardStatusBusy,
    /// Card initialization failed
    CardInitFailed,
    /// IRQ initialization failed
    IrqInitFailed,
}

/// Result type for MCI host operations
pub type MCIHostStatus<T = ()> = Result<T, MCIHostError>;
