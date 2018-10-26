#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    AddressRangeMemory,
    AddressRangeReserved,
    Unknown
}

#[repr(C)]
pub struct MemoryTag {
    start_address: u64,
    length: u64,
    mem_type: u32,
    _reversed: u32,
}

impl MemoryTag {
    pub fn start_address(&self) -> u64 {
        self.start_address
    }

    pub fn end_address(&self) -> u64 {
        self.start_address + self.length
    }

    pub fn memory_type(&self) -> MemoryType {
        match self.mem_type {
            1 => MemoryType::AddressRangeMemory,
            2 => MemoryType::AddressRangeReserved,
            _ => MemoryType::Unknown
        }
    }
}

pub struct MemoryMapIter {
    cur_tag_address: u64,
    tag_size: u64,
}

impl Iterator for MemoryMapIter {
    type Item = &'static MemoryTag;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // TODO: Should do stricter check of memory_map.
        let next_tag_addr = (self.cur_tag_address + self.tag_size);
        if unsafe {
            *(next_tag_addr as *mut u64) < *(self.cur_tag_address as *mut u64)
        } {
            None
        } else {
            self.cur_tag_address = next_tag_addr;
            let tag = unsafe {
                &mut *(self.cur_tag_address as *mut MemoryTag
                )
            };
            Some(tag)
        }
    }
}

pub fn iterate_memory_map(start_address: u64) -> MemoryMapIter {
    // TODO: let start_address be a constant(0x500-0x5000).
    MemoryMapIter {
        cur_tag_address: start_address,
        tag_size: 24,  // size of ARDS
    }
}