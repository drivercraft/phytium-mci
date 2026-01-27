/// Swaps the two 16-bit halfwords in a 32-bit value.
///
/// This function exchanges the lower 16 bits with the upper 16 bits.
///
/// # Arguments
///
/// * `value` - The 32-bit value to swap
///
/// # Returns
///
/// The value with halfwords swapped
///
/// # Example
///
/// ```rust,ignore
/// let val = 0x12345678u32;
/// let swapped = swap_half_word_byte_sequence_u32(val);
/// assert_eq!(swapped, 0x56781234);
/// ```
pub fn swap_half_word_byte_sequence_u32(value: u32) -> u32 {
    ((value & 0x0000FFFF) << 16) | ((value & 0xFFFF0000) >> 16)
}

/// Reverses the byte order of a 32-bit value.
///
/// This function performs a big-endian to little-endian conversion (or vice versa)
/// by reversing the order of all 4 bytes in the value.
///
/// # Arguments
///
/// * `value` - The 32-bit value to reverse
///
/// # Returns
///
/// The value with bytes in reverse order
///
/// # Example
///
/// ```rust,ignore
/// let val = 0x12345678u32;
/// let reversed = swap_word_byte_sequence_u32(val);
/// assert_eq!(reversed, 0x78563412);
/// ```
pub fn swap_word_byte_sequence_u32(value: u32) -> u32 {
    ((value & 0x000000FF) << 24)
        | ((value & 0x0000FF00) << 8)
        | ((value & 0x00FF0000) >> 8)
        | ((value & 0xFF000000) >> 24)
}

// pub fn u8_to_u32_slice(bytes: &Vec<u8>) -> Vec<u32> {
//     assert!(bytes.len() % 4 == 0, "Byte array length must be a multiple of 4");

//     let mut result = Vec::with_capacity(bytes.len() / 4);

//     // Process 4 bytes at a time
//     for chunk in bytes.chunks_exact(4) {
//         let value = u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
//         result.push(value);
//     }

//     result
// }
