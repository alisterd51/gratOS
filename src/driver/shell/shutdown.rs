use crate::io::outw;
use core::arch::asm;

#[allow(dead_code)]
pub fn shutdown_qemu() {
    unsafe {
        asm!("cli",);
        outw(0x2000, 0x604);
    }
}
