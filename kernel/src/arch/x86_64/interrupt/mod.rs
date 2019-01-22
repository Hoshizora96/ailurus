#[macro_use]
pub mod util;
pub mod handler;
pub mod idt;

pub fn init_idt() {
    unsafe {
        use self::idt::{lidt, DescriptorTablePointer, Idt, IDT};
        use core::mem::size_of;
        IDT[0].set_handler_fn(handler::divide_by_zero);

        IDT[32].set_handler_fn(handler::timer);
        IDT[33].set_handler_fn(handler::keyboard);

        let ptr = DescriptorTablePointer {
            base: &IDT as *const _ as u64,
            limit: (size_of::<Idt>() - 1) as u16,
        };

        lidt(&ptr);
    }
}

pub fn run_without_interrupt<F>(p: F) where F: Fn() {
    use super::platform::instructions;
    unsafe {
        instructions::cli();
        p();
        instructions::sti();
    }
}