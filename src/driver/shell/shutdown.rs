use crate::io::outw;
use core::arch::asm;

pub fn qemu() {
    unsafe {
        asm!("cli",);
        outw(0x2000, 0x604);
    }
}
