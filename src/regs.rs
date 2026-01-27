//! # Register Abstraction Layer
//!
//! This module provides a type-safe abstraction layer for hardware register access.
//! It offers:
//! - Bit manipulation utilities and macros
//! - Generic register read/write operations
//! - Bitflag-based register field manipulation
//! - Polling and retry mechanisms for hardware operations
//!
//! ## Overview
//!
//! The register abstraction is built around the [`Reg`] struct, which provides
//! safe access to memory-mapped hardware registers. It uses Rust's type system
//! to ensure type safety through phantom types and traits.
//!
//! ## Example
//!
//! ```rust,ignore
//! // Define a register with flags
//! bitflags! {
//!     struct ControlReg: u32 {
//!         const ENABLE = 1 << 0;
//!         const RESET = 1 << 1;
//!     }
//! }
//!
//! impl FlagReg for ControlReg {
//!     const REG: u32 = 0x00;
//! }
//!
//! // Use the register
//! let reg = Reg::new(base_addr);
//! reg.set_reg(ControlReg::ENABLE);
//! reg.wait_for::<ControlReg, _>(|r| r.contains(ControlReg::ENABLE), timeout, None)?;
//! ```

#![allow(unused)]

use crate::sleep;
use bitflags::{Flags, bitflags};
use core::{marker::PhantomData, ops, ptr::NonNull, time::Duration};
use log::info;

/// Marker trait for types that support bitwise operations.
///
/// This trait is implemented for all types that support the standard
/// bitwise operations (OR, AND, NOT, XOR). It's used to enable generic
/// bit manipulation on flag types.
///
/// # Trait Bounds
///
/// - `BitOr<Output = Self>` - Bitwise OR operation
/// - `BitAnd<Output = Self>` - Bitwise AND operation
/// - `Not<Output = Self>` - Bitwise NOT operation
/// - `BitXor<Output = Self>` - Bitwise XOR operation
/// - `Sized` - Sized type requirement
///
/// # Example
///
/// ```rust,ignore
/// fn clear_flags<F: BitsOps + Copy>(reg: &mut F, flags: F) {
///     *reg = !flags & *reg;
/// }
/// ```
pub trait BitsOps:
    ops::BitOr<Output = Self>
    + ops::BitAnd<Output = Self>
    + ops::Not<Output = Self>
    + ops::BitXor<Output = Self>
    + Sized
{
}
impl<T> BitsOps for T where
    T: ops::BitOr<Output = Self>
        + ops::BitAnd<Output = Self>
        + ops::Not<Output = Self>
        + ops::BitXor<Output = Self>
{
}

/// Creates a contiguous bitmask starting at bit position `l` and ending at position `h`.
///
/// This macro generates a 32-bit mask with bits set from position `l` (low) to `h` (high),
/// inclusive. Both positions are zero-indexed from the least significant bit.
///
/// # Arguments
///
/// * `$h` - High bit position (inclusive, 0-31)
/// * `$l` - Low bit position (inclusive, 0-31)
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(genmask!(7, 4), 0b0000_0000_0000_0000_0000_0000_1111_0000);
/// assert_eq!(genmask!(3, 0), 0b0000_0000_0000_0000_0000_0000_0000_1111);
/// assert_eq!(genmask!(15, 0), 0x0000FFFF);
/// ```
///
/// # Notes
///
/// - The high bit `$h` must be greater than or equal to the low bit `$l`
/// - Both positions must be within 0-31 range for 32-bit masks
#[macro_export]
macro_rules! genmask {
    ($h:expr, $l:expr) => {
        (((!0u32) - (1u32 << $l) + 1) & ((!0u32) >> (32 - 1 - $h)))
    };
}

/// Creates a contiguous bitmask for 64-bit values.
///
/// This macro generates a 64-bit mask with bits set from position `l` (low) to `h` (high),
/// inclusive. Both positions are zero-indexed from the least significant bit.
///
/// # Arguments
///
/// * `$h` - High bit position (inclusive, 0-63)
/// * `$l` - Low bit position (inclusive, 0-63)
///
/// # Examples
///
/// ```rust,ignore
/// assert_eq!(genmask_ull!(39, 21), 0x000000ffffe00000);
/// assert_eq!(genmask_ull!(63, 32), 0xFFFFFFFF00000000);
/// ```
///
/// # Notes
///
/// - The high bit `$h` must be greater than or equal to the low bit `$l`
/// - Both positions must be within 0-63 range for 64-bit masks
#[macro_export]
macro_rules! genmask_ull {
    ($h:expr, $l:expr) => {
        (((!0u64) - (1u64 << $l) + 1) & ((!0u64) >> (64 - 1 - $h)))
    };
}

