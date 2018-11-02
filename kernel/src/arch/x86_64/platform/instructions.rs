#![allow(unused_variables)]

pub unsafe fn sti() {
    asm!("sti"::::"volatile");
}

pub unsafe fn cli() {
    asm!("cli"::::"volatile");
}

pub unsafe fn hlt() {
    asm!("hlt"::::"volatile");
}


// Instructions for port io
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outb(port: u16, value: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outw(port: u16, value: u16) {
    asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}

pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outl(port: u16, value: u32) {
    asm!("outw %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}