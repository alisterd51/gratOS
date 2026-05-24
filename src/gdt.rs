use crate::println;
use core::{arch::asm, fmt};

const RPL_KERNEL: u16 = 0;
#[allow(dead_code)]
const RPL_USER: u16 = 3;

const KERNEL_CODE_SEGMENT_SELECTOR: u16 = (1 << 3) | RPL_KERNEL;
const KERNEL_DATA_SEGMENT_SELECTOR: u16 = (2 << 3) | RPL_KERNEL;
#[allow(dead_code)]
pub const USER_CODE_SEGMENT_SELECTOR: u16 = (3 << 3) | RPL_USER;
#[allow(dead_code)]
pub const USER_DATA_SEGMENT_SELECTOR: u16 = (4 << 3) | RPL_USER;

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    const fn new(base: u32, limit: u32, access: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | 0xC0,
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }

    const fn null() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}

impl fmt::Display for SegmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = u32::from(self.base_low)
            | u32::from(self.base_middle) << 16
            | u32::from(self.base_high) << 24;
        let limit = u32::from(self.limit_low) | (u32::from(self.granularity) & 0x0F) << 16;
        let limit = if (self.granularity & 0x80) != 0 {
            (limit << 12) | 0xFFF
        } else {
            limit
        };
        write!(
            f,
            "base: {:#010X}, limit: {:#010X}, access: {:#04X}, flags: {:#04X}",
            base, limit, self.access, self.granularity
        )
    }
}

const GDT_SIZE: usize = 5;

#[repr(C, align(8))]
struct Gdt([SegmentDescriptor; GDT_SIZE]);

static mut GDT: Gdt = Gdt([
    SegmentDescriptor::null(),                // Index 0 : Null
    SegmentDescriptor::new(0, 0xFFFFF, 0x9A), // Index 1 : Kernel Code
    SegmentDescriptor::new(0, 0xFFFFF, 0x92), // Index 2 : Kernel Data/Stack
    SegmentDescriptor::new(0, 0xFFFFF, 0xFA), // Index 3 : User Code
    SegmentDescriptor::new(0, 0xFFFFF, 0xF2), // Index 4 : User Data/Stack
]);

pub fn init() {
    let gdtr = GdtDescriptor {
        #[allow(clippy::cast_possible_truncation)]
        limit: (core::mem::size_of::<Gdt>() - 1) as u16,
        base: (&raw const GDT) as u32,
    };
    unsafe {
        asm!(
            "lgdt [{}]",
            in(reg) &raw const gdtr,
            options(nostack, preserves_flags)
        );

        asm!(
            "mov ax, {selector}",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            options(nostack),
            selector = const KERNEL_DATA_SEGMENT_SELECTOR
        );

        asm!(
            "push {selector}",
            "lea {tmp}, [2f]",
            "push {tmp}",
            "retf",
            "2:",
            selector = const KERNEL_CODE_SEGMENT_SELECTOR,
            tmp = out(reg) _,
        );
    }
}

pub fn print() {
    let mut gdtr = GdtDescriptor { limit: 0, base: 0 };

    unsafe {
        asm!(
            "sgdt [{}]",
            in(reg) &raw mut gdtr,
            options(nostack, preserves_flags)
        );
    }
    let base = gdtr.base;
    let limit = gdtr.limit;
    println!("gdt: base: {base:#010X}, limit: {limit:#010X}");
    let entry_count = (gdtr.limit + 1) / 8;
    let gdt_ptr = gdtr.base as *const SegmentDescriptor;
    for entry in 0..entry_count {
        let segment_descriptor = unsafe { &*gdt_ptr.add(entry as usize) };
        println!("entry: {entry}: {segment_descriptor}");
    }
}
