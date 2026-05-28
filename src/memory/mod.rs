use crate::{bootprotocol, memory::bitmap::BitmapAllocator, mutex::Mutex, println};
use core::ptr::addr_of;

mod allocator;
mod bitmap;
mod heap;
mod paging;

pub const PAGE_SIZE: usize = 4096;

// Physical Memory Manager
static PMM: Mutex<BitmapAllocator> = Mutex::new(BitmapAllocator::new());

#[derive(Clone, Copy)]
pub struct PhysAddr(pub u64);

#[derive(Clone, Copy)]
pub struct PhysFrame {
    pub number: usize,
}

impl PhysFrame {
    pub const SIZE: u64 = PAGE_SIZE as u64;

    pub const fn containing_address(address: PhysAddr) -> Self {
        Self {
            number: (address.0 / Self::SIZE) as usize,
        }
    }

    #[allow(dead_code)]
    pub const fn start_address(self) -> PhysAddr {
        PhysAddr((self.number as u64) * Self::SIZE)
    }
}

#[derive(Clone, Copy)]
pub struct VirtAddr(pub u32);

impl VirtAddr {
    pub const fn pde_index(self) -> usize {
        (self.0 >> 22) as usize
    }

    pub const fn pte_index(self) -> usize {
        ((self.0 >> 12) & 0x3FF) as usize
    }

    #[allow(clippy::cast_possible_truncation)]
    pub const fn page_table_vaddr(self) -> Self {
        Self(0xFFC0_0000 + (self.pde_index() as u32 * PAGE_SIZE as u32))
    }

    pub const fn page_directory_vaddr() -> Self {
        Self(0xFFFF_F000)
    }
}

unsafe extern "C" {
    static _kernel_start: u8;
    static _kernel_end: u8;
}

fn kernel_start() -> PhysAddr {
    PhysAddr(addr_of!(_kernel_start) as u64)
}

fn kernel_end() -> PhysAddr {
    PhysAddr(addr_of!(_kernel_end) as u64)
}

fn kernel_size() -> u64 {
    kernel_end().0 - kernel_start().0
}

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

        let kernel_end_frame = PhysFrame::containing_address(kernel_end());

        for number in 0..=kernel_end_frame.number {
            pmm.lock_frame(PhysFrame { number });
        }
    }

    let kernel_end_frame = PhysFrame::containing_address(kernel_end());

    for number in 0..=kernel_end_frame.number {
        let frame = PhysFrame { number };
        let phys_addr = frame.start_address();
        let virt_addr = VirtAddr(phys_addr.0 as u32);
        let flags = paging::PageTableFlags::PRESENT | paging::PageTableFlags::WRITABLE;

        unsafe {
            paging::map_page(virt_addr, phys_addr, flags);
        }
    }

    unsafe {
        paging::setup_recursive_paging();
        paging::load_page_directory(paging::get_page_directory_address());
        paging::enable_paging();
        paging::set_paging_active();
    }

    heap::init();
}

#[allow(dead_code)]
pub fn allocate_frame() -> Option<PhysFrame> {
    PMM.lock().allocate_frame()
}

pub fn print() {
    let start = kernel_start().0;
    let end = kernel_end().0;
    let size = kernel_size();

    println!("kernel: start: {start}, end: {end}, size: {size}");
}
