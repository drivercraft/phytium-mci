#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;

#[bare_test::tests]
mod tests {
    use core::time::Duration;

    use alloc::vec::Vec;
    use bare_test::{
        GetIrqConfig,
        globals::{PlatformInfoKind, global_val},
        irq::{IrqHandleResult, IrqParam},
        mem::mmu::iomap,
        time::spin_delay,
    };
    use log::*;
    use phytium_mci::{
        sd::{SdCard, init_reg_base},
        *,
    };

    const SD_START_BLOCK: u32 = 131072;
    const SD_USE_BLOCK: u32 = 4;
    // const SD_BLOCK_SIZE: u32 = 512;
    // const SD_MAX_RW_BLK: u32 = 1024;

    #[test]
    fn test_work() {
        let fdt = match &global_val().platform_info {
            PlatformInfoKind::DeviceTree(fdt) => fdt.get(),
        };

        let mci0 = fdt.find_compatible(&["phytium,mci"]).next().unwrap();
        let reg = mci0.reg().unwrap().next().unwrap();
        info!(
            "mci0 reg: {:#x}, mci0 reg size: {:#x}",
            reg.address,
            reg.size.unwrap()
        );

        let mci_reg_base = iomap((reg.address as usize).into(), reg.size.unwrap());

        // 一定要初始化，不然注册中断处理函数会报错
        init_reg_base(mci_reg_base);

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

        info!("mci0 reg mapped to: {:#x}", mci_reg_base.as_ptr() as usize);

        let mut sdcard = SdCard::new(mci_reg_base);

        ////////////////////// SD card init finished //////////////////////

        // 初始化write buffer
        // let mut buffer: Vec<u32> = Vec::with_capacity((SD_BLOCK_SIZE * SD_MAX_RW_BLK / 4) as usize);
        // buffer.resize((SD_BLOCK_SIZE * SD_MAX_RW_BLK / 4) as usize, 0);
        // for i in 0..buffer.len() {
        //     buffer[i] = i as u32;
        // }

        // sdcard
        //     .write_blocks(&mut buffer, SD_START_BLOCK, SD_USE_BLOCK)
        //     .unwrap();

        let mut receive_buf = Vec::new();

        sdcard
            .read_blocks(&mut receive_buf, SD_START_BLOCK, SD_USE_BLOCK)
            .unwrap();

        // for i in 0..receive_buf.len() {
        //     assert_eq!(receive_buf[i], buffer[i]);
        // }

        info!("read {} blocks from SD card", receive_buf.len());
        info!("buffer is {:?}", receive_buf);

        info!("test_work passed\n");
    }

    fn sleep(duration: Duration) {
        spin_delay(duration);
    }

    struct KernelImpl;

    impl Kernel for KernelImpl {
        fn sleep(duration: Duration) {
            sleep(duration);
        }
    }

    set_impl!(KernelImpl);
}
