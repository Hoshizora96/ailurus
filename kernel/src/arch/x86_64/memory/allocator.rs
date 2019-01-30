use super::PhysAddr;
use super::layout::{all_memory_area, E820Type, E820Tag, MemoryAreaIter};

const PT_SIZE: usize = 4096;

struct PtAllocator {
    number: usize,
    kstart_num: usize,
    kend_num: usize,
    current_area: Option<E820Tag>,
    iterator: MemoryAreaIter
}

impl PtAllocator {
    pub fn new(kernel_start: PhysAddr, kernel_end: PhysAddr) -> Self {
        let mut allocator = PtAllocator {
            number: 0,
            kstart_num: kernel_start.as_u64() as usize / PT_SIZE,
            kend_num: kernel_end.as_u64() as usize / PT_SIZE,
            current_area: None,
            iterator: all_memory_area(),
        };
        allocator.choose_next_area();
        allocator
    }

    pub fn alloc_page(&mut self) -> PhysAddr {
        if let Some(area) = self.current_area {
            let area_end = (area.base_address.as_u64() as usize + area.size) / PT_SIZE;
            if self.number >= area_end {
                self.choose_next_area()
            }
            else if self.kstart_num <= self.number && self.number <= self.kend_num {
                self.number = self.kend_num + 1
            }
            else {
                let addr =  PhysAddr::new((self.number * PT_SIZE) as u64);
                self.number += 1;
                return addr;
            }
            self.alloc_page()
        }
        else {
            panic!("No memory to allocate page table")
        }
    }

    fn choose_next_area(&mut self) {
        self.current_area = self.iterator.clone().filter(|area| {
            let num = (area.base_address.as_u64() as usize + area.size) / PT_SIZE;
            num > self.number
        }).min_by_key(|area| area.base_address.as_u64());

        if let Some(area) = self.current_area {
            self.number = area.base_address.as_u64() as usize / PT_SIZE;
        }
    }
}