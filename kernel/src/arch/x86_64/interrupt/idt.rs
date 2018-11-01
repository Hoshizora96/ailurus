use super::super::segmentation::{SegmentSelector, PrivilegeLevel, get_cs};
use super::handler;

pub const INT_COUNT: usize = 256;

pub type HandlerFunc = unsafe extern fn();
pub type Idt = [IdtEntry; INT_COUNT];

pub static mut IDT: Idt = [IdtEntry::missing(); INT_COUNT];

pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    asm!("lidt ($0)" :: "r" (idt) : "memory");
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base: u64,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: IdtEntryOption,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        IdtEntry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: IdtEntryOption::minimal(),
            reserved: 0,
        }
    }

    pub fn set_handler_fn(&mut self, handler: HandlerFunc) -> &mut IdtEntryOption {
        let address = handler as u64;
        self.pointer_low = address as u16;
        self.pointer_middle = (address >> 16) as u16;
        self.pointer_high = (address >> 32) as u32;
        self.gdt_selector = get_cs();
        // The compiler told me that borrow of packed field is unsafe and requires unsafe function
        // or block, so I add one, though there's no need to worry about it.
        unsafe {
            self.options.set_present(true);
            &mut self.options
        }
    }
}

#[derive(Clone, Copy)]
pub struct IdtEntryOption(u16);

impl IdtEntryOption {
    const fn minimal() -> Self {
        IdtEntryOption(1 << 9 | 1 << 10 | 1 << 11)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).enable_interrupt(false);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        if present {
            self.0 |= 1 << 15;
        } else {
            self.0 &= 0x7fff;
        }
        self
    }

    pub fn enable_interrupt(&mut self, enable: bool) -> &mut Self {
        if enable {
            self.0 |= 1 << 8;
        } else {
            self.0 &= 0xfeff;
        }
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0 = (self.0 & 0x9fff) | dpl;
        self
    }

    pub fn set_stack_index(&mut self, stack_index: u16) -> &mut Self {
        self.0 = (self.0 & 0xfff8) | stack_index;
        self
    }
}