/// Extracts a bit field from a 32-bit register value.
///
/// Reads the bits in the range \[`a`:`b`\] from a register value, where `a` is the
/// high bit position and `b` is the low bit position. The extracted field is
/// right-aligned to bit 0.
///
/// # Arguments
///
/// * `$reg` - Register value to extract from
/// * `$a` - High bit position (inclusive)
/// * `$b` - Low bit position (inclusive)
///
/// # Examples
///
/// ```rust,ignore
/// let reg_value = 0b1011_0100u32;
/// let field = get_reg32_bits!(reg_value, 7, 4); // Extract bits 7:4
/// assert_eq!(field, 0b1011); // 0xb, right-aligned
/// ```
#[macro_export]
macro_rules! get_reg32_bits {
    ($reg:expr, $a:expr, $b:expr) => {
        ($reg & genmask!($a, $b)) >> $b
    };
}

/// Prepares a value to be written to a bit field in a 32-bit register.
///
/// Takes a value and shifts it to the correct position for bits \[`a`:`b`\] of a
/// register, where `a` is the high bit position and `b` is the low bit position.
///
/// # Arguments
///
/// * `$reg` - Value to write (will be shifted to position)
/// * `$a` - High bit position (inclusive)
/// * `$b` - Low bit position (inclusive)
///
/// # Examples
///
/// ```rust,ignore
/// let value = 0b1011u32; // Value to write
/// let prepared = set_reg32_bits!(value, 7, 4); // Prepare for bits 7:4
/// assert_eq!(prepared, 0b1011_0000u32); // Shifted to position
/// ```
#[macro_export]
macro_rules! set_reg32_bits {
    ($reg:expr, $a:expr, $b:expr) => {
        ($reg << $b) & genmask!($a, $b)
    };
}

/// Hardware register accessor providing type-safe memory-mapped I/O operations.
///
/// The [`Reg`] struct encapsulates a base address for a memory-mapped register region
/// and provides methods for reading, writing, and modifying registers. It uses phantom
/// types to encode error types at compile time.
///
/// # Type Parameters
///
/// * `E` - Error type that implements [`RegError`], used for timeout errors
///
/// # Safety
///
/// This struct performs volatile memory accesses on arbitrary memory addresses.
/// The caller must ensure:
/// - The address is valid for the hardware register being accessed
/// - The address is properly aligned for register access
/// - Accesses comply with the hardware's memory ordering requirements
///
/// # Example
///
/// ```rust,ignore
/// // Define register flags
/// bitflags! {
///     struct StatusReg: u32 {
///         const BUSY = 1 << 0;
///         const DONE = 1 << 1;
///     }
/// }
///
/// impl FlagReg for StatusReg {
///     const REG: u32 = 0x04;
/// }
///
/// // Create register accessor
/// let reg = Reg::<MyError>::new(base_addr);
///
/// // Check status
/// let status = reg.read_reg::<StatusReg>();
///
/// // Wait for completion
/// reg.wait_for::<StatusReg, _>(
///     |s| s.contains(StatusReg::DONE),
///     Duration::from_millis(100),
///     Some(10)
/// )?;
/// ```
#[derive(Debug)]
pub struct Reg<E: RegError> {
    /// Base address of the register region
    pub addr: NonNull<u8>,
    /// Phantom marker for the error type
    _marker: PhantomData<E>,
}

impl<E: RegError> Reg<E> {
    /// Creates a new register accessor from a base address.
    ///
    /// # Arguments
    ///
    /// * `addr` - Non-null pointer to the base of the register region
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let base = NonNull::new(0x2800_1000 as *mut u8).unwrap();
    /// let reg = Reg::<MyError>::new(base);
    /// ```
    pub fn new(addr: NonNull<u8>) -> Self {
        Self {
            addr,
            _marker: PhantomData,
        }
    }

    /// Performs a volatile 32-bit read from a register offset.
    ///
    /// # Arguments
    ///
    /// * `reg` - Byte offset from the base address
    ///
    /// # Returns
    ///
    /// The 32-bit value read from the register
    ///
    /// # Safety
    ///
    /// The offset must point to a valid 32-bit register within the memory region.
    pub fn read_32(&self, reg: u32) -> u32 {
        unsafe {
            let ptr = self.addr.add(reg as _);
            ptr.cast().read_volatile()
        }
    }

    /// Performs a volatile 32-bit write to a register offset.
    ///
    /// # Arguments
    ///
    /// * `reg` - Byte offset from the base address
    /// * `val` - 32-bit value to write
    ///
    /// # Safety
    ///
    /// The offset must point to a valid 32-bit register within the memory region.
    pub fn write_32(&self, reg: u32, val: u32) {
        unsafe {
            let ptr = self.addr.add(reg as _);
            ptr.cast().write_volatile(val);
        }
    }

