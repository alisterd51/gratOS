use crate::print;
use core::arch::asm;

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn test() {
    #[allow(unused_variables)]
    let alphabet = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    ];
    print_kernel_stack(600);
}

fn is_printable(c: u8) -> bool {
    (b' '..=b'~').contains(&c)
}

fn print_stack_line(stack: &[u8; 16], len: usize) {
    let stack = &stack[..len];
    for byte in stack {
        if is_printable(*byte) {
            print!("{}", *byte as char);
        } else {
            print!(".");
        }
    }
    print!("\n");
}

pub fn print_kernel_stack(bytes: u32) {
    let mut stack_pointer: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) stack_pointer);
    }

    let mut stack_pointer = stack_pointer as *const u8;
    let mut i = 0;
    let mut j = 0;
    let bytes = if bytes == 0 { 128 } else { bytes };
    let mut stack = [0u8; 16];

    while i < bytes {
        if j == 16 {
            print_stack_line(&stack, j);
            j = 0;
        }

        if j == 0 {
            print!("{:08x}  ", stack_pointer as u32);
        }

        print!("{:02x} ", unsafe { *stack_pointer });
        stack[j] = unsafe { *stack_pointer };

        stack_pointer = unsafe { stack_pointer.offset(1) };
        i += 1;
        j += 1;
    }

    if j != 16 {
        for _ in j..16 {
            print!("   ");
        }
    }
    print_stack_line(&stack, j);
}
