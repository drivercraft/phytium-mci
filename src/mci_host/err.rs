//! # MCI Host Error Types
//!
//! This module defines error types for MCI host controller operations.

/// MCI host controller error enumeration.
///
/// This enum represents various error conditions that can occur during
/// SD/MMC card operations.
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCIHostError {
    /// General failure
    Fail,
    /// Read-only operation
    ReadOnly,
    /// Out of range
    OutOfRange,
    /// Invalid argument
    InvalidArgument,
    /// Operation timeout
    Timeout,
    /// No transfer in progress
    NoTransferInProgress,
    /// Controller busy
    Busy,
    /// No data available
    NoData,
    // SD/MMC card API's running status.
    /// Feature not yet supported
    NotSupportYet,
    /// Send command failed
    TransferFailed,
    /// Set block size failed
    SetCardBlockSizeFailed,
    /// Host doesn't support feature
    HostNotSupport,
    /// Card doesn't support feature
    CardNotSupport,
    /// Send CID failed
    AllSendCidFailed,
    /// Send relative address failed
    SendRelativeAddressFailed,
    /// Send CSD failed
    SendCsdFailed,
    /// Select card failed
    SelectCardFailed,
    /// Send SCR failed
    SendScrFailed,
    /// Set bus width failed
    SetDataBusWidthFailed,
    /// Go idle failed
    GoIdleFailed,
    /// Send Operation Condition failed
    HandShakeOperationConditionFailed,
    /// Send application command failed
    SendApplicationCommandFailed,
    /// Switch command failed
    SwitchFailed,
    /// Stop transmission failed
    StopTransmissionFailed,
    /// Wait write complete failed
    WaitWriteCompleteFailed,
    /// Set block count failed
    SetBlockCountFailed,
    /// Set relative address failed
    SetRelativeAddressFailed,
    /// Switch high speed failed
    SwitchBusTimingFailed,
    /// Send EXT_CSD failed
    SendExtendedCsdFailed,
    /// Configure boot failed
    ConfigureBootFailed,
    /// Configure EXT_CSD failed
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
    /// SDIO send operation condition fail
    SdioSendOperationConditionFail,
    /// Invalid voltage
    InvalidVoltage,
    /// SDIO switch to high speed fail
    SdioSwitchHighSpeedFail,
    /// SDIO read CIS fail
    SdioReadCISFail,
    /// Invalid SDIO card
    SdioInvalidCard,
    /// Tuning failed
    TuningFail,
    /// Switch voltage failed
    SwitchVoltageFail,
    /// Switch to 1.8V failed, card remains at 3.3V
    SwitchVoltage18VFail33VSuccess,
    /// Re-tuning requested by card
    ReTuningRequest,
    /// Set driver strength failed
    SetDriverStrengthFail,
    /// Set power class failed
    SetPowerClassFail,
    /// Host controller not ready
    HostNotReady,
    /// Card detect failed
    CardDetectFailed,
    /// AU size not set properly
    AuSizeNotSetProperly,
    /// Polling card idle status failed
    PollingCardIdleFailed,
    /// Deselect card failed
    DeselectCardFailed,
    /// Card is idle
    CardStatusIdle,
    /// Card is busy
    CardStatusBusy,
    /// Card initialization failed
    CardInitFailed,
}

pub type MCIHostStatus<T = ()> = Result<T, MCIHostError>;
