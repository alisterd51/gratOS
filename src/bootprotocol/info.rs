#[cfg(feature = "multiboot")]
use crate::bootprotocol::multiboot;
#[cfg(feature = "multiboot2")]
use crate::bootprotocol::multiboot2;
use crate::{bootprotocol::MemoryMapEntry, println};
use core::sync::atomic::{AtomicU32, Ordering};

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

pub fn get_memory_map() -> Option<&'static [MemoryMapEntry]> {
    let magic = MAGIC.load(Ordering::SeqCst);
    match magic {
        #[cfg(feature = "multiboot")]
        MULTIBOOT_BOOTLOADER_MAGIC => Some(multiboot::get_memory_map()),
        #[cfg(feature = "multiboot2")]
        MULTIBOOT2_BOOTLOADER_MAGIC => Some(multiboot2::get_memory_map()),
        _ => None,
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
