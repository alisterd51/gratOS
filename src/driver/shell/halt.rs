use core::arch::asm;

#[allow(dead_code)]
pub fn halt() {
    unsafe {
        asm!("hlt",);
    }
}
