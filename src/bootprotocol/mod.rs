mod info;
mod mmap;

#[cfg(feature = "multiboot")]
pub mod multiboot;
#[cfg(feature = "multiboot2")]
pub mod multiboot2;

pub use info::{get_memory_map, init, print};
pub use mmap::{MemoryMapEntry, MemoryType};
