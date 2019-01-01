#[macro_use]
pub mod vga_buffer;
pub mod pic;
pub mod cpu;

pub fn init_devices() {
    unsafe { pic::PIC_8259.lock().initialize(); }
}