#![no_std]
#![feature(asm)]
#![feature(min_const_fn)]
#![feature(const_fn)]
#![feature(naked_functions)]


#[macro_use]
extern crate lazy_static;

extern crate spin;

use core::panic::PanicInfo;

#[macro_use]
mod arch;

pub use self::arch::kstart;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

pub fn kmain() -> ! {
    println!("Started Ailurus-OS successfully!");

    print_memory_map();

    // divide_by_zero();
    panic!("It should panic here!");

    loop {}
}

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}

fn print_memory_map() {
    println!("Memory map:");
    println!("{:<20}{:<20}{:<20}", "Start Address", "Size", "Memory Type");
    for tag in arch::memory::layout::all_memory_area() {
        println!("0x{:0>16X}  0x{:0>16X}  {:?}",
                 tag.base_address.as_u64(), tag.size, tag.mem_type)
    }

    let mut total_mem_size = arch::memory::layout::physical_memory_size();
    println!("Total memory size: {}MB", total_mem_size / 1024 / 1024)
}