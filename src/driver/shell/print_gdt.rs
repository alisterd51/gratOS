use crate::driver;
use crate::println;
use core::arch::asm;

struct SegmentDescriptor {
    base: u32,
    limit: u32,
    flags: u8,
    access_byte: u8,
}

// Function which parses the segment descriptor
// Elements of a segment descriptor are quite scattered
// so we need to combine them to get the actual values
fn parse_segment_descriptor(descriptor: u64) -> SegmentDescriptor {
    // The base address is split into 3 parts
    // The bits from 16 to 31 are the first 16 bits of the base address
    // The bits from 32 to 39 are the next 8 bits of the base address
    // and the bits from 56 to 63 are the last 8 bits of the base address
    let base = ((descriptor >> 16) & 0xFFFF) as u32
        | (((descriptor >> 32) & 0xFF) as u32) << 16
        | (((descriptor >> 56) & 0xFF) as u32) << 24;

    // The limit is split into 2 parts
    // The bits from 0 to 15 are the first 16 bits of the limit
    // The bits from 48 to 51 are the next 4 bits of the limit
    let limit = ((descriptor & 0xFFFF) as u32) | (((descriptor >> 48) & 0xF) as u32) << 16;

    // The flags are the bits from 52 to 55
    let flags = ((descriptor >> 52) & 0xF) as u8;

    // The access byte is the bits from 40 to 47
    let access_byte = ((descriptor >> 40) & 0xFF) as u8;

    SegmentDescriptor {
        base,
        limit,
        flags,
        access_byte,
    }
}

fn describe_flags(flags: u8) {
    let granularity = if (flags & 0b1000) != 0 { "4KB" } else { "1B" };
    let size = if (flags & 0b0100) != 0 {
        "32-bit"
    } else {
        "16-bit"
    };
    let long_mode = if (flags & 0b0010) != 0 {
        "64-bit"
    } else {
        "Legacy"
    };
    let reserved = if (flags & 0b0001) != 0 {
        "Reserved"
    } else {
        "Unused"
    };

    println!(
        "+-------------+---------+-----------+----------+\n\
         | Granularity | Size    | Long Mode | Reserved |\n\
         +-------------+---------+-----------+----------+\n\
         | {:<11} | {:<7} | {:<9} | {:<8} |\n\
         +-------------+---------+-----------+----------+",
        granularity, size, long_mode, reserved
    );
}

fn check_bit(byte: u8, bit: u8) -> bool {
    (byte & (1 << bit)) != 0
}

fn describe_access_byte(access_byte: u8) {
    let accessed = check_bit(access_byte, 0);
    let readable_or_writable = check_bit(access_byte, 1);
    let direction_or_conforming = check_bit(access_byte, 2);
    let executable = check_bit(access_byte, 3);
    let descriptor_type = if check_bit(access_byte, 4) {
        "Code/Data"
    } else {
        "System"
    };
    let dpl = (access_byte & 0b0110_0000) >> 5;
    let present = check_bit(access_byte, 7);

    println!(
        "+----------+-------------------+-----------------------+------------+\n\
         | Accessed | Readable/Writable | Direction/Conforming  | Executable |\n\
         +----------+-------------------+-----------------------+------------+\n\
         | {:<8} | {:<17} | {:<21} | {:<10} |\n\
         +----------+-------------------+-----------------------+------------+\n\
         +-----------------+----------------+--------------+\n\
         | Descriptor Type | DPL            |   Present    |\n\
         +-----------------+----------------+--------------+\n\
         | {:<15} | {:<14} | {:<12} |\n\
         +-----------------+----------------+--------------+",
        accessed,
        readable_or_writable,
        direction_or_conforming,
        executable,
        descriptor_type,
        dpl,
        present
    );
}

fn print_segment(segment: &SegmentDescriptor, index: u16) {
    println!(
        "{}+-------------------------+\n\
             |       Segment {:<9} |\n\
             +-------------------------+{}",
        driver::console::FG_BLUE,
        index,
        driver::console::RESET
    );
    println!("Base: {:#X}", segment.base);
    println!("Limit: {:#X}", segment.limit);
    describe_flags(segment.flags);
    describe_access_byte(segment.access_byte);
}

fn print_base(base: u16) {
    println!(
        "{}Base: {:#010X}{}",
        driver::console::FG_BRIGHT_CYAN,
        base,
        driver::console::RESET
    );
}

fn print_limit(limit: u16) {
    println!(
        "{}Limit: {:#010X}{}",
        driver::console::FG_BRIGHT_CYAN,
        limit,
        driver::console::RESET
    );
}

pub fn print_gdt() {
    let mut gdt = [0; 2];
    unsafe {
        asm!("sgdt [{}]", in(reg) &mut gdt);
    }
    let gdt_base = gdt[1];
    let gdt_limit = gdt[0];
    print_base(gdt_base);
    print_limit(gdt_limit);

    let mut gdt_ptr = gdt_base as *const u64;
    let gdt_limit = gdt_limit / 8 + 1;

    for i in 0..gdt_limit {
        let gdt_entry = unsafe { *gdt_ptr };
        let segment = parse_segment_descriptor(gdt_entry);
        print_segment(&segment, i);
        gdt_ptr = unsafe { gdt_ptr.offset(1) };
    }
}
