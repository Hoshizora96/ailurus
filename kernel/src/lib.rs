#![no_std]
#![feature(asm)]

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;
mod memory;
mod segmentation;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[no_mangle]
pub extern fn kmain() -> ! {
    vga_buffer::WRITER.lock().clear_screen();
    println!("Started Rika-OS successfully!");
    println!("{:<20}{:<20}{:<20}", "Start Address", "End Address", "Memory Type");
    for tag in memory::iterate_memory_map(0x500) {
        println!("0x{:0>16x}  0x{:0>16x}  {:?}",
        tag.start_address(), tag.end_address(), tag.memory_type())
    }
    panic!("It should panic here!");

    loop {}
}
