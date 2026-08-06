pub const PAGE_SIZE: usize = 4096;

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