    /// Reads a flag-based register, returning its type-safe representation.
    ///
    /// This method is used with types that implement [`FlagReg`] to read
    /// a register and interpret its bits as flag values.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Flag register type implementing [`FlagReg`]
    ///
    /// # Returns
    ///
    /// The flag value read from the register
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let status = reg.read_reg::<StatusReg>();
    /// if status.contains(StatusReg::BUSY) {
    ///     // Handle busy state
    /// }
    /// ```
    pub fn read_reg<F: FlagReg>(&self) -> F {
        F::from_bits_retain(self.read_32(F::REG))
    }

    /// Writes a flag-based register with the provided value.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Flag register type implementing [`FlagReg`]
    ///
    /// # Arguments
    ///
    /// * `val` - Flag value to write to the register
    pub fn write_reg<F: FlagReg>(&self, val: F) {
        self.write_32(F::REG, val.bits())
    }

    /// Modifies a register using a transformation function.
    ///
    /// Reads the current register value, applies the transformation function,
    /// and writes the result back. This is useful for read-modify-write operations.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Flag register type implementing [`FlagReg`]
    ///
    /// # Arguments
    ///
    /// * `f` - Function that transforms the current register value
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Set multiple flags atomically
    /// reg.modify_reg::<ControlReg>(|r| r | ControlReg::ENABLE | ControlReg::RESET);
    /// ```
    pub fn modify_reg<F: FlagReg>(&self, f: impl Fn(F) -> F) {
        let old = self.read_reg::<F>();
        self.write_reg(f(old));
    }

    /// Clears specific flags in a register.
    ///
    /// Performs a read-modify-write operation that clears the specified flags
    /// while preserving all other bits.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Flag register type implementing [`FlagReg`], [`Copy`], and [`BitsOps`]
    ///
    /// # Arguments
    ///
    /// * `val` - Flags to clear
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// reg.clear_reg::<StatusReg>(StatusReg::BUSY);
    /// ```
    pub fn clear_reg<F: FlagReg + Copy + BitsOps>(&self, val: F) {
        self.modify_reg(|old| !val & old)
    }

    /// Sets specific flags in a register.
    ///
    /// Performs a read-modify-write operation that sets the specified flags
    /// while preserving all other bits.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Flag register type implementing [`FlagReg`], [`Copy`], and [`BitsOps`]
    ///
    /// # Arguments
    ///
    /// * `val` - Flags to set
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// reg.set_reg::<ControlReg>(ControlReg::ENABLE);
    /// ```
    pub fn set_reg<F: FlagReg + Copy + BitsOps>(&self, val: F) {
        self.modify_reg(|old| val | old)
    }

    /// Returns the base address of this register region.
    ///
    /// # Returns
    ///
    /// The base address as a [`NonNull<u8>`]
    pub fn get_base_addr(&self) -> NonNull<u8> {
        self.addr
    }

    /// Polls a register until a condition is met or a timeout occurs.
    ///
    /// This method repeatedly checks a register at the specified interval until
    /// the provided predicate returns `true` or the maximum number of attempts
    /// is exceeded.
    ///
    /// # Type Parameters
    ///
    /// * `R` - Flag register type implementing [`FlagReg`]
    /// * `F` - Predicate function type
    ///
    /// # Arguments
    ///
    /// * `f` - Predicate function that returns `true` when the condition is met
    /// * `interval` - Duration to sleep between attempts
    /// * `try_count` - Maximum number of attempts (`None` for unlimited)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the condition was met
    /// * `Err(E)` - If the timeout was exceeded
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// reg.wait_for::<StatusReg, _>(
    ///     |s| !s.contains(StatusReg::BUSY),
    ///     Duration::from_millis(10),
    ///     Some(100)
    /// )?;
    /// ```
    pub fn wait_for<R: FlagReg, F: Fn(R) -> bool>(
        &self,
        f: F,
        interval: Duration,
        try_count: Option<usize>,
    ) -> Result<(), E> {
        for _ in 0..try_count.unwrap_or(usize::MAX) {
            if f(self.read_reg::<R>()) {
                return Ok(());
            }

            sleep(interval);
        }
        Err(E::timeout())
    }

