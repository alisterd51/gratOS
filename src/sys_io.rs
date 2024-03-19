use core::arch::asm;

/// Read a one byte data from port
pub unsafe fn inb(port: u16) -> u8 {
    let value;
    
    asm!(
        "inb %dx, %al",
        in("dx") port,
        out("al") value,
        options(att_syntax)
    );
    value
}

/// Write a one byte data to port
pub unsafe fn outb(value: u8, port: u16) {
    asm!(
        "outb %al, %dx",
        in("dx") port,
        in("al") value,
        options(att_syntax)
    );
}
