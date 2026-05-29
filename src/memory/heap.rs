use crate::{
    memory::{
        PAGE_SIZE, PMM, PhysAddr, PhysFrame, VirtAddr, allocate_frame,
        allocator::ALLOCATOR,
        paging::{self, PageTableFlags},
    },
    mutex::Mutex,
};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub const HEAP_START: usize = 0x2000_0000;
pub const HEAP_SIZE: usize = 4 * PAGE_SIZE;
pub const DMA_START: usize = 0x4000_0000;

static HEAP_CURRENT_END: AtomicUsize = AtomicUsize::new(HEAP_START + HEAP_SIZE);
static DMA_CURRENT_END: AtomicUsize = AtomicUsize::new(DMA_START);
static DMA_FREE_RANGES: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());

#[allow(clippy::cast_possible_truncation)]
pub fn init() {
    let heap_start = VirtAddr(HEAP_START as u32);
    let heap_end = VirtAddr((HEAP_START + HEAP_SIZE - 1) as u32);
    let start_page = heap_start.pde_index() * 1024 + heap_start.pte_index();
    let end_page = heap_end.pde_index() * 1024 + heap_end.pte_index();
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    for page in start_page..=end_page {
        let frame = allocate_frame().expect("VMM: Out of memory to initialize the heap");
        let phys_addr = frame.start_address();
        let virt_addr = VirtAddr((page * PAGE_SIZE) as u32);

        unsafe {
            paging::map_page(virt_addr, phys_addr, flags);
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn sbrk(increment: usize) -> Option<(usize, usize)> {
    if increment == 0 {
        return None;
    }

    let aligned_increment = crate::memory::allocator::align_up(increment, PAGE_SIZE);
    let old_end = HEAP_CURRENT_END.fetch_add(aligned_increment, Ordering::SeqCst);
    let new_end = old_end + aligned_increment;
    let start_page = old_end / PAGE_SIZE;
    let end_page = (new_end - 1) / PAGE_SIZE;
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    for page in start_page..=end_page {
        let frame = allocate_frame()?;
        let phys_addr = frame.start_address();
        let virt_addr = VirtAddr((page * PAGE_SIZE) as u32);

        unsafe {
            paging::map_page(virt_addr, phys_addr, flags);
        }
    }

    Some((old_end, aligned_increment))
}

#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub fn dma_alloc(size_in_bytes: usize) -> Option<(usize, u64)> {
    if size_in_bytes == 0 {
        return None;
    }

    let pages_needed = size_in_bytes.div_ceil(PAGE_SIZE);
    let bytes_needed = pages_needed * PAGE_SIZE;
    let start_frame = PMM.lock().allocate_contiguous_frames(pages_needed)?;
    let phys_base_addr = start_frame.start_address().0;
    let mut vaddr_start = {
        let mut ranges = DMA_FREE_RANGES.lock();
        let mut addr = 0;
        let mut found_idx = None;

        for (i, range) in ranges.iter_mut().enumerate() {
            if range.1 >= bytes_needed {
                addr = range.0;
                if range.1 == bytes_needed {
                    found_idx = Some(i);
                } else {
                    range.0 += bytes_needed;
                    range.1 -= bytes_needed;
                }
                break;
            }
        }

        if let Some(idx) = found_idx {
            ranges.remove(idx);
        }

        addr
    };

    if vaddr_start == 0 {
        vaddr_start = DMA_CURRENT_END.fetch_add(bytes_needed, Ordering::SeqCst);
    }

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::CACHE_DISABLE;

    for i in 0..pages_needed {
        let virt = VirtAddr((vaddr_start + i * PAGE_SIZE) as u32);
        let phys = PhysAddr(phys_base_addr + (i * PAGE_SIZE) as u64);
        unsafe {
            paging::map_page(virt, phys, flags);
        }
    }

    Some((vaddr_start, phys_base_addr))
}

fn coalesce_dma_ranges(ranges: &mut alloc::vec::Vec<(usize, usize)>) {
    if ranges.len() < 2 {
        return;
    }

    ranges.sort_unstable_by_key(|&(addr, _)| addr);

    let mut i = 0;

    while i < ranges.len() - 1 {
        let current_start = ranges[i].0;
        let current_size = ranges[i].1;
        let next_start = ranges[i + 1].0;
        let next_size = ranges[i + 1].1;

        if current_start + current_size == next_start {
            ranges[i].1 += next_size;
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub fn dma_free(virt_base_addr: usize, phys_base_addr: u64, size_in_bytes: usize) {
    if size_in_bytes == 0 {
        return;
    }

    let pages_to_free = size_in_bytes.div_ceil(PAGE_SIZE);
    let bytes_freed = pages_to_free * PAGE_SIZE;
    let start_frame_number = (phys_base_addr / PAGE_SIZE as u64) as usize;

    {
        let mut pmm = PMM.lock();
        for i in 0..pages_to_free {
            pmm.free_frame(PhysFrame {
                number: start_frame_number + i,
            });
        }
    }

    for i in 0..pages_to_free {
        unsafe {
            paging::unmap_page(VirtAddr((virt_base_addr + i * PAGE_SIZE) as u32));
        }
    }

    {
        let mut ranges = DMA_FREE_RANGES.lock();

        ranges.push((virt_base_addr, bytes_freed));
        coalesce_dma_ranges(&mut ranges);
    }
}

pub fn get_heap_end() -> usize {
    HEAP_CURRENT_END.load(Ordering::SeqCst)
}

pub fn get_dma_end() -> usize {
    DMA_CURRENT_END.load(Ordering::SeqCst)
}
