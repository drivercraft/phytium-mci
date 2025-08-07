#![no_std]

#[cfg(all(feature = "dma", feature = "pio"))]
compile_error!("can't enable feature dma and pio at the same time!");
#[cfg(all(feature = "irq", feature = "poll"))]
compile_error!("can't enable feature irq and poll at the same time!");

extern crate alloc;

#[macro_use]
pub mod regs;
pub mod mci;
pub mod mci_host;
pub mod osa;

mod tools;

use log::debug;
pub use mci::mci_intr::*;
pub use mci_host::*;

use core::{
    ptr::{self, NonNull},
    time::Duration,
};

pub trait Kernel {
    fn sleep(duration: Duration);
}

pub(crate) fn mci_sleep(duration: Duration) {
    unsafe extern "Rust" {
        fn _phytium_mci_sleep(duration: Duration);
    }

    unsafe {
        _phytium_mci_sleep(duration);
    }
}

#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[unsafe(no_mangle)]
        unsafe fn _phytium_mci_sleep(duration: core::time::Duration) {
            <$t as $crate::Kernel>::sleep(duration)
        }
    };
}

pub fn dump_registers(addr: NonNull<u8>, size: usize) {
    let base_addr = addr.as_ptr() as usize;

    debug!(
        "=== Dumping registers (0x{:x} bytes from base: 0x{:p}) ===",
        size,
        addr.as_ptr()
    );

    let aligned_size = size & !0xf;

    for offset in (0..aligned_size).step_by(16) {
        if offset + 16 > size {
            break;
        }

        unsafe {
            let reg_addr = (base_addr + offset) as *const u32;

            let val0 = if offset < size {
                ptr::read_volatile(reg_addr)
            } else {
                0
            };
            let val1 = if offset + 4 < size {
                ptr::read_volatile(reg_addr.add(1))
            } else {
                0
            };
            let val2 = if offset + 8 < size {
                ptr::read_volatile(reg_addr.add(2))
            } else {
                0
            };
            let val3 = if offset + 12 < size {
                ptr::read_volatile(reg_addr.add(3))
            } else {
                0
            };

            debug!(
                "Offset 0x{:04x}: {:08x}, {:08x}, {:08x}, {:08x}",
                offset, val0, val1, val2, val3
            );
        }
    }

    debug!("=== End of register dump ===");
}
