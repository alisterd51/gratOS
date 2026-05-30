use crate::{io::outw, power::halt};
use core::arch::asm;

pub fn qemu() -> ! {
    unsafe {
        asm!("cli", options(nomem, nostack, preserves_flags));
        outw(0x2000, 0x604);
    }
    halt::halt();
}
