use spin::Mutex;
use super::super::platform::port::UnsafePort;

const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;
const MODE_8086: u8 = 0x01;

pub const PIC1_INTERRUPT_OFFSET: u8 = 32;
pub const PIC2_INTERRUPT_OFFSET: u8 = PIC1_INTERRUPT_OFFSET + 8;

// 8259 PIC graphic
//                      ____________                          ____________
// Real Time Clock --> |            |   Timer -------------> |            |
// ACPI -------------> |            |   Keyboard-----------> |            |      _____
// Available --------> | Secondary  |----------------------> | Primary    |     |     |
// Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
// Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
// Co-Processor -----> |            |   Parallel Port 2/3 -> |            |
// Primary ATA ------> |            |   Floppy disk -------> |            |
// Secondary ATA ----> |____________|   Parallel Port 1----> |____________|

struct Pic {
    offset: u8,
    command_port: UnsafePort<u8>,
    data_port: UnsafePort<u8>,
}

impl Pic {
    /// Are we in change of handling the specified interrupt?
    /// (Each PIC handles 8 interrupts.)
    fn handles_interrupt(&self, interupt_id: u8) -> bool {
        self.offset <= interupt_id && interupt_id < self.offset + 8
    }

    /// Notify us that an interrupt has been handled and that we're ready
    /// for more.
    unsafe fn end_of_interrupt(&mut self) {
        self.command_port.write(CMD_END_OF_INTERRUPT);
    }
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command_port: UnsafePort::new(0x20),
                    data_port: UnsafePort::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command_port: UnsafePort::new(0xA0),
                    data_port: UnsafePort::new(0xA1),
                },
            ]
        }
    }

    pub unsafe fn initialize(&mut self) {
        let mut wait_port: UnsafePort<u8> = UnsafePort::new(0x80);
        let mut wait = || { wait_port.write(0) };

        let saved_mask1 = self.pics[0].data_port.read();
        let saved_mask2 = self.pics[1].data_port.read();

        // Tell each pic that we're going to send three-bytes initialization sequence.
        self.pics[0].command_port.write(CMD_INIT);
        wait();
        self.pics[1].command_port.write(CMD_INIT);
        wait();

        // Byte 1: Set up base offset.
        self.pics[0].data_port.write(self.pics[0].offset);
        wait();
        self.pics[1].data_port.write(self.pics[1].offset);
        wait();

        // Byte 2: Configure chaining between PIC1 and PIC2.
        self.pics[0].data_port.write(4);
        wait();
        self.pics[1].data_port.write(2);
        wait();

        // Bytes 3: Set operation mode
        self.pics[0].data_port.write(MODE_8086);
        wait();
        self.pics[1].data_port.write(MODE_8086);
        wait();

        self.pics[0].data_port.write(saved_mask1);
        self.pics[1].data_port.write(saved_mask2);
    }

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }
}


pub static PIC_8259: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe {
        ChainedPics::new(PIC1_INTERRUPT_OFFSET, PIC2_INTERRUPT_OFFSET)
    });
