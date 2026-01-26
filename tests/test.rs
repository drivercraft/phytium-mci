//! # Phytium MCI Driver Bare-Metal Tests
//!
//! This module contains bare-metal integration tests for the phytium-mci driver.
//! These tests run on actual Phytium Pi hardware and verify SD card functionality
//! including initialization, block writes, and block reads with data verification.
//!
//! # Test Requirements
//!
//! - **Hardware**: Phytium Pi development board with E2000 series SoC
//! - **Software**: bare_test framework for bare-metal testing
//! - **Firmware**: U-Boot with TFTP support for loading test binaries
//! - **Configuration**: Device tree (phytium.dtb) with MCI and IOPAD nodes
//!
//! # Test Flow
//!
//! 1. Parse device tree to get MCI and IOPAD register addresses
//! 2. Clear any pending interrupts from previous runs
//! 3. Initialize global register base for interrupt handling
//! 4. Optionally register interrupt handler (with `irq` feature)
//! 5. Initialize SD card and IOPAD
//! 6. Write test pattern to SD card
//! 7. Read back the data and verify integrity
//!
//! # Memory Layout
//!
//! - `SD_START_BLOCK`: 131072 (128KB offset - avoids boot/partition data)
//! - `SD_USE_BLOCK`: 4 blocks (2KB total)
//! - `SD_BLOCK_SIZE`: 512 bytes (standard SD block size)
//! - `SD_MAX_RW_BLK`: 1024 blocks maximum per transfer

#![no_std]
#![no_main]
#![feature(used_with_arg)]
#![feature(stdarch_arm_barrier)]

extern crate alloc;

#[bare_test::tests]
mod tests {
    use core::{
        arch::{
            aarch64::{__dsb, __isb, SY},
            asm,
        },
        ptr::NonNull,
        time::Duration,
    };

    use alloc::vec::Vec;
    #[cfg(feature = "dma")]
    use bare_test::mem::{PhysAddr, VirtAddr};
    use bare_test::{
        GetIrqConfig,
        globals::{PlatformInfoKind, global_val},
        irq::{IrqHandleResult, IrqParam},
        mem::iomap,
        platform::CacheOp,
        time::spin_delay,
    };
    use log::*;
    use phytium_mci::{
        IoPad, Kernel, PAD_ADDRESS,
        mci::{
            fsdif_interrupt_handler,
            regs::{MCICtrl, MCIDMACStatus, MCIIntMask, MCIRawInts, MCIReg},
        },
        sd::{SdCard, init_reg_base},
        set_impl,
    };

    /// Starting block number for test operations.
    ///
    /// This offset (128KB) is chosen to avoid interfering with potential
    /// boot sectors or partition tables at the beginning of the SD card.
    const SD_START_BLOCK: u32 = 131072;

    /// Number of blocks to use for the test.
    ///
    /// Four blocks provides 2KB of test data while keeping test duration short.
    const SD_USE_BLOCK: u32 = 4;

    /// Standard SD card block size in bytes.
    ///
    /// All SD cards use 512-byte blocks for data transfers.
    const SD_BLOCK_SIZE: u32 = 512;

    /// Maximum number of blocks per read/write operation.
    ///
    /// This limit is based on the hardware controller's FIFO depth and DMA capabilities.
    const SD_MAX_RW_BLK: u32 = 1024;