    /// Retries a register check without delay between attempts.
    ///
    /// Similar to [`wait_for`] but without any delay between attempts. This is
    /// useful for spin-waiting on fast operations.
    ///
    /// # Type Parameters
    ///
    /// * `R` - Flag register type implementing [`FlagReg`]
    /// * `F` - Predicate function type
    ///
    /// # Arguments
    ///
    /// * `f` - Predicate function that returns `true` when the condition is met
    /// * `try_count` - Maximum number of attempts (`None` for unlimited)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the condition was met
    /// * `Err(E)` - If the timeout was exceeded
    pub fn retry_for<R: FlagReg, F: Fn(R) -> bool>(
        &self,
        f: F,
        try_count: Option<usize>,
    ) -> Result<(), E> {
        for _ in 0..try_count.unwrap_or(usize::MAX) {
            if f(self.read_reg::<R>()) {
                return Ok(());
            }
        }
        Err(E::timeout())
    }
}

impl<E: RegError> PartialEq for Reg<E> {
    /// Checks equality based on the base address.
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

/// Trait for error types used with [`Reg`].
///
/// This trait must be implemented by error types used with the [`Reg`] struct.
/// It provides a way to create timeout errors from polling operations.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug)]
/// enum MyError {
///     Timeout,
/// }
///
/// impl RegError for MyError {
///     fn timeout() -> Self {
///         Self::Timeout
///     }
/// }
/// ```
pub trait RegError {
    /// Creates a timeout error instance.
    ///
    /// This method is called by [`Reg::wait_for`] and [`Reg::retry_for`]
    /// when the maximum number of attempts is exceeded.
    ///
    /// # Returns
    ///
    /// An error instance representing a timeout condition
    fn timeout() -> Self;
}

/// Trait for flag-based register types.
///
/// This trait combines a bitflags type with its register offset, enabling
/// type-safe register access through the [`Reg`] struct.
///
/// # Requirements
///
/// Types implementing this trait must:
/// - Be bitflags generated by the `bitflags!` macro with `u32` as the underlying type
/// - Have an associated constant `REG` specifying the byte offset from the base address
///
/// # Example
///
/// ```rust,ignore
/// use bitflags::bitflags;
///
/// bitflags! {
///     struct ControlReg: u32 {
///         const ENABLE = 1 << 0;
///         const RESET  = 1 << 1;
///     }
/// }
///
/// impl FlagReg for ControlReg {
///     const REG: u32 = 0x00; // Offset 0x00 from base
/// }
///
/// // Usage:
/// let reg = Reg::<MyError>::new(base);
/// reg.set_reg::<ControlReg>(ControlReg::ENABLE);
/// ```
pub trait FlagReg: Flags<Bits = u32> {
    /// The byte offset of this register from the base address.
    const REG: u32;
}

/// Implements bitwise operations between a flag type and `u32`.
///
/// This macro generates trait implementations that allow flag types (typically
/// generated by `bitflags!`) to interact directly with `u32` values through
/// bitwise operations like OR and AND.
///
/// This is useful when working with register values that may come from or
/// need to be converted to raw `u32` values.
///
/// # Arguments
///
/// * `$name` - The identifier of the flag type to implement traits for
///
/// # Implemented Traits
///
/// - `BitOr<u32>` - Bitwise OR with `u32`, returns the flag type
/// - `BitAnd<u32>` - Bitwise AND with `u32`, returns the flag type
/// - `From<u32>` - Conversion from `u32` to the flag type
///
/// # Example
///
/// ```rust,ignore
/// bitflags! {
///     struct StatusReg: u32 {
///         const BUSY = 1 << 0;
///         const DONE = 1 << 1;
///     }
/// }
///
/// BitsOpsForU32!(StatusReg);
///
/// // Now you can use u32 values directly:
/// let status = StatusReg::BUSY;
/// let combined = status | 0xFFu32; // OR with raw u32
/// let masked = status & 0x01u32;   // AND with raw u32
/// let from_raw: StatusReg = 0xFFu32.into(); // Convert from u32
/// ```
#[macro_export]
macro_rules! BitsOpsForU32 {
    ($name:ident) => {
        /// Bitwise OR operation with `u32`.
        ///
        /// Allows combining flag values with raw `u32` values.
        impl ops::BitOr<u32> for $name {
            type Output = Self;
            fn bitor(self, rhs: u32) -> Self {
                self | Self::from_bits_truncate(rhs)
            }
        }

        /// Bitwise AND operation with `u32`.
        ///
        /// Allows masking flag values with raw `u32` values.
        impl ops::BitAnd<u32> for $name {
            type Output = Self;
            fn bitand(self, rhs: u32) -> Self {
                self & Self::from_bits_truncate(rhs)
            }
        }

        /// Conversion from `u32` to the flag type.
        ///
        /// Creates a flag value from a raw `u32`, truncating to valid bits.
        impl From<u32> for $name {
            fn from(val: u32) -> Self {
                Self::from_bits_truncate(val)
            }
        }
    };
}
