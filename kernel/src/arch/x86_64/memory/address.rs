#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct PhysAddr(u64);

impl PhysAddr {
    pub fn new(address: u64) -> Self {
        assert_eq!(address & 0xfff8_0000_0000_0000, 0,
                   "Invalid physical address: 0x{:x}", address);
        PhysAddr(address)
    }

    pub const unsafe fn new_unchecked(address: u64) -> Self {
        PhysAddr(address)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn align_down<U>(&self, align: U) -> Self
        where U: Into<u64> {
        PhysAddr::new(align_down(self.0, align.into()))
    }

    pub fn is_aligned<U>(&self, align: U) -> bool
        where U: Into<u64> {
        self.align_down(align) == *self
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn new(address: u64) -> Self {
        assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
                "Invalid virtual address: 0x{:x}", address);
        VirtAddr(address)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn align_down<U>(&self, align: U) -> Self
        where U: Into<u64> {
        VirtAddr::new(align_down(self.0, align.into()))
    }

    pub fn is_aligned<U>(&self, align: U) -> bool
        where U: Into<u64> {
        self.align_down(align) == *self
    }

    pub fn p4_index(&self) -> usize {
        ((self.0 >> 39) & 0o777) as usize
    }

    pub fn p3_index(&self) -> usize {
        ((self.0 >> 30) & 0o777) as usize
    }
    pub fn p2_index(&self) -> usize {
        ((self.0 >> 21) & 0o777) as usize
    }
    pub fn p1_index(&self) -> usize {
        ((self.0 >> 12) & 0o777) as usize
    }

    pub fn page_offset(&self) -> usize {
        (self.0 & 0o7777) as usize
    }
}

fn align_down(address: u64, align: u64) -> u64 {
    address & !(align - 1)
}