    /// Main SD card functionality test.
    ///
    /// This test performs a complete write-read-verify cycle:
    ///
    /// 1. Parses the device tree to locate MCI controller registers
    /// 2. Maps the physical registers to virtual memory address space
    /// 3. Clears any pending interrupts from previous driver instances
    /// 4. Initializes the global register base (required for interrupt handling)
    /// 5. Optionally registers an interrupt handler when `irq` feature is enabled
    /// 6. Initializes the IOPAD for pin configuration
    /// 7. Initializes the SD card and queries its capabilities
    /// 8. Writes a sequential pattern test data to the card
    /// 9. Reads back the data and verifies integrity
    ///
    /// # Panics
    ///
    /// - If SD card initialization fails
    /// - If write or read operations fail
    /// - If data verification fails (mismatch between written and read data)
    ///
    /// # Example Output
    ///
    /// ```text
    /// mci0 reg: 0x28001000, mci0 reg size: 0x1000
    /// registered irq 123 for "mci0", irq_parent: GIC, trigger: LevelHigh
    /// Card initialized!
    /// Block size: 512 bytes
    /// Total blocks: xxxxx
    /// Total capacity: xxx MB
    /// receive buffer len is 512
    /// test_work passed
    /// ```
    #[test]
    fn test_work() {
        // if cfg!(feature = "irq") {
        //     compile_error!("feature irq isn't finished yet!");
        // }

        // Extract flattened device tree (FDT) from platform info
        let fdt = match &global_val().platform_info {
            PlatformInfoKind::DeviceTree(fdt) => fdt.get(),
            // _ => panic!("unsupported platform"),
        };

        // Find MCI controller node in device tree by compatible string
        let mci0 = fdt.find_compatible(&["phytium,mci"]).next().unwrap();
        let reg = mci0.reg().unwrap().next().unwrap();
        info!(
            "mci0 reg: {:#x},mci0 reg size: {:#x}",
            reg.address,
            reg.size.unwrap()
        );

        // Map physical MCI registers to virtual address space
        let mci_reg_base = iomap((reg.address as usize).into(), reg.size.unwrap());

        // Clear any pending interrupts from previous runs
        clear_pending_irq(mci_reg_base);

        // Initialize global register base for interrupt handling
        // This MUST be called before registering interrupt handlers
        init_reg_base(mci_reg_base);

        // Map IOPAD registers for pin configuration
        let iopad_reg_base = iomap((PAD_ADDRESS as usize).into(), 0x2000);
        let iopad = IoPad::new(iopad_reg_base);

        // Register interrupt handler if IRQ feature is enabled
        if cfg!(feature = "irq") {
            let irq_info = mci0.irq_info().unwrap();
            IrqParam {
                intc: irq_info.irq_parent,
                cfg: irq_info.cfgs[0].clone(),
            }
            .register_builder(|_irq_num| {
                fsdif_interrupt_handler();
                IrqHandleResult::Handled
            })
            .register();
            info!(
                "registered irq {:?} for {:?}, irq_parent: {:?}, trigger: {:?}",
                irq_info.cfgs[0].irq,
                mci0.name(),
                irq_info.irq_parent,
                irq_info.cfgs[0].trigger
            );
        }

        // Create SD card instance with MCI registers and IOPAD
        let mut sdcard = SdCard::new(mci_reg_base, iopad);
        if let Err(err) = sdcard.init(mci_reg_base) {
            error!("Sd Card Init Fail, error = {:?}", err);
            panic!();
        }

        ////////////////////// SD card init finished //////////////////////

        // Initialize write buffer with sequential pattern (0, 1, 2, 3, ...)
        // Buffer size is calculated for maximum transfer size
        // Divided by 4 because we use u32 elements (4 bytes each)
        let mut buffer: Vec<u32> = Vec::with_capacity((SD_BLOCK_SIZE * SD_MAX_RW_BLK / 4) as usize);
        buffer.resize((SD_BLOCK_SIZE * SD_MAX_RW_BLK / 4) as usize, 0);
        for i in 0..buffer.len() {
            buffer[i] = i as u32;
        }

        // Write the test pattern to SD card
        sdcard
            .write_blocks(&mut buffer, SD_START_BLOCK, SD_USE_BLOCK)
            .unwrap();

        // Allocate buffer for reading back data
        let mut receive_buf = Vec::new();

        // Read back the data from the same location
        sdcard
            .read_blocks(&mut receive_buf, SD_START_BLOCK, SD_USE_BLOCK)
            .unwrap();

        // Verify data integrity: each u32 should match its index
        for i in 0..receive_buf.len() {
            assert_eq!(receive_buf[i], buffer[i]);
        }

        // Debug: Uncomment to print raw bytes for troubleshooting
        // for i in 0..receive_buf.len() {
        //     warn!("{:x},{:x},{:x},{:x}",
        //     receive_buf[i] as u8,
        //     (receive_buf[i] >> 8) as u8,
        //     (receive_buf[i] >> 16) as u8,
        //     (receive_buf[i] >> 24) as u8);
        // }
        info!("receive buffer len is {}", receive_buf.len());

        info!("test_work passed\n");
    }

    /// Sleeps for the specified duration using busy-waiting.
    ///
    /// This is used for implementing the `Kernel::sleep` trait method
    /// which is required by the phytium-mci driver for timing operations.
    ///
    /// # Arguments
    ///
    /// * `duration` - The amount of time to sleep
    fn sleep(duration: Duration) {
        spin_delay(duration);
    }

    /// Clears pending interrupts from the MCI controller.
    ///
    /// This function reads the raw interrupt status and DMA status registers,
    /// logs them for debugging, then writes them back to clear the pending
    /// interrupts. This is important to prevent spurious interrupts from
    /// previous driver instances or bootloader code.
    ///
    /// # Arguments
    ///
    /// * `reg_base` - Base address of the MCI controller registers
    ///
    /// # Why This is Needed
    ///
    /// - The MCI controller may have pending interrupts from U-Boot or previous runs
    /// - These interrupts could trigger immediately after we register our handler
    /// - Clearing them ensures a clean state before driver initialization
    fn clear_pending_irq(reg_base: NonNull<u8>) {
        let reg = MCIReg::new(reg_base);
        let raw_ints = reg.read_reg::<MCIRawInts>();
        let dmac_status = reg.read_reg::<MCIDMACStatus>();
        info!("before SD card init, clear pending irq!");
        info!(
            "int_mask 0x{:x}, ctrl 0x{:x}, raw_ints 0x{:x}, dmac_status 0x{:x}",
            reg.read_reg::<MCIIntMask>(),
            reg.read_reg::<MCICtrl>(),
            reg.read_reg::<MCIRawInts>(),
            reg.read_reg::<MCIDMACStatus>()
        );
        // Write back to clear the pending interrupts
        reg.write_reg(raw_ints);
        reg.write_reg(dmac_status);
        drop(reg);
    }

