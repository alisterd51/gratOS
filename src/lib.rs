#![no_std]
#![no_main]

mod vga_buffer;
mod io;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    vga_buffer::WRITER.lock().clear();

    println!("42");

    loop {}
}
