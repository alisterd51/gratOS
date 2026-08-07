use core::arch::asm;

pub unsafe fn clean_registers() {
    unsafe {
        asm!(
            "xor eax, eax",
            "xor ebx, ebx",
            "xor ecx, ecx",
            "xor edx, edx",
            "xor esi, esi",
            "xor edi, edi",
            "xor ebp, ebp",
            options(nomem, nostack, preserves_flags)
        );
    }
}
