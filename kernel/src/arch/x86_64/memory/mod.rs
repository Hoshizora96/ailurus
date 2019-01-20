pub mod address;
pub mod layout;

pub use self::address::{PhysAddr, VirtAddr};

pub fn init_memory() {
    unsafe { layout::read_e820_map(PhysAddr::new(0x500)); }
}