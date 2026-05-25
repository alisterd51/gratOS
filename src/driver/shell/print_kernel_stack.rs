use crate::{print, println};
use core::{arch::asm, slice};

const fn is_printable(c: u8) -> bool {
    matches!(c, b' '..=b'~')
}

fn print_hex(data: &[u8]) {
    for &byte in data {
        print!("{:02x} ", byte);
    }
    let padding = 16usize.saturating_sub(data.len());
    for _ in 0..padding {
        print!("   ");
    }
}

fn print_ascii(data: &[u8]) {
    print!("|");
    for &byte in data {
        let c = if is_printable(byte) {
            byte as char
        } else {
            '.'
        };
        print!("{c}");
    }
    print!("|");
}

#[allow(clippy::cast_possible_truncation)]
pub fn print_kernel_stack(bytes: u32) {
    let stack_pointer: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) stack_pointer);
    }
    let len = if bytes == 0 { 128 } else { bytes as usize };
    let stack_slice = unsafe { slice::from_raw_parts(stack_pointer as *const u8, len) };
    let mut previous_chunk: Option<&[u8]> = None;
    let mut star_printed = false;
    for (i, chunk) in stack_slice.chunks(16).enumerate() {
        let current_addr = stack_pointer + (i * 16) as u32;

        if Some(chunk) == previous_chunk {
            if !star_printed {
                println!("*");
                star_printed = true;
            }
        } else {
            print!("{:08x}  ", current_addr);
            print_hex(chunk);
            print_ascii(chunk);
            println!();
            star_printed = false;
            previous_chunk = Some(chunk);
        }
    }
    let end_addr = stack_pointer + len as u32;
    println!("{:08x}", end_addr);
}
