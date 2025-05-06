use crate::{print, println};
use core::arch::asm;

pub fn test() {
    #[allow(unused_variables)]
    let alphabet = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    ];
    print_kernel_stack(6000);
}

// wait `contains` in `const fn` <https://github.com/rust-lang/rust/issues/67792>
fn is_printable(c: u8) -> bool {
    (b' '..=b'~').contains(&c)
}

fn print_hex(stack: &[u8; 16], len: usize) {
    (0..16).for_each(|i| {
        if i < len {
            print!("{:02x} ", stack[i]);
        } else {
            print!("   ");
        }
    });
}

fn print_ascii(stack: &[u8; 16], len: usize) {
    stack[..len].iter().for_each(|&byte| {
        print!(
            "{}",
            if is_printable(byte) {
                byte as char
            } else {
                '.'
            }
        );
    });
}

#[allow(clippy::cast_possible_truncation)]
pub fn print_kernel_stack(bytes: u32) {
    let mut stack_pointer: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) stack_pointer);
    }

    let mut stack_pointer = stack_pointer as *const u8;
    let bytes = if bytes == 0 { 128 } else { bytes };
    let mut remaining = bytes;

    while remaining > 0 {
        let count = if remaining >= 16 {
            16
        } else {
            remaining as usize
        };
        let mut line = [0u8; 16];
        line[..count].iter_mut().enumerate().for_each(|(i, byte)| {
            *byte = unsafe { *stack_pointer.add(i) };
        });
        print!("{:08x}  ", stack_pointer as u32);
        print_hex(&line, count);
        print_ascii(&line, count);
        println!();
        stack_pointer = unsafe { stack_pointer.add(count) };
        remaining -= count as u32;
    }
}
