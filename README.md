# phytium-mci

A `no_std` Rust driver for SD/MMC cards on Phytium E2000 series SoCs.

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/crate-name.svg)](https://crates.io/crates/phytium-mci)
[![Documentation](https://docs.rs/crate-name/badge.svg)](https://docs.rs/phytium-mci)
[![License](https://img.shields.io/crates/l/crate-name.svg)](LICENSE)

## Overview

**phytium-mci** is a comprehensive SD/MMC host controller driver designed for Phytium SoC platforms (specifically the E2000 series used in Phytium Pi development boards). It implements the full SD/MMC protocol stack from hardware register manipulation to high-level card operations, supporting both SD and eMMC cards with DMA and PIO transfer modes.

## Key Features

- **Full SD Specification Support**: SDSC, SDHC, SDXC (Specification versions 1.0-3.0)
- **eMMC Support**: MMC protocol implementation
- **Flexible Transfer Modes**: DMA (high-performance) and PIO (simple) transfers
- **Voltage Support**: 3.3V (default) and 1.8V (UHS-I modes)
- **Bus Widths**: 1-bit, 4-bit, and 8-bit (eMMC) data bus
- **High-Speed Modes**: SDR12, SDR25, SDR50, SDR104, DDR50
- **Clock Speed Support**: From 400 KHz (initialization) up to 208 MHz (SDR104)
- **Card Detection**: GPIO-based and host-based card detection
- **Interrupt Support**: Command completion, data transfer, and card detection interrupts
- **Platform Abstraction**: Clean separation through the `Kernel` trait

## Architecture

The driver is organized into distinct layers:

```
Application Layer    (SdCard, MCIHost - High-level API)
       ↓
Protocol Layer       (Command/Data transfer, Card initialization)
       ↓
Hardware Abstraction (Register access, DMA/PIO control)
       ↓
Hardware Support     (IoPad pin configuration, OSA memory/timing)
```

### Module Structure

- **[mci/](src/mci/)** - Hardware controller driver (register access, DMA/PIO, interrupts)
- **[mci_host/](src/mci_host/)** - Host controller protocol layer (SD/MMC protocol implementation)
- **[iopad/](src/iopad/)** - I/O pad configuration for pin multiplexing
- **[osa/](src/osa/)** - OS abstraction layer (memory management, event flags)

## Requirements

- Rust 2024 edition
- Phytium E2000 series SoC or compatible platform
- `no_std` environment (bare-metal or custom OS)

## Dependencies

```toml
tock-registers = "0.9.0"   # Type-safe register access
log = "0.4"                 # Logging facade
nb = "1.1"                  # Non-blocking I/O
bitflags = "2.8"            # Bit flags
bytemuck = "1.22.0"         # Safe byte casting
lazy_static = "1.5.0"       # Global state
spin = "0.10.0"             # Spin locks
rlsf = "0.2.1"              # Memory allocator
```

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `dma`   | Enable DMA transfers | No |
| `pio`   | Enable PIO transfers | Yes |
| `poll`  | Enable polling mode | Yes |
| `irq`   | Enable interrupt mode | No |

```toml
# Default: PIO + Poll mode (simpler, good for debugging)
[dependencies]
phytium-mci = { version = "0.1.0" }

# Recommended: DMA + IRQ (high performance)
[dependencies]
phytium-mci = { version = "0.1.0", features = ["dma", "irq"] }
```

## Usage

### 1. Platform Integration

Implement the `Kernel` trait to provide platform-specific functionality:

```rust
use phytium_mci::{Kernel, set_impl};
use core::{ptr::NonNull, time::Duration};

struct MyPlatform;

impl Kernel for MyPlatform {
    fn sleep(duration: Duration) {
        // Platform-specific delay implementation
        platform_delay(duration);
    }

    #[cfg(feature = "dma")]
    fn mmap(virt_addr: NonNull<u8>) -> u64 {
        // Virtual to physical address translation for DMA
        platform_virt_to_phys(virt_addr)
    }

    fn flush(addr: NonNull<u8>, size: usize) {
        // Cache clean for DMA
        platform_cache_clean(addr, size);
    }

    fn invalidate(addr: NonNull<u8>, size: usize) {
        // Cache invalidate for DMA
        platform_cache_invalidate(addr, size);
    }
}

// Register your implementation
set_impl!(MyPlatform);
```

### 2. Basic SD Card Initialization

```rust
use phytium_mci::{sd::SdCard, IoPad};
use core::ptr::NonNull;

fn main() {
    // Get register base addresses from device tree or platform config
    let mci_reg_base = 0x2800_1000 as *mut u8;
    let iopad_reg_base = 0x2800_0000 as *mut u8;

    // Initialize IOPAD for pin configuration
    let iopad = unsafe { IoPad::new(NonNull::new_unchecked(iopad_reg_base)) };

    // Create SD card instance
    let mut sdcard = unsafe {
        SdCard::new(
            NonNull::new_unchecked(mci_reg_base),
            iopad
        )
    };

    // Initialize the card
    if let Err(e) = sdcard.init(NonNull::new_unchecked(mci_reg_base)) {
        panic!("SD card init failed: {:?}", e);
    }

    println!("Card initialized!");
    println!("Block size: {} bytes", sdcard.block_size());
    println!("Total blocks: {}", sdcard.block_count());
    println!("Total capacity: {} MB", sdcard.capacity() / (1024 * 1024));
}
```

### 3. Reading Blocks

```rust
use phytium_mci::sd::SdCard;
use alloc::vec::Vec;

fn read_blocks(sdcard: &mut SdCard, start_block: u32, block_count: u32) {
    let mut buffer = Vec::new();

    sdcard.read_blocks(&mut buffer, start_block, block_count)
        .expect("Read failed");

    println!("Read {} blocks ({} bytes)", block_count, buffer.len() * 4);
}
```

### 4. Writing Blocks

```rust
use phytium_mci::sd::SdCard;
use alloc::vec::Vec;

fn write_blocks(sdcard: &mut SdCard, start_block: u32, block_count: u32) {
    // Prepare data buffer (blocks are in 32-bit words)
    let mut buffer: Vec<u32> = Vec::with_capacity((block_count * 128) as usize);
    buffer.resize((block_count * 128) as usize, 0);

    // Fill with pattern
    for i in 0..buffer.len() {
        buffer[i] = i as u32;
    }

    // Write blocks
    sdcard.write_blocks(&mut buffer, start_block, block_count)
        .expect("Write failed");

    println!("Written {} blocks starting at {}", block_count, start_block);
}
```

### 5. Configuration for Different Modes

```rust
use phytium_mci::mci::MCIConfig;
use phytium_mci::mci_host::MCIHostConfig;
use phytium_mci::mci_host::MCIHostType;
use phytium_mci::mci_host::MCIHostCardType;
use phytium_mci::mci_host::MCIHostEndianMode;

// For DMA mode (high performance)
let host_config = MCIHostConfig {
    host_type: MCIHostType::SDIF,
    card_type: MCIHostCardType::SDCard,
    card_clock: 50_000_000,     // 50 MHz
    max_trans_size: 512 * 1024, // 512KB max transfer
    def_block_size: 512,
    enable_dma: true,
    is_uhs_card: true,          // Enable UHS-I support
    endian_mode: MCIHostEndianMode::Little,
};
```

## Hardware Details

### Target Hardware

| Component | Description |
|-----------|-------------|
| **SoC** | Phytium E2000 series (ARMv8-A architecture) |
| **Board** | Phytium Pi development board |
| **Controller** | Phytium SDIF (Synopsys DesignWare-based) |
| **MCI0 Base** | 0x2800_1000 |
| **MCI1 Base** | 0x2800_2000 |
| **IOPAD Base** | 0x2800_0000 |

### Clock Configuration

- **Source Clock**: 1.2 GHz
- **Initialization**: 400 KHz (for card detection and initialization)
- **Default Speed**: 25 MHz
- **High Speed**: 50 MHz
- **UHS-I SDR104**: Up to 208 MHz

### Voltage Modes

| Mode | Voltage | Bus Width | Max Clock |
|------|---------|-----------|-----------|
| Default | 3.3V | 1-bit/4-bit | 25 MHz |
| High Speed | 3.3V | 4-bit | 50 MHz |
| SDR12 | 1.8V | 4-bit | 25 MHz |
| SDR25 | 1.8V | 4-bit | 50 MHz |
| SDR50 | 1.8V | 4-bit | 100 MHz |
| SDR104 | 1.8V | 4-bit | 208 MHz |
| DDR50 | 1.8V | 4-bit | 50 MHz |

## API Reference

### Main Types

#### `SdCard`

High-level SD card interface.

```rust
impl SdCard {
    pub unsafe fn new(reg_base: NonNull<u8>, io_pad: IoPad) -> Self;
    pub fn init(&mut self, reg_base: NonNull<u8>) -> Result<(), MCIHostError>;
    pub fn read_blocks(&mut self, buf: &mut Vec<u32>, start: u32, cnt: u32) -> Result<(), MCIHostError>;
    pub fn write_blocks(&mut self, buf: &mut Vec<u32>, start: u32, cnt: u32) -> Result<(), MCIHostError>;
    pub fn block_size(&self) -> u32;
    pub fn block_count(&self) -> u32;
    pub fn capacity(&self) -> u64;
    pub fn cid(&self) -> &SdCid;
    pub fn csd(&self) -> &SdCsd;
    pub fn scr(&self) -> &SdScr;
}
```

#### `MCIHost`

Host controller abstraction.

```rust
impl MCIHost {
    pub fn new(dev: Box<SDIFDev>, config: MCIHostConfig) -> Self;
    pub fn init(&mut self) -> Result<(), MCIHostError>;
    pub fn transfer(&mut self, content: &mut MCICmdData) -> Result<(), MCIHostError>;
    pub fn set_card_bus_width(&mut self, width: MCIHostBusWdith) -> Result<(), MCIHostError>;
    pub fn set_card_clock(&mut self, freq: u32) -> Result<(), MCIHostError>;
}
```

#### `IoPad`

I/O pad configuration for pin multiplexing.

```rust
impl IoPad {
    pub unsafe fn new(reg: NonNull<u8>) -> Self;
    pub fn init(&mut self) -> Result<(), IoPadError>;
    pub fn set_pin_function(&mut self, pin: u8, func: PinFunction) -> Result<(), IoPadError>;
    pub fn set_pin_pull(&mut self, pin: u8, pull: PinPull) -> Result<(), IoPadError>;
}
```

### Card Information Structures

```rust
pub struct SdCid {
    pub manufacturer_id: u8,
    pub oem_id: [u8; 2],
    pub product_name: [u8; 5],
    pub product_revision: u8,
    pub serial_number: u32,
    pub month: u8,
    pub year: u16,
}

pub struct SdCsd {
    pub card_capacity: u64,
    pub read_block_length: u32,
    pub write_speed: u32,
}

pub struct SdScr {
    pub sd_spec: u8,
    pub bus_widths: [bool; 3],
}
```

## Error Handling

The crate provides comprehensive error types:

```rust
// MCI (Hardware) errors
pub enum MCIError {
    Timeout,           // Operation timeout
    NotInit,           // Controller not initialized
    ShortBuf,          // Buffer too small
    NotSupport,        // Operation not supported
    InvalidState,      // Invalid controller state
    TransTimeout,      // Transfer timeout
    CmdTimeout,        // Command timeout
    NoCard,            // No card detected
    Busy,              // Card busy
    DmaBufUnalign,     // DMA buffer misaligned
    InvalidTiming,     // Invalid timing configuration
}

// Host (Protocol) errors
pub enum MCIHostError {
    Fail, TransferFailed, Timeout, Busy, NoData,
    NotSupportYet, CardNotSupport, HostNotSupport,
    SwitchVoltageFail, TuningFail, CardInitFailed,
    // ... 60+ specific error variants
}
```

## Memory Management

The crate includes a custom TLSF-based memory pool allocator for DMA operations:

```rust
use phytium_mci::osa::{FMemp, PoolBuffer};

// Initialize the global memory pool
unsafe { FMemp::init(pool_base, pool_size); }

// Allocate aligned buffer for DMA
let buffer = PoolBuffer::alloc(4096, 512).expect("Allocation failed");

// Use buffer...
// Buffer is automatically freed when dropped
```

## Testing

⚠️ **Hardware integration tests require physical Phytium Pi hardware.**

This project provides bare-metal integration tests that run on actual Phytium Pi hardware to verify SD/MMC card functionality.

#### Prerequisites

1. **Phytium Pi Hardware**
   - Phytium Pi development board
   - SD card inserted
   - Serial port connected

2. **Install ostool:**
   ```bash
   cargo install ostool
   ```

3. **Configure device tree** (use `firmware/phytium.dtb`)

#### Running Hardware Tests

```bash
# Build and run on Phytium Pi
cargo test --test test --target aarch64-unknown-none -- --show-output uboot

# PIO mode only
cargo test --test test --target aarch64-unknown-none --no-default-features --features pio -- --show-output uboot
```

**IMPORTANT:** Hardware integration tests CANNOT run on:
- ❌ Virtual machines or emulators
- ❌ x86_64 or other non-ARM platforms
- ❌ Systems without SD/MMC hardware

The tests communicate via serial port and require:
- U-Boot with TFTP support
- Physical Phytium Pi hardware
- Working SD card interface

## Platform Abstraction

The `Kernel` trait provides a clean abstraction for platform-specific operations:

```rust
pub trait Kernel {
    fn sleep(duration: Duration);
    #[cfg(feature = "dma")]
    fn mmap(virt_addr: NonNull<u8>) -> u64;
    fn flush(addr: NonNull<u8>, size: usize);
    fn invalidate(addr: NonNull<u8>, size: usize);
}
```

This allows the driver to work with:
- Bare-metal applications
- Custom operating systems
- Embedded frameworks

## Card Information

The driver provides detailed card information:

```rust
let sdcard: &SdCard = /* ... */;

// Basic information
println!("Capacity: {} MB", sdcard.capacity() / (1024 * 1024));
println!("Block size: {} bytes", sdcard.block_size());
println!("Block count: {}", sdcard.block_count());

// Card identification
println!("Manufacturer ID: {:#02x}", sdcard.cid().manufacturer_id);
println!("Product name: {}", sdcard.cid().product_name);
println!("Serial number: {:#x}", sdcard.cid().serial_number);
println!("Manufacturing date: {}/{}", sdcard.cid().month, sdcard.cid().year);

// Card specific data
println!("Card capacity: {}", sdcard.csd().card_capacity());
println!("Read block length: {}", sdcard.csd().read_block_length());
println!("Write speed: {}", sdcard.csd().write_speed());

// SD configuration
println!("SD version: {}", sdcard.scr().sd_spec());
println!("Bus width support: {:#?}", sdcard.scr().bus_widths());
```

## License

This project is licensed under MIT.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For issues, questions, or contributions related to Phytium hardware, please visit:
- [Phytium Developer Community](https://www.phytium.com.cn)
- [GitHub Issues](https://github.com/drivercraft/phytium-mci/issues)
