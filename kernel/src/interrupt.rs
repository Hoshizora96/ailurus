use super::segmentation::{SegmentSelector, PrivilegeLevel, get_cs};
use core::marker::PhantomData;

#[repr(C)]
pub struct Idt([IdtEntry; 256]);

/// A handler function for an interrupt or an exception without error code.
pub type HandlerFunc = extern "x86-interrupt" fn(&mut ExceptionStackFrame);
//pub type HandlerFunc = extern "x86-interrupt" fn(&mut ExceptionStackFrame);
///// A handler function for an exception that pushes an error code.
//pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(&mut ExceptionStackFrame, error_code: u64);
///// A page fault handler function that pushes a page fault error code.
//pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(&mut ExceptionStackFrame, error_code: PageFaultErrorCode);


impl Idt {
    pub fn load(&self) {
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }

    pub fn new() -> Self {
        Idt([IdtEntry::missing(); 256])
    }

    pub fn set_handler_fn(&mut self, index: usize, func: HandlerFunc) -> &mut IdtEntryOption {
        self.0[index].set_handler_and_enable(func as u64)
    }
}

pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    asm!("lidt ($0)" :: "r" (idt) : "memory");
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base: u64,
}

#[derive(Debug, Clone, Copy)]
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
    pub fn missing() -> Self {
        IdtEntry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: IdtEntryOption::minimal(),
            reserved: 0,
        }
    }

    fn set_handler_and_enable(&mut self, address: u64) -> &mut IdtEntryOption {
        self.pointer_low = address as u16;
        self.pointer_middle = (address >> 16) as u16;
        self.pointer_high = (address >> 32) as u32;
        self.gdt_selector = get_cs();
        self.options.set_present(true);
        &mut self.options
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IdtEntryOption(u16);

impl IdtEntryOption {
    fn minimal() -> Self {
        let attribute = 1 << 9 | 1 << 10 | 1 << 11;  // 'must-be-one' bits
        IdtEntryOption(attribute)
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

pub fn turn_off_interrupts() {
    unsafe {
        asm!("sti"::::"volatile");
    }
}

pub fn turn_on_interrupts() {
    unsafe {
        asm!("cli"::::"volatile");
    }
}

pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}
