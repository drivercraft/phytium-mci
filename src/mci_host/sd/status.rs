#[derive(Debug, Default)]
pub(crate) struct SdStatus {
    // Current bus width
    bus_width: u8, // Note: Fixed typo in field name from "witdh" to "width"
    // Secured mode status
    secure_mode: u8,
    // SD card type
    card_type: u16,
    // Size of protected area
    protected_size: u32,
    // Speed class of card
    speed_class: u8,
    // Performance of move indicated by 1[MB/S] step
    performance_move: u8,
    // Size of allocation unit (AU)
    au_size: u8,
    // Number of AUs to be erased at a time
    erase_size: u16,
    // Timeout value for erasing areas specified by UNIT OF ERASE AU
    erase_timeout: u8,
    // Fixed offset value added to erase time
    erase_offset: u8,
    // Speed grade for UHS mode
    uhs_speed_grade: u8,
    // Size of allocation unit (AU) for UHS mode
    uhs_au_size: u8,
}

impl SdStatus {
    pub fn new() -> Self {
        SdStatus {
            bus_width: 0,
            secure_mode: 0,
            card_type: 0,
            protected_size: 0,
            speed_class: 0,
            performance_move: 0,
            au_size: 0,
            erase_size: 0,
            erase_timeout: 0,
            erase_offset: 0,
            uhs_speed_grade: 0,
            uhs_au_size: 0,
        }
    }
}
