impl_handler!(divide_by_zero, frame, {
    dump_interrupt_info!("DIVIDE BY ZERO", frame);
    loop{}
});

use super::super::device::pic::PIC_8259;
use super::super::platform::port::UnsafePort;
impl_handler!(timer, frame, {
//    print!(".");
    PIC_8259.lock().notify_end_of_interrupt(32);
});

use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};
use spin::Mutex;
lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
}

impl_handler!(keyboard, frame, {
    unsafe {
        let mut port = UnsafePort::new(0x60);
        let scancode: u8 = port.read();

        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
    PIC_8259.lock().notify_end_of_interrupt(33);
});

