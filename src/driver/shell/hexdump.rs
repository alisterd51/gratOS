use crate::{memory::paging::is_page_mapped, print, println};
use core::ptr::read_volatile;

const fn is_printable(c: u8) -> bool {
    matches!(c, b' '..=b'~')
}

#[allow(clippy::cast_possible_truncation)]
fn safe_read_byte(addr: usize) -> Option<u8> {
    let virt_addr = crate::memory::VirtAddr(addr as u32);
    if is_page_mapped(virt_addr) {
        Some(unsafe { read_volatile(addr as *const u8) })
    } else {
        None
    }
}

fn print_hex(data: &[Option<u8>]) {
    for &byte_opt in data {
        if let Some(byte) = byte_opt {
            print!("{:02x} ", byte);
        } else {
            print!("?? ");
        }
    }
    let padding = 16usize.saturating_sub(data.len());
    for _ in 0..padding {
        print!("   ");
    }
}

fn print_ascii(data: &[Option<u8>]) {
    print!("|");
    for &byte_opt in data {
        let c = match byte_opt {
            Some(byte) if is_printable(byte) => byte as char,
            Some(_) => '.',
            None => '?',
        };
        print!("{c}");
    }
    print!("|");
}

#[allow(clippy::cast_possible_truncation)]
pub fn hexdump(start_addr: usize, bytes: usize) {
    if bytes == 0 {
        return;
    }

    let mut previous_chunk: Option<([Option<u8>; 16], usize)> = None;
    let mut star_printed = false;

    for offset in (0..bytes).step_by(16) {
        let current_addr = start_addr + offset;
        let chunk_size = (bytes - offset).min(16);
        let mut current_chunk = [None; 16];

        for (i, slot) in current_chunk.iter_mut().take(chunk_size).enumerate() {
            *slot = safe_read_byte(current_addr + i);
        }

        let active_chunk = &current_chunk[..chunk_size];
        let is_duplicate = previous_chunk.is_some_and(|(prev_arr, prev_size)| {
            prev_size == chunk_size && active_chunk == &prev_arr[..chunk_size]
        });

        if is_duplicate {
            if !star_printed {
                println!("*");
                star_printed = true;
            }
        } else {
            print!("{:08x}  ", current_addr);
            print_hex(active_chunk);
            print_ascii(active_chunk);
            println!();
            star_printed = false;
            previous_chunk = Some((current_chunk, chunk_size));
        }
    }

    let end_addr = start_addr + bytes;

    println!("{:08x}", end_addr);
}
