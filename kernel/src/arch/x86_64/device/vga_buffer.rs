#[allow(dead_code)]

use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row_position: usize,
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                let row = self.row_position;
                let col = self.column_position;
                self.buffer.chars[row][col] = ScreenChar{
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;

                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }
            }
        }
        self.refresh_cursor();
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7f | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn clear_screen(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col] = blank;
            }
        }
        self.column_position = 0;
        self.row_position = 0;
        self.refresh_cursor();
    }

    fn new_line(&mut self) {
        self.row_position += 1;
        if self.row_position >= BUFFER_HEIGHT {
            self.scroll_screen();
            self.row_position = BUFFER_HEIGHT - 1;
            self.clear_line();
        }
        else {
            self.clear_line();
        }
        self.column_position = 0;
    }

    fn scroll_screen(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
    }

    fn clear_line(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[self.row_position][col] = blank;
        }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn refresh_cursor(&self) {
        let index : u16 = self.row_position as u16 * BUFFER_WIDTH as u16 + self.column_position as u16;
        unsafe {
            asm!("push rax\n\
                  push rbx\n\
                  push rdx\n
                  mov dx,0x03d4\n
                  mov al,0x0e\n
                  out dx,al\n
                  inc dx\n
                  mov al,bh\n
                  out dx,al\n
                  dec dx\n
                  mov al,0xf\n
                  out dx,al\n
                  inc dx\n
                  mov al,bl\n
                  out dx,al\n
                  pop rdx\n
                  pop rbx\n
                  pop rax\n"
                 : :"{bx}"(index): : "intel")
        }
    }
}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_position: 0,
        column_position: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::x86_64::device::vga_buffer::print(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    use super::super::interrupt;
    interrupt::run_without_interrupt(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}