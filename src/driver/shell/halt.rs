use core::arch::asm;

pub fn halt() {
    unsafe {
        asm!("hlt",);
    }
}
