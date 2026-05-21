use crate::println;
use core::sync::atomic::{AtomicU32, Ordering};

// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format
#[repr(C)]
struct MultibootInfo {
    flags: u32,              // (required)
    mem_lower: u32,          // (present if flags[0] is set)
    mem_upper: u32,          // (present if flags[0] is set)
    boot_device: u32,        // (present if flags[1] is set)
    cmdline: u32,            // (present if flags[2] is set)
    mods_count: u32,         // (present if flags[3] is set)
    mods_addr: u32,          // (present if flags[3] is set)
    syms: u128,              // (present if flags[4] or flags[5] is set)
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

static MULTIBOOT_ADDR: AtomicU32 = AtomicU32::new(0);

pub fn init(info_addr: u32) {
    MULTIBOOT_ADDR.store(info_addr, Ordering::SeqCst);
}

pub fn print() {
    let info_addr = MULTIBOOT_ADDR.load(Ordering::SeqCst);
    let multiboot_info = unsafe { &*(info_addr as *const MultibootInfo) };
    println!("flags: {}", multiboot_info.flags);
    if multiboot_info.flags & (1 << 0) != 0 {
        println!(
            "mem_lower: {}, mem_upper: {}",
            multiboot_info.mem_lower, multiboot_info.mem_upper
        );
    }
    if multiboot_info.flags & (1 << 1) != 0 {
        println!("boot_device: {}", multiboot_info.boot_device);
    }
    if multiboot_info.flags & (1 << 2) != 0 {
        println!("cmdline: {}", multiboot_info.cmdline);
    }
    if multiboot_info.flags & (1 << 3) != 0 {
        println!(
            "mods_count: {}, mods_addr: {}",
            multiboot_info.mods_count, multiboot_info.mods_addr
        );
    }
    if multiboot_info.flags & ((1 << 4) | (1 << 5)) != 0 {
        println!("syms: {}", multiboot_info.syms);
    }
    if multiboot_info.flags & (1 << 6) != 0 {
        println!(
            "mmap_length: {}, mmap_addr: {}",
            multiboot_info.mmap_length, multiboot_info.mmap_addr
        );
    }
    if multiboot_info.flags & (1 << 7) != 0 {
        println!(
            "drives_length: {}, drives_addr: {}",
            multiboot_info.drives_length, multiboot_info.drives_addr
        );
    }
    if multiboot_info.flags & (1 << 8) != 0 {
        println!("config_table: {}", multiboot_info.config_table);
    }
    if multiboot_info.flags & (1 << 9) != 0 {
        println!("boot_loader_name: {}", multiboot_info.boot_loader_name);
    }
    if multiboot_info.flags & (1 << 10) != 0 {
        println!("apm_table: {}", multiboot_info.apm_table);
    }
    if multiboot_info.flags & (1 << 11) != 0 {
        println!(
            "vbe_control_info: {}, vbe_mode_info: {}, vbe_mode: {}, vbe_interface_seg: {}, vbe_interface_off: {}, vbe_interface_len: {}",
            multiboot_info.vbe_control_info,
            multiboot_info.vbe_mode_info,
            multiboot_info.vbe_mode,
            multiboot_info.vbe_interface_seg,
            multiboot_info.vbe_interface_off,
            multiboot_info.vbe_interface_len,
        );
    }
    if multiboot_info.flags & (1 << 12) != 0 {
        println!(
            "framebuffer_addr: {}, framebuffer_pitch: {}, framebuffer_width: {}, framebuffer_height: {}, framebuffer_bpp: {}, framebuffer_type: {}, color_info: {} {} {}",
            multiboot_info.framebuffer_addr,
            multiboot_info.framebuffer_pitch,
            multiboot_info.framebuffer_width,
            multiboot_info.framebuffer_height,
            multiboot_info.framebuffer_bpp,
            multiboot_info.framebuffer_type,
            multiboot_info.color_info_0,
            multiboot_info.color_info_1,
            multiboot_info.color_info_2,
        );
    }
}
