use crate::{bootprotocol::MemoryMapEntry, println};
use core::{
    cell::UnsafeCell,
    ffi::{CStr, c_char},
    fmt,
};

// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format
#[derive(Clone, Copy)]
#[repr(C)]
struct MultibootInfo {
    flags: u32,              // (required)
    mem_lower: u32,          // (present if flags[0] is set)
    mem_upper: u32,          // (present if flags[0] is set)
    boot_device: u32,        // (present if flags[1] is set)
    cmdline: u32,            // (present if flags[2] is set)
    mods_count: u32,         // (present if flags[3] is set)
    mods_addr: u32,          // (present if flags[3] is set)
    syms: [u32; 4],          // (present if flags[4] or flags[5] is set)
    mmap_length: u32,        // (present if flags[6] is set)
    mmap_addr: u32,          // (present if flags[6] is set)
    drives_length: u32,      // (present if flags[7] is set)
    drives_addr: u32,        // (present if flags[7] is set)
    config_table: u32,       // (present if flags[8] is set)
    boot_loader_name: u32,   // (present if flags[9] is set)
    apm_table: u32,          // (present if flags[10] is set)
    vbe_control_info: u32,   // (present if flags[11] is set)
    vbe_mode_info: u32,      // (present if flags[11] is set)
    vbe_mode: u16,           // (present if flags[11] is set)
    vbe_interface_seg: u16,  // (present if flags[11] is set)
    vbe_interface_off: u16,  // (present if flags[11] is set)
    vbe_interface_len: u16,  // (present if flags[11] is set)
    framebuffer_addr: u64,   // (present if flags[12] is set)
    framebuffer_pitch: u32,  // (present if flags[12] is set)
    framebuffer_width: u32,  // (present if flags[12] is set)
    framebuffer_height: u32, // (present if flags[12] is set)
    framebuffer_bpp: u8,     // (present if flags[12] is set)
    framebuffer_type: u8,    // (present if flags[12] is set)
    color_info_0: u16,       // (present if flags[12] is set)
    color_info_1: u16,       // (present if flags[12] is set)
    color_info_2: u16,       // (present if flags[12] is set)
}

#[repr(C, packed)]
pub struct MultibootMemoryMapEntry {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub entry_type: u32,
}

impl fmt::Display for MultibootMemoryMapEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.size;
        let base_addr = self.base_addr;
        let length = self.length;
        let entry_type = self.entry_type;

        write!(
            f,
            "memory_map_entry: size: {size}, base_addr: {base_addr}, length: {length}, entry_type: {entry_type}"
        )
    }
}

const MAX_MEMORY_ENTRIES: usize = 64;

struct BootCache {
    is_ready: bool,
    info: Option<MultibootInfo>,
    cmdline: [u8; 256],
    bootloader_name: [u8; 64],
    memory_map: [MemoryMapEntry; MAX_MEMORY_ENTRIES],
    memory_map_count: usize,
}

impl BootCache {
    const fn empty() -> Self {
        Self {
            is_ready: false,
            info: None,
            cmdline: [0; 256],
            bootloader_name: [0; 64],
            memory_map: [MemoryMapEntry::empty(); MAX_MEMORY_ENTRIES],
            memory_map_count: 0,
        }
    }
}

struct GlobalCache(UnsafeCell<BootCache>);

unsafe impl Sync for GlobalCache {}

static CACHE: GlobalCache = GlobalCache(UnsafeCell::new(BootCache::empty()));

unsafe fn copy_cstr(phys_addr: u32, dest: &mut [u8]) {
    let src_bytes = unsafe { CStr::from_ptr(phys_addr as *const c_char).to_bytes() };
    let len = src_bytes.len().min(dest.len() - 1);
    dest[..len].copy_from_slice(&src_bytes[..len]);
    dest[len] = 0;
}

fn as_str(buffer: &[u8]) -> &str {
    CStr::from_bytes_until_nul(buffer)
        .map_or("", |c_str| c_str.to_str().unwrap_or("<invalid utf8>"))
}

