use crate::println;
use core::{
    fmt,
    sync::atomic::{AtomicU32, Ordering},
};

#[cfg(feature = "multiboot")]
pub mod multiboot;
#[cfg(feature = "multiboot2")]
pub mod multiboot2;

// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format
// https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/15_System_Address_Map_Interfaces/Sys_Address_Map_Interfaces.html
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum MemoryType {
    Memory,           // 1
    Reserved,         // 2
    Acpi,             // 3
    Nvs,              // 4
    Unusable,         // 5
    Disabled,         // 6
    PersistentMemory, // 7
    Undefined(u32),
}

#[derive(Clone, Copy)]
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

impl MemoryMapEntry {
    pub const fn empty() -> Self {
        Self {
            base_addr: 0,
            length: 0,
            entry_type: 0,
        }
    }

    #[allow(dead_code)]
    pub const fn memory_type(&self) -> MemoryType {
        match self.entry_type {
            1 => MemoryType::Memory,
            2 => MemoryType::Reserved,
            3 => MemoryType::Acpi,
            4 => MemoryType::Nvs,
            5 => MemoryType::Unusable,
            6 => MemoryType::Disabled,
            7 => MemoryType::PersistentMemory,
            other => MemoryType::Undefined(other),
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
