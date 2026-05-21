use crate::println;
use core::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "multiboot")]
pub mod multiboot;
#[cfg(feature = "multiboot2")]
pub mod multiboot2;

#[cfg(feature = "multiboot")]
const MULTIBOOT_BOOTLOADER_MAGIC: u32 = 0x2BAD_B002;
#[cfg(feature = "multiboot2")]
const MULTIBOOT2_BOOTLOADER_MAGIC: u32 = 0x36D7_6289;

static MAGIC: AtomicU32 = AtomicU32::new(0);

pub fn init(magic: u32, info_addr: u32) {
    MAGIC.store(magic, Ordering::SeqCst);

    match magic {
        #[cfg(feature = "multiboot")]
        MULTIBOOT_BOOTLOADER_MAGIC => multiboot::init(info_addr),
        #[cfg(feature = "multiboot2")]
        MULTIBOOT2_BOOTLOADER_MAGIC => multiboot2::init(info_addr),
        _ => println!("ERROR: invalid magic: {magic:#010X}, info_addr: {info_addr}"),
    }
}

pub fn print() {
    let magic = MAGIC.load(Ordering::SeqCst);

    match magic {
        #[cfg(feature = "multiboot")]
        MULTIBOOT_BOOTLOADER_MAGIC => multiboot::print(),
        #[cfg(feature = "multiboot2")]
        MULTIBOOT2_BOOTLOADER_MAGIC => multiboot2::print(),
        _ => println!("ERROR: invalid magic: {magic:#010X}"),
    }
}