pub fn init(info_addr: u32) {
    let info = unsafe { &*(info_addr as *const MultibootInfo) };
    let cache = unsafe { &mut *CACHE.0.get() };

    cache.info = Some(*info);
    cache.is_ready = true;

    if info.flags & (1 << 2) != 0 {
        let dest = &mut cache.cmdline;
        unsafe { copy_cstr(info.cmdline, dest) };
    }
    if info.flags & (1 << 9) != 0 {
        let dest = &mut cache.bootloader_name;
        unsafe { copy_cstr(info.boot_loader_name, dest) };
    }

    if info.flags & (1 << 6) != 0 {
        let mut current_addr = info.mmap_addr;
        let end_addr = info.mmap_addr + info.mmap_length;

        while current_addr < end_addr && cache.memory_map_count < MAX_MEMORY_ENTRIES {
            let entry_ptr = current_addr as *const MultibootMemoryMapEntry;
            let entry = unsafe { core::ptr::read_unaligned(entry_ptr) };

            cache.memory_map[cache.memory_map_count] = MemoryMapEntry {
                base_addr: entry.base_addr,
                length: entry.length,
                entry_type: entry.entry_type,
            };

            cache.memory_map_count += 1;
            current_addr += entry.size + 4;
        }
    }
}

pub fn get_memory_map() -> &'static [MemoryMapEntry] {
    let cache = unsafe { &*CACHE.0.get() };
    &cache.memory_map[..cache.memory_map_count]
}

pub fn print() {
    let cache = unsafe { &*CACHE.0.get() };

    if cache.is_ready
        && let Some(info) = cache.info
    {
        println!("flags: {}", info.flags);
        if info.flags & (1 << 0) != 0 {
            println!(
                "mem_lower: {}, mem_upper: {}",
                info.mem_lower, info.mem_upper
            );
        }
        if info.flags & (1 << 1) != 0 {
            println!("boot_device: {}", info.boot_device);
        }
        if info.flags & (1 << 2) != 0 {
            println!("cmdline: {}", as_str(&cache.cmdline));
        }
        if info.flags & (1 << 3) != 0 {
            println!(
                "mods_count: {}, mods_addr: {}",
                info.mods_count, info.mods_addr
            );
        }
        if info.flags & ((1 << 4) | (1 << 5)) != 0 {
            println!("syms: {:?}", info.syms);
        }
        if info.flags & (1 << 6) != 0 {
            println!(
                "mmap_length: {}, mmap_addr: {}",
                info.mmap_length, info.mmap_addr
            );
            for memory_map_entry in get_memory_map() {
                println!("{memory_map_entry}");
            }
        }
        if info.flags & (1 << 7) != 0 {
            println!(
                "drives_length: {}, drives_addr: {}",
                info.drives_length, info.drives_addr
            );
        }
        if info.flags & (1 << 8) != 0 {
            println!("config_table: {}", info.config_table);
        }
        if info.flags & (1 << 9) != 0 {
            println!("boot_loader_name: {}", as_str(&cache.bootloader_name));
        }
        if info.flags & (1 << 10) != 0 {
            println!("apm_table: {}", info.apm_table);
        }
        if info.flags & (1 << 11) != 0 {
            println!(
                "vbe_control_info: {}, vbe_mode_info: {}, vbe_mode: {}, vbe_interface_seg: {}, vbe_interface_off: {}, vbe_interface_len: {}",
                info.vbe_control_info,
                info.vbe_mode_info,
                info.vbe_mode,
                info.vbe_interface_seg,
                info.vbe_interface_off,
                info.vbe_interface_len,
            );
        }
        if info.flags & (1 << 12) != 0 {
            println!(
                "framebuffer_addr: {}, framebuffer_pitch: {}, framebuffer_width: {}, framebuffer_height: {}, framebuffer_bpp: {}, framebuffer_type: {}, color_info: {} {} {}",
                info.framebuffer_addr,
                info.framebuffer_pitch,
                info.framebuffer_width,
                info.framebuffer_height,
                info.framebuffer_bpp,
                info.framebuffer_type,
                info.color_info_0,
                info.color_info_1,
                info.color_info_2,
            );
        }
    }
}
