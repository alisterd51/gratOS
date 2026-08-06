use crate::{
    bootprotocol,
    memory::{
        VirtAddr,
        address::{PAGE_SIZE, PhysFrame},
        allocator::HEAP_USED_BYTES,
        bitmap::BitmapAllocator,
        heap, kernel, paging,
    },
    mutex::Mutex,
    println,
};
use core::sync::atomic::Ordering;

// Physical Memory Manager
pub static PMM: Mutex<BitmapAllocator> = Mutex::new(BitmapAllocator::new());

#[allow(clippy::cast_possible_truncation)]
pub fn init() {
    {
        let mut pmm = PMM.lock();

        if let Some(memory_map) = bootprotocol::get_memory_map() {
            for memory_map_entry in memory_map {
                if memory_map_entry.memory_type() == bootprotocol::MemoryType::Memory {
                    let start_addr = memory_map_entry.base_addr;
                    let end_addr = start_addr + memory_map_entry.length;
                    let start_frame_number = start_addr.div_ceil(PhysFrame::SIZE);
                    let end_frame_number = end_addr / PhysFrame::SIZE;

                    if start_frame_number < end_frame_number {
                        for number in start_frame_number..end_frame_number {
                            pmm.free_frame(PhysFrame {
                                number: number as usize,
                            });
                        }
                    }
                }
            }
        }

        let kernel_end_frame = PhysFrame::containing_address(kernel::end());

        for number in 0..=kernel_end_frame.number {
            pmm.lock_frame(PhysFrame { number });
        }
    }

    let kernel_end_frame = PhysFrame::containing_address(kernel::end());

    for number in 0..=kernel_end_frame.number {
        let frame = PhysFrame { number };
        let phys_addr = frame.start_address();
        let virt_addr = VirtAddr(phys_addr.0 as u32);
        let flags = paging::PageTableFlags::PRESENT | paging::PageTableFlags::WRITABLE;

        unsafe { paging::map_page(virt_addr, phys_addr, flags) };
    }

    unsafe {
        paging::setup_recursive();
        paging::load_page_directory(paging::get_page_directory_address());
        paging::enable();
        paging::set_paging_active();
    };

    heap::init();
}

pub fn allocate_frame() -> Option<PhysFrame> {
    PMM.lock().allocate_frame()
}

#[allow(clippy::similar_names)]
pub fn print() {
    let start = kernel::start().0;
    let end = kernel::end().0;
    let size = kernel::size();

    let free_frames = PMM.lock().count_free_frames();
    let free_ram_kb = (free_frames * PAGE_SIZE) / 1024;
    let free_ram_mb = free_ram_kb / 1024;

    let heap_start = heap::HEAP_START;
    let heap_end = heap::get_heap_end();
    let heap_capacity_bytes = heap_end - heap_start;
    let heap_used_bytes = HEAP_USED_BYTES.load(Ordering::Relaxed);

    let dma_start = heap::DMA_START;
    let dma_end = heap::get_dma_end();
    let dma_capacity_bytes = dma_end - dma_start;
    let dma_free_bytes = heap::get_free_dma_bytes();
    let dma_used_bytes = dma_capacity_bytes - dma_free_bytes;

    println!(
        "Kernel Physical: 0x{:08X} to 0x{:08X} ({} KB)",
        start,
        end,
        size / 1024
    );
    println!(
        "Physical RAM: {} MB Free ({} free frames)",
        free_ram_mb, free_frames
    );
    println!(
        "Virtual Heap: {} Bytes in use / {} KB capacity",
        heap_used_bytes,
        heap_capacity_bytes / 1024
    );
    println!(
        "Virtual DMA: {} Bytes in use / {} KB capacity",
        dma_used_bytes,
        dma_capacity_bytes / 1024
    );
}