    /// Returns the CPU data cache line size.
    ///
    /// This reads the CTR_EL0 system register which contains cache geometry
    /// information. The cache line size is encoded as a logarithmic value
    /// (e.g., 6 means 2^6 = 64 bytes).
    ///
    /// # Returns
    ///
    /// The cache line size in bytes (typically 64 bytes on ARMv8-A)
    #[inline(always)]
    fn cache_line_size() -> usize {
        unsafe {
            let mut ctr_el0: u64;
            asm!("mrs {}, ctr_el0", out(reg) ctr_el0);
            let log2_cache_line_size = ((ctr_el0 >> 16) & 0xF) as usize;
            // Calculate the cache line size: 2^(log2 + 2)
            4 << log2_cache_line_size
        }
    }

    /// Performs a cache operation on a single cache line.
    ///
    /// This uses inline assembly to execute the appropriate data cache
    /// instruction for the given operation type.
    ///
    /// # Arguments
    ///
    /// * `op` - The cache operation to perform (invalidate, clean, or both)
    /// * `addr` - The address of the cache line (must be cache-line aligned)
    ///
    /// # Safety
    ///
    /// The address must be properly aligned to a cache line boundary.
    #[inline(always)]
    fn _dcache_line(op: CacheOp, addr: usize) {
        unsafe {
            match op {
                CacheOp::Invalidate => asm!("dc ivac, {0}", in(reg) addr),
                CacheOp::Clean => asm!("dc cvac, {0}", in(reg) addr),
                CacheOp::CleanAndInvalidate => asm!("dc civac, {0}", in(reg) addr),
            }
        }
    }

    /// Performs a cache operation on a range of memory.
    ///
    /// This function aligns the address range to cache line boundaries and
    /// performs the specified operation on each cache line in the range.
    /// After completing the operations, it issues data and instruction
    /// barrier instructions to ensure the changes are visible.
    ///
    /// # Arguments
    ///
    /// * `op` - The cache operation to perform
    /// * `addr` - Starting virtual address
    /// * `size` - Size of the memory region in bytes
    ///
    /// # Why This is Needed for DMA
    ///
    /// - **Before DMA read**: Invalidate cache (I) to discard stale cached data
    /// - **After DMA read**: Nothing needed (data is in cache)
    /// - **Before DMA write**: Clean cache (C) to flush data to memory
    /// - **After DMA write**: Invalidate cache (I) to prevent using stale cached data
    #[inline(always)]
    fn dcache_range(op: CacheOp, addr: usize, size: usize) {
        let start = addr;
        let end = start + size;
        let cache_line_size = cache_line_size();

        let mut aligned_addr = addr & !(cache_line_size - 1);

        // Perform operation on each cache line in the range
        while aligned_addr < end {
            _dcache_line(op, aligned_addr);
            aligned_addr += cache_line_size;
        }

        // Ensure memory operations complete before continuing
        unsafe {
            __dsb(SY);
            __isb(SY);
        }
    }

    /// Platform-specific implementation of the [`Kernel`] trait.
    ///
    /// This struct provides the bridge between the phytium-mci driver and
    /// the bare-metal test environment. It implements all platform-specific
    /// operations required by the driver:
    ///
    /// - **Sleep**: Busy-wait delay using spin_delay
    /// - **mmap**: Virtual-to-physical address translation for DMA
    /// - **flush**: Data cache clean for DMA writes
    /// - **invalidate**: Data cache invalidate for DMA reads
    struct KernelImpl;

    impl Kernel for KernelImpl {
        fn sleep(duration: Duration) {
            sleep(duration);
        }

        /// Translates a virtual address to physical address for DMA.
        ///
        /// This is required by the DMA engine which needs physical addresses
        /// to access memory. The bare_test framework provides the translation
        /// through its memory management module.
        #[cfg(feature = "dma")]
        fn mmap(virt_addr: NonNull<u8>) -> u64 {
            let vaddr = VirtAddr::from(virt_addr);
            let paddr = PhysAddr::from(vaddr);
            debug!(
                "do mmap, va: {:x}, pa {:x}",
                virt_addr.as_ptr() as usize,
                paddr.raw()
            );
            paddr.raw() as _
        }

        /// Flushes the data cache for the specified memory range.
        ///
        /// This is called before DMA write operations to ensure that all
        /// data in the cache is written back to memory so the DMA engine
        /// can see the latest data.
        fn flush(addr: NonNull<u8>, size: usize) {
            dcache_range(CacheOp::Clean, addr.as_ptr() as _, size);
        }

        /// Invalidates the data cache for the specified memory range.
        ///
        /// This is called after DMA read operations to discard any stale
        /// data in the cache, forcing the CPU to read the fresh data that
        /// the DMA engine just wrote to memory.
        fn invalidate(addr: core::ptr::NonNull<u8>, size: usize) {
            dcache_range(CacheOp::Invalidate, addr.as_ptr() as _, size);
        }
    }

    // Register our Kernel implementation with the phytium-mci driver
    set_impl!(KernelImpl);
}
