use super::RegError;

#[derive(Debug)]
pub enum MCIError {
    Timeout,
    NotInit,
    ShortBuf,
    NotSupport,
    InvalidState,
    TransTimeout,
    CmdTimeout,
    NoCard,
    Busy,
    DmaBufUnalign,
    InvalidTiming,
    CrcError,
    InvalidParam,
    BadMalloc,
}

impl RegError for MCIError {
    fn timeout() -> Self {
        MCIError::Timeout
    }
}

pub type MCIResult<T = ()> = Result<T, MCIError>;
