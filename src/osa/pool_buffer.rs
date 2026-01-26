//! A managed memory buffer for aligned allocations.
//!
//! Provides [`PoolBuffer`] - a safe wrapper around pooled memory allocations
//! from the global TLSF memory pool. This is primarily used for DMA buffers
//! that require specific alignment constraints.
//!
//! # Example
//!
//! ```rust,ignore
//! use phytium_mci::osa::PoolBuffer;
//!
//! // Allocate a 4KB buffer with 512-byte alignment
//! let buffer = PoolBuffer::new(4096, 512)?;
//!
//! // Clear the buffer
//! buffer.clear();
//!
//! // Convert to Vec
//! let data: Vec<u32> = buffer.to_vec()?;
//! ```
use core::{
    ptr::{NonNull, copy_nonoverlapping, write_bytes},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use alloc::vec::Vec;
use log::error;

use super::{err::FMempError, osa_alloc_aligned, osa_dealloc};

/// Managed memory buffer with alignment support.
///
/// `PoolBuffer` provides a safe wrapper for memory allocated from the global
/// TLSF memory pool. It automatically frees the memory when dropped and
/// provides convenient methods for data access.
///
/// # Memory Pool
///
/// Buffers are allocated from the global TLSF memory pool, which must be
/// initialized with [`osa_init`](super::osa_init) before use.
///
/// # Alignment
///
/// The buffer supports custom alignment requirements, which is essential for
/// DMA operations that require specific address alignment.
///
/// # Example
///
/// ```rust,ignore
/// use phytium_mci::osa::PoolBuffer;
///
/// // Allocate a 4KB buffer with 512-byte alignment
/// let buffer = PoolBuffer::new(4096, 512)?;
///
/// // Clear the buffer
/// buffer.clear();
///
/// // Convert to Vec<u32>
/// let data: Vec<u32> = buffer.to_vec()?;
/// ```
pub struct PoolBuffer {
    size: usize,
    addr: NonNull<u8>,
    align: usize,
}

impl PoolBuffer {
    /// Allocates a new buffer from the memory pool.
    ///
    /// # Arguments
    ///
    /// * `size` - Buffer size in bytes
    /// * `align` - Alignment requirement in bytes (must be power of 2)
    ///
    /// # Returns
    ///
    /// A new `PoolBuffer` instance, or an error if allocation failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let buffer = PoolBuffer::new(4096, 512)?;
    /// ```
    pub fn new(size: usize, align: usize) -> Result<Self, &'static str> {
        let ptr = match osa_alloc_aligned(size, align) {
            Err(_) => return Err("osa alloc failed!"),
            Ok(ptr) => ptr,
        };
        Ok(Self {
            size,
            addr: ptr,
            align,
        })
    }

    /// Copies data from a slice into the buffer.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of elements to copy (must implement `Copy`)
    ///
    /// # Arguments
    ///
    /// * `src` - Source slice to copy from
    pub fn copy_from_slice<T: Copy>(&mut self, src: &[T]) -> Result<(), &'static str> {
        let len = size_of_val(src);
        if self.size < len {
            return Err("Too small to receive data!");
        }

        unsafe {
            // equivalent to memcpy in C
            copy_nonoverlapping(src.as_ptr() as *mut u8, self.addr.as_ptr(), len);
        }

        Ok(())
    }

    /// Returns a slice view of the buffer.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of elements in the slice
    ///
    /// # Returns
    ///
    /// A slice reference to the buffer contents
    pub fn as_slice<T>(&self) -> Result<&[T], FMempError> {
        let size = size_of::<T>();
        if !self.size().is_multiple_of(size) {
            return Err(FMempError::SizeNotAligned);
        }

        unsafe {
            let result = from_raw_parts(self.addr.as_ptr() as *const T, self.size() / size);
            Ok(result)
        }
    }

    /// Returns a slice view of the buffer with specified length.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of elements in the slice
    ///
    /// # Arguments
    ///
    /// * `len` - Number of elements to include in the slice
    pub fn as_slice_in_len<T>(&self, len: usize) -> Result<&[T], FMempError> {
        if len * size_of::<T>() > self.size {
            error!("Acquiring length to big for this PoolBuffer");
            return Err(FMempError::NotEnoughSpace);
        }

        unsafe {
            let result = from_raw_parts(self.addr.as_ptr() as *const T, len);
            Ok(result)
        }
    }

    /// Returns a mutable slice view of the buffer.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of elements in the slice
    ///
    /// # Returns
    ///
    /// A mutable slice reference to the buffer contents
    pub fn as_slice_mut<T>(&self) -> Result<&[T], FMempError> {
        let size = size_of::<T>();
        if !self.size().is_multiple_of(size) {
            return Err(FMempError::SizeNotAligned);
        }

        unsafe {
            let result = from_raw_parts_mut(self.addr.as_ptr() as *mut T, self.size() / size);
            Ok(result)
        }
    }

    /// Converts the buffer contents to a `Vec`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of elements (must implement `Clone`)
    pub fn to_vec<T: Clone>(&self) -> Result<Vec<T>, FMempError> {
        let slice = self.as_slice::<T>()?;
        Ok(slice.to_vec())
    }

    /// Converts the buffer contents to a `Vec` with specified length.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type of elements (must implement `Clone`)
    ///
    /// # Arguments
    ///
    /// * `len` - Number of elements to include in the vector
    pub fn to_vec_in_len<T: Clone>(&self, len: usize) -> Result<Vec<T>, FMempError> {
        let slice = self.as_slice_in_len::<T>(len)?;
        Ok(slice.to_vec())
    }

    /// Clears the buffer contents by writing zeros.
    ///
    /// This fills the entire buffer with zero bytes.
    pub fn clear(&mut self) {
        unsafe {
            write_bytes(self.addr.as_ptr(), 0, self.size);
        }
    }

    /// Returns the buffer size in bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the buffer address as a `NonNull` pointer.
    pub fn addr(&self) -> NonNull<u8> {
        self.addr
    }
}

impl Drop for PoolBuffer {
    /// Automatically deallocates the buffer when dropped.
    ///
    /// This ensures that the memory is returned to the global pool
    /// when the `PoolBuffer` goes out of scope.
    fn drop(&mut self) {
        osa_dealloc(self.addr, self.align);
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec<u32>> for PoolBuffer {
    /// Converts the buffer into a `Vec<u32>`.
    ///
    /// # Panics
    ///
    /// Panics if the buffer size is not a multiple of 4 bytes.
    fn into(self) -> Vec<u32> {
        unsafe {
            let slice = from_raw_parts(self.addr.as_ptr() as *const u32, self.size / 4);
            slice.to_vec()
        }
    }
}
