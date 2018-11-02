#![no_std]
#![feature(asm)]
#![feature(min_const_fn)]
#![feature(const_fn)]
#![feature(naked_functions)]


#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;
mod memory;
mod arch;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[no_mangle]
pub extern fn kmain() -> ! {
    arch::interrupt::init_idt();

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

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}
