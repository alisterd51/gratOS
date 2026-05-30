use crate::{io::outb, power::halt};
use core::arch::asm;

pub fn reboot() -> ! {
    unsafe {
        asm!("cli", options(nomem, nostack, preserves_flags));
        outb(0xFE, 0x64);
    }
    halt::halt();
}
