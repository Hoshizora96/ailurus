#[macro_use]
pub mod device;

pub mod memory;
pub mod interrupt;
pub mod platform;

#[repr(packed)]
pub struct KernelArgs {
    kernel_base: u64,
    kernel_size: u64,
    stack_base: u64,
    stack_size: u64,
    env_base: u64,
    env_size: u64,
}


#[no_mangle]
pub extern fn kstart(kernel_args: &KernelArgs) {
    device::init_devices(); 
    interrupt::init_idt();
    unsafe { platform::instructions::sti();}
    

    device::vga_buffer::WRITER.lock().clear_screen();
    super::super::kmain();
}