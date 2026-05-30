use crate::{
    driver::shell::hexdump::hexdump,
    memory::heap::{self, dma_alloc, dma_free},
    mutex::Mutex,
    println,
};
use alloc::{boxed::Box, vec::Vec};
use core::ptr::write_bytes;

static HEAP_ALLOCS: Mutex<Vec<Box<[u8]>>> = Mutex::new(Vec::new());
static DMA_ALLOCS: Mutex<Vec<(usize, u64, usize)>> = Mutex::new(Vec::new());

pub fn alloc_heap(size: usize) {
    let vec = alloc::vec![b'A'; size];
    let boxed_slice = vec.into_boxed_slice();

    HEAP_ALLOCS.lock().push(boxed_slice);
}

pub fn free_heap() {
    HEAP_ALLOCS.lock().pop();
}

pub fn alloc_dma(size: usize) {
    if let Some((virt_base_addr, phys_base_addr)) = dma_alloc(size) {
        unsafe {
            write_bytes(virt_base_addr as *mut u8, b'B', size);
        }

        DMA_ALLOCS
            .lock()
            .push((virt_base_addr, phys_base_addr, size));
    }
}

pub fn free_dma() {
    if let Some((virt_base_addr, phys_base_addr, size)) = DMA_ALLOCS.lock().pop() {
        dma_free(virt_base_addr, phys_base_addr, size);
    }
}

pub fn print_stack(bytes: usize) {
    let stack_pointer: usize;

    unsafe {
        core::arch::asm!("mov {}, esp", out(reg) stack_pointer);
    }

    let safe_bytes = bytes.min(1024);

    hexdump(stack_pointer, safe_bytes);
}

pub fn print_heap(bytes: usize) {
    let heap_start = heap::HEAP_START;
    let heap_end = heap::get_heap_end();
    let current_size = heap_end - heap_start;

    if current_size == 0 {
        println!("Heap is currently empty.");
        return;
    }

    let safe_bytes = bytes.min(current_size);

    hexdump(heap_start, safe_bytes);
}

pub fn print_dma(bytes: usize) {
    let dma_start = heap::DMA_START;
    let dma_end = heap::get_dma_end();
    let current_size = dma_end - dma_start;

    if current_size == 0 {
        println!("DMA space is untouched.");
        return;
    }

    let safe_bytes = bytes.min(current_size);

    hexdump(dma_start, safe_bytes);
}

pub fn print_vga(lines: usize) {
    let vga_start = 0xB8000;
    let max_vga_size = 4000; // 80 * 25 * 2
    let bytes = (lines * 160).min(max_vga_size);

    hexdump(vga_start, bytes);
}
