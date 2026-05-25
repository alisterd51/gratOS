use core::arch::asm;

pub fn reboot() {
    unsafe {
        asm!("lea eax, [1b]", "push eax", "retf",);
    }
}
