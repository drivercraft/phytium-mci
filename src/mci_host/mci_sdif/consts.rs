//! Constants for SDIF device operations
//!
//! This module defines constants and enumerations used by the SDIF
//! device implementation.

/// SD card insertion status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SDStatus {
    /// Card removed from slot
    Removed = 0,
    /// Card inserted in slot
    Inserted = 1,
}
