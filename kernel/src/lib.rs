#![no_std]
#![feature(asm)]

#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

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
    panic!("It should panic here!");

    loop {}
}
