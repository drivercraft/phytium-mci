use core::alloc::Layout;

use alloc::vec::Vec;
use log::{debug, info};

pub fn swap_half_word_byte_sequence_u32(value: u32) -> u32 {
    // Swap each 16-bit halfword
    ((value & 0x0000FFFF) << 16) | ((value & 0xFFFF0000) >> 16)
}

pub fn swap_word_byte_sequence_u32(value: u32) -> u32 {
    ((value & 0x000000FF) << 24)
        | ((value & 0x0000FF00) << 8)
        | ((value & 0x00FF0000) >> 8)
        | ((value & 0xFF000000) >> 24)
}

pub fn realign_vec(input: Vec<u32>, align: usize) -> Result<Vec<u32>, &'static str> {
    let element_count = input.len();
    let size_bytes = element_count * core::mem::size_of::<u32>();

    debug!(
        "Realigning Vec<u32>: elements={}, size_bytes={}, old_addr={:p}",
        element_count,
        size_bytes,
        input.as_ptr()
    );

    let old_addr = input.as_ptr() as usize;
    if old_addr & (align - 1) == 0 {
        debug!("Vec<u32> already {} byte aligned!", align);
        return Ok(input);
    }

    info!(
        "Realigning Vec<u32>: old_addr={:#x}, need {}-byte alignment",
        old_addr, align
    );

    // Only align the address, keep the original size
    let layout =
        Layout::from_size_align(size_bytes, align).map_err(|_| "Failed to create memory layout")?;

    let aligned_ptr = unsafe {
        let raw_ptr = alloc::alloc::alloc(layout);
        if raw_ptr.is_null() {
            return Err("Failed to allocate aligned memory");
        }
        raw_ptr as *mut u32
    };

    debug_assert_eq!(
        aligned_ptr as usize & (align - 1),
        0,
        "Allocated memory not {align} byte aligned"
    );

    // Copy original data, keep original size
    unsafe {
        core::ptr::copy_nonoverlapping(input.as_ptr(), aligned_ptr, element_count);
    }

    // Use the original element count when creating Vec
    let aligned_vec = unsafe { Vec::from_raw_parts(aligned_ptr, element_count, element_count) };

    debug!(
        "Realigned Vec<u32>: new_addr={:#x}, elements={}, capacity={}",
        aligned_ptr as usize, element_count, element_count
    );

    // drop(input);

    Ok(aligned_vec)
}

pub fn realign_vec_inplace(input: &mut Vec<u32>, align: usize) -> Result<(), &'static str> {
    let old_addr = input.as_ptr() as usize;

    if old_addr & (align - 1) == 0 {
        debug!("Vec<u32> already aligned at {:#x}", old_addr);
        return Ok(());
    }

    debug!(
        "Realigning Vec<u32> inplace: old_addr={:#x}, len={}",
        old_addr,
        input.len()
    );

    let element_count = input.len();
    let size_bytes = element_count * core::mem::size_of::<u32>();
    let aligned_size_bytes = (size_bytes + 63) & !63;
    let aligned_element_count = aligned_size_bytes / core::mem::size_of::<u32>();

    let layout = Layout::from_size_align(aligned_size_bytes, align)
        .map_err(|_| "Failed to create memory layout")?;

    let new_ptr = unsafe {
        let raw_ptr = alloc::alloc::alloc(layout);
        if raw_ptr.is_null() {
            return Err("Failed to allocate aligned memory");
        }
        raw_ptr as *mut u32
    };

    unsafe {
        core::ptr::copy_nonoverlapping(input.as_ptr(), new_ptr, element_count);
    }

    let old_ptr = input.as_mut_ptr();
    let old_len = input.len();
    let old_capacity = input.capacity();

    let new_vec = unsafe { Vec::from_raw_parts(new_ptr, element_count, aligned_element_count) };

    // Free old memory
    unsafe {
        let _old_vec = Vec::from_raw_parts(old_ptr, old_len, old_capacity);
    }

    *input = new_vec;

    debug!(
        "Vec<u32> realigned inplace: new_addr={:#x}",
        input.as_ptr() as usize
    );

    Ok(())
}
