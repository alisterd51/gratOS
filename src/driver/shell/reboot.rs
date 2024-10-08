use core::arch::asm;

#[allow(dead_code)]
pub fn reboot() {
    unsafe {
        asm!("lea eax, [1b]", "push eax", "retf",);
    }
}
