//! ARMv8-A architecture-specific functions.
//!
//! This module provides low-level ARM architecture synchronization and
//! barrier instructions for cache coherency and memory ordering.

#![allow(dead_code)]

/// Data Synchronization Barrier.
///
/// Ensures that all explicit memory accesses before the DSB instruction
/// complete before any explicit memory accesses after the DSB instruction.
/// This is essential for cache maintenance operations and ensuring
/// memory ordering.
///
/// # Safety
///
/// This function uses inline assembly and should only be called when
/// direct hardware access is required. The function combines both DSB
/// (Data Synchronization Barrier) and ISB (Instruction Synchronization
/// Barrier) to ensure full synchronization.
///
/// # Example
///
/// ```rust,ignore
/// unsafe { dsb() };
/// ```
#[inline(always)]
pub unsafe fn dsb() {
    unsafe {
        core::arch::asm!("dsb sy");
        core::arch::asm!("isb sy");
    }
}

/// Instruction Synchronization Barrier.
///
/// Ensures that all instructions in the pipeline before the ISB
/// instruction complete before fetching any subsequent instructions.
/// This is used to ensure instruction stream consistency after
/// modifying system registers or changing memory mappings.
///
/// # Safety
///
/// This function uses inline assembly and should only be called when
/// direct hardware access is required.
///
/// # Example
///
/// ```rust,ignore
/// unsafe { isb() };
/// ```
#[inline(always)]
pub unsafe fn isb() {
    unsafe {
        core::arch::asm!("isb", options(nostack, preserves_flags));
    }
}
