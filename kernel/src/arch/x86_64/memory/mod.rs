pub mod address;
pub mod layout;
pub mod page_table;

pub use self::address::{PhysAddr, VirtAddr};

pub fn init_memory() {
    unsafe { layout::read_e820_map(PhysAddr::new(0x500)); }
}

use core::marker::PhantomData;
pub struct PhysFrame<S> {
    start_address: PhysAddr,
    size: PhantomData<S>,
}

impl<S: PageSize> PhysFrame<S> {
    pub fn containing_address(address: PhysAddr) -> Self {
        PhysFrame {
            start_address: address.align_down(S::SIZE),
            size: PhantomData
        }
    }

    pub fn start_address(&self) -> PhysAddr {
        self.start_address
    }

    pub fn size(&self) -> u64 {
        S::SIZE
    }
}

pub trait PageSize: Copy + Eq + PartialOrd + Ord {
    const SIZE: u64;
}

pub trait NotGiantPageSize: PageSize {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size4KiB {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size2MiB {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size1GiB {}

impl PageSize for Size4KiB {
    const SIZE: u64 = 4096;
}

impl NotGiantPageSize for Size4KiB {}

impl PageSize for Size2MiB {
    const SIZE: u64 = Size4KiB::SIZE * 512;
}

impl NotGiantPageSize for Size2MiB {}

impl PageSize for Size1GiB {
    const SIZE: u64 = Size2MiB::SIZE * 512;
}