use crate::println;
use core::arch::asm;

struct SegmentDescriptor {
    base: u32,
    limit: u32,
    flags: u8,
    access_byte: u8,
}

fn parse_segment_descriptor(descriptor: u64) -> SegmentDescriptor {
    let base = ((descriptor >> 16) & 0xFFFF) as u32
        | (((descriptor >> 32) & 0xFF) as u32) << 16
        | (((descriptor >> 56) & 0xFF) as u32) << 24;

    let limit = ((descriptor & 0xFFFF) as u32) | (((descriptor >> 48) & 0xF) as u32) << 16;

    let flags = ((descriptor >> 52) & 0xF) as u8;

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

fn describe_access_byte(access_byte: u8) {
    let accessed = if (access_byte & 0b0000_0001) != 0 {
        "Yes"
    } else {
        "No"
    };
    let readable_or_writable = if (access_byte & 0b000_0010) != 0 {
        "Yes"
    } else {
        "No"
    };
    let direction_or_conforming = if (access_byte & 0b0000_0100) != 0 {
        "Yes"
    } else {
        "No"
    };
    let executable = if (access_byte & 0b0000_1000) != 0 {
        "Yes"
    } else {
        "No"
    };
    let descriptor_type = if (access_byte & 0b0001_0000) != 0 {
        "Code/Data"
    } else {
        "System"
    };
    let dpl = (access_byte & 0b0110_0000) >> 5;
    let present = if (access_byte & 0b1000_0000) != 0 {
        "Yes"
    } else {
        "No"
    };

    println!(
        "+------------+-------------------+--------------------+-------------+\n\
         | Accessed | Readable/Writable | Direction/Conforming  | Executable |\n\
         +------------+-------------------+--------------------+-------------+\n\
         | {:<8} | {:<17} | {:<21} | {:<10} |\n\
         +------------+-------------------+--------------------+-------------+\n\
         +----------------+----------------+-----+---------+\n\
         | Descriptor Type | DPL            |     Present  |\n\
         +----------------+----------------+-----+---------+\n\
         | {:<15} | {:<14} | {:<12} |\n\
         +----------------+----------------+-----+---------+",
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
    let blue_fg = "\x1B[94m";
    let reset = "\x1B[0m";
    println!(
        "{}+-------------------------+\n\
             |       Segment {:<9} |\n\
             +-------------------------+{}",
        blue_fg, index, reset
    );
    println!("Base: {:#X}", segment.base);
    println!("Limit: {:#X}", segment.limit);
    describe_flags(segment.flags);
    describe_access_byte(segment.access_byte);
}

fn print_base(base: u16) {
    let bright_cyan = "\x1B[96m";
    let reset = "\x1B[0m";

    println!("{}Base: {:#010X}{}", bright_cyan, base, reset);
}

fn print_limit(limit: u16) {
    let bright_cyan = "\x1B[96m";
    let reset = "\x1B[0m";

    println!("{}Limit: {:#010X}{}", bright_cyan, limit, reset);
}

pub fn print_gdt() {
    let mut gdt: [u16; 2] = [0; 2];
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
