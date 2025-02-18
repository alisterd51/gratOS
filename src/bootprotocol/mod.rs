use crate::println;
use core::{
    fmt,
    sync::atomic::{AtomicU32, Ordering},
};

#[cfg(feature = "multiboot")]
pub mod multiboot;
#[cfg(feature = "multiboot2")]
pub mod multiboot2;

pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub entry_type: u32,
}

impl fmt::Display for MemoryMapEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "memory_map_entry: base_addr: {}, length: {}, entry_type: {}",
            self.base_addr, self.length, self.entry_type
        )
    }
}

pub enum MemoryMapIter {
    #[cfg(feature = "multiboot")]
    Multiboot(multiboot::MemoryMapIter),
    #[cfg(feature = "multiboot2")]
    Multiboot2(multiboot2::MemoryMapIter),
    #[allow(dead_code)]
    Empty,
}

impl Iterator for MemoryMapIter {
    type Item = MemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            #[cfg(feature = "multiboot")]
            Self::Multiboot(iter) => iter.next(),
            #[cfg(feature = "multiboot2")]
            Self::Multiboot2(iter) => iter.next(),
            Self::Empty => None,
        }
    }
}

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

#[allow(dead_code)]
pub fn get_memory_map() -> Option<MemoryMapIter> {
    let magic = MAGIC.load(Ordering::SeqCst);
    match magic {
        #[cfg(feature = "multiboot")]
        MULTIBOOT_BOOTLOADER_MAGIC => multiboot::get_memory_map().map(MemoryMapIter::Multiboot),
        #[cfg(feature = "multiboot2")]
        MULTIBOOT2_BOOTLOADER_MAGIC => multiboot2::get_memory_map().map(MemoryMapIter::Multiboot2),
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
