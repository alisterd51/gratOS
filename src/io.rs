use core::arch::asm;

/// Read a one byte data from port
#[allow(dead_code)]
pub unsafe fn inb(port: u16) -> u8 {
    let value;
    
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port
    );
    value
}

/// Write a one byte data to port
pub unsafe fn outb(value: u8, port: u16) {
    asm!(
        "out dx, al",
        in("al") value,
        in("dx") port
    );
}
