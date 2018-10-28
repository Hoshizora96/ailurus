#![no_std]
#![feature(asm)]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;
mod memory;
mod segmentation;
mod interrupt;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[no_mangle]
pub extern fn kmain() -> ! {
    let mut idt = interrupt::Idt::new();
    idt.set_handler_fn(0x0, divide_by_zero_handler);
    idt.load();

    vga_buffer::WRITER.lock().clear_screen();
    println!("Started Rika-OS successfully!");
    println!("{:<20}{:<20}{:<20}", "Start Address", "End Address", "Memory Type");
    for tag in memory::iterate_memory_map(0x500) {
        println!("0x{:0>16x}  0x{:0>16x}  {:?}",
        tag.start_address(), tag.end_address(), tag.memory_type())
    }

    divide_by_zero();
    panic!("It should panic here!");

    loop {}
}

extern "x86-interrupt" fn divide_by_zero_handler(test: &mut interrupt::ExceptionStackFrame) {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}
