#[macro_use]
pub mod vga_buffer;
pub mod pic;

pub unsafe fn init_pics() {
    use self::pic::PIC_8259;
    PIC_8259.lock().initialize();
}