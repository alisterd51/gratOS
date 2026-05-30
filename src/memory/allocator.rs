use crate::{
    memory::{PAGE_SIZE, heap::sbrk},
    mutex::Mutex,
};
use core::{
    alloc::GlobalAlloc,
    ptr::null_mut,
    sync::atomic::{AtomicUsize, Ordering},
};

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub static HEAP_USED_BYTES: AtomicUsize = AtomicUsize::new(0);

pub const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

struct ListNode {
    size: usize,
    next: Option<&'static mut Self>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        Self { size, next: None }
    }

    fn start_addr(&self) -> usize {
        core::ptr::from_ref::<Self>(self) as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    fn coalesce(&mut self) {
        let mut current = &mut self.head;

        loop {
            let merge_possible = current
                .next
                .as_ref()
                .is_some_and(|next| current.size > 0 && current.end_addr() == next.start_addr());

            if merge_possible {
                if let Some(next_node) = current.next.take() {
                    current.size += next_node.size;
                    current.next = next_node.next.take();
                }
            } else if let Some(next_node) = current.next.as_mut() {
                current = next_node;
            } else {
                break;
            }
        }
    }

    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        let aligned_addr = align_up(addr, core::mem::align_of::<ListNode>());
        let padding = aligned_addr.saturating_sub(addr);
        let aligned_size = size.saturating_sub(padding);

        if aligned_size < core::mem::size_of::<ListNode>() {
            return;
        }

        let mut node = ListNode::new(aligned_size);
        let mut current = &mut self.head;

        while current
            .next
            .as_ref()
            .is_some_and(|next| next.start_addr() < aligned_addr)
        {
            current = unsafe { current.next.as_mut().unwrap_unchecked() };
        }

        node.next = current.next.take();

        let node_ptr = aligned_addr as *mut ListNode;

        unsafe {
            node_ptr.write(node);
            current.next = Some(&mut *node_ptr);
        }

        self.coalesce();
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe { self.add_free_region(heap_start, heap_size) };
    }

    fn size_align(layout: core::alloc::Layout) -> (usize, usize) {
        let layout = layout
            .align_to(align_of::<ListNode>())
            .expect("Allocator alignment failed")
            .pad_to_align();
        let size = layout.size().max(size_of::<ListNode>());

        (size, layout.align())
    }

    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;

        if excess_size > 0 && excess_size < size_of::<ListNode>() {
            return Err(());
        }

        Ok(alloc_start)
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(region, size, align) {
                let found_node = current.next.take().expect("List inconsistency");

                current.next = found_node.next.take();

                return Some((found_node, alloc_start));
            }
            current = current.next.as_mut()?;
        }
        None
    }

    unsafe fn split_and_allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let (region, alloc_start) = self.find_region(size, align)?;
        let alloc_end = alloc_start.checked_add(size).expect("Overflow in alloc");
        let excess_size = region.end_addr() - alloc_end;

        if excess_size > 0 {
            unsafe { self.add_free_region(alloc_end, excess_size) };
        }

        Some(alloc_start as *mut u8)
    }
}

struct SlabNode {
    next: Option<&'static mut Self>,
}

fn list_index(layout: &core::alloc::Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());

    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}

pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut SlabNode>; BLOCK_SIZES.len()],
    fallback_allocator: LinkedListAllocator,
}

impl FixedSizeBlockAllocator {
    pub const fn new() -> Self {
        Self {
            list_heads: [const { None }; BLOCK_SIZES.len()],
            fallback_allocator: LinkedListAllocator::new(),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe { self.fallback_allocator.init(heap_start, heap_size) };
    }

    unsafe fn fallback_alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);

        if let Some(ptr) = unsafe { self.fallback_allocator.split_and_allocate(size, align) } {
            return ptr;
        }

        let increment = size.max(4 * PAGE_SIZE);

        if let Some((new_start, new_size)) = sbrk(increment) {
            unsafe { self.fallback_allocator.add_free_region(new_start, new_size) };
            if let Some(ptr) = unsafe { self.fallback_allocator.split_and_allocate(size, align) } {
                return ptr;
            }
        }

        null_mut()
    }
}

unsafe impl GlobalAlloc for Mutex<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = self.lock();
        let ptr = match list_index(&layout) {
            Some(index) => {
                if let Some(node) = allocator.list_heads[index].take() {
                    allocator.list_heads[index] = node.next.take();
                    core::ptr::from_mut::<SlabNode>(node).cast::<u8>()
                } else {
                    let block_size = BLOCK_SIZES[index];
                    let chunk_size = PAGE_SIZE;
                    let chunk_layout = core::alloc::Layout::from_size_align(chunk_size, block_size)
                        .expect("Invalid layout for the chunk");
                    let chunk_ptr = unsafe { allocator.fallback_alloc(chunk_layout) };

                    if chunk_ptr.is_null() {
                        return null_mut();
                    }

                    let mut current_addr = chunk_ptr as usize;
                    let end_addr = current_addr + chunk_size;
                    let user_block = current_addr;

                    current_addr += block_size;

                    while current_addr + block_size <= end_addr {
                        let node_ptr = current_addr as *mut SlabNode;
                        unsafe {
                            node_ptr.write(SlabNode {
                                next: allocator.list_heads[index].take(),
                            });
                        }
                        allocator.list_heads[index] = Some(unsafe { &mut *node_ptr });

                        current_addr += block_size;
                    }

                    user_block as *mut u8
                }
            }
            None => unsafe { allocator.fallback_alloc(layout) },
        };

        if !ptr.is_null() {
            HEAP_USED_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        }

        ptr
    }

    #[allow(clippy::cast_ptr_alignment)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        HEAP_USED_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);

        let mut allocator = self.lock();

        if let Some(index) = list_index(&layout) {
            let new_node = SlabNode {
                next: allocator.list_heads[index].take(),
            };
            let node_ptr = ptr.cast::<SlabNode>();

            unsafe { node_ptr.write(new_node) };
            allocator.list_heads[index] = Some(unsafe { &mut *node_ptr });
        } else {
            let (size, _) = LinkedListAllocator::size_align(layout);

            unsafe {
                allocator
                    .fallback_allocator
                    .add_free_region(ptr as usize, size);
            }
        }
    }
}

#[global_allocator]
pub static ALLOCATOR: Mutex<FixedSizeBlockAllocator> = Mutex::new(FixedSizeBlockAllocator::new());
