use crate::{print, println};
use core::{arch::asm, slice};

pub fn test() {
    #[allow(unused_variables)]
    let alphabet = *b"ABCDEFGHIJKLMNO";
    print_kernel_stack(6000);
}

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
    for &byte in data {
        let c = if is_printable(byte) {
            byte as char
        } else {
            '.'
        };
        print!("{c}");
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn print_kernel_stack(bytes: u32) {
    let stack_pointer: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) stack_pointer);
    }
    let len = if bytes == 0 { 128 } else { bytes as usize };
    let stack_slice = unsafe { slice::from_raw_parts(stack_pointer as *const u8, len) };
    for (i, chunk) in stack_slice.chunks(16).enumerate() {
        let current_addr = stack_pointer + (i * 16) as u32;
        print!("{:08x}  ", current_addr);
        print_hex(chunk);
        print_ascii(chunk);
        println!();
    }
}
