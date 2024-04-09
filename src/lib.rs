#![no_std]
#![no_main]

mod driver;
mod io;

use core::panic::PanicInfo;
use driver::vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[allow(clippy::empty_loop)]
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    vga::text_mode::clear();

    println!("42");

    loop {}
}
