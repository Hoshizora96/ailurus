use core::mem::size_of;
use super::PhysAddr;

const E820_MAX: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum E820Type {
    None,
    Free,
    Reserved,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub struct E820Tag {
    pub base_address: PhysAddr,
    pub size: usize,
    pub mem_type: E820Type,
}

impl E820Tag {
    const fn missing() -> Self {
        E820Tag {
            base_address: unsafe{ PhysAddr::new_unchecked(0) },
            size: 0,
            mem_type: E820Type::None
        }
    }

    pub fn is_free(&self) -> bool {
        self.mem_type == E820Type::Free
    }
}

static mut MEMORY_AREA_NUM: usize = 0;
static mut MEMORY_SIZE: usize = 0;
static mut E820_MAP: [E820Tag; E820_MAX] = [E820Tag::missing(); E820_MAX];

pub unsafe fn read_e820_map(address: PhysAddr) {
    #[repr(C)]
    struct RawE820Tag {
        base_address: u64,
        size: u64,
        mem_type: u32,
        _reversed: u32,
    }
    let mut last_tag_addr = address.as_u64();
    loop {
        let tag = &mut *(last_tag_addr as *mut RawE820Tag);
        last_tag_addr += size_of::<RawE820Tag>() as u64;
        if tag.mem_type == 0 {
            break
        } else {
            MEMORY_SIZE += tag.size as usize;
            E820_MAP[MEMORY_AREA_NUM] = E820Tag {
                base_address: PhysAddr::new(tag.base_address),
                size: tag.size as usize,
                mem_type: match tag.mem_type {
                    0 => E820Type::None,
                    1 => E820Type::Free,
                    2 => E820Type::Reserved,
                    _ => E820Type::Unknown
                }
            };
            MEMORY_AREA_NUM += 1;
        }
        assert_ne!(MEMORY_AREA_NUM, E820_MAX, "Incorrect E820 map")
    }
}

pub fn physical_memory_size() -> usize {
    unsafe { MEMORY_SIZE }
}

pub fn memory_area_num() -> usize {
    unsafe { MEMORY_AREA_NUM }
}

pub fn all_memory_area() -> MemoryAreaIter {
    MemoryAreaIter {
        loc: 0
    }
}

#[derive(Debug, Clone)]
pub struct MemoryAreaIter {
    loc: usize
}

impl Iterator for MemoryAreaIter {
    type Item = E820Tag;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.loc >= unsafe { MEMORY_AREA_NUM } {
            None
        } else {
            let tag = unsafe { E820_MAP[self.loc] };
            self.loc += 1;
            Some(tag)
        }
    }
}

