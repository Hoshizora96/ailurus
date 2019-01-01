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

// It will fail to execute when CPU does not support `cpuid`, so this function is unsafe.
pub unsafe fn cpuid(eax: u32) -> (u32, u32, u32) {
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    asm!("cpuid"
        : "={ebx}"(ebx), "={ecx}"(ecx), "={edx}"(edx)
        : "{eax}"(eax)
        ::"volatile", "intel");
    (ebx, ecx, edx)
}

// It works when CPUID.01H:EDX[5]=1
pub unsafe fn rdmsr(ecx: u32) -> (u32, u32) {
    let eax: u32;
    let edx: u32;
    asm!("rdmsr"
        : "={eax}"(eax), "={edx}"(edx)
        : "{ecx}"(ecx)
        ::"volatile", "intel");
    (eax, edx)
}

pub unsafe fn wrmsr(eax: u32, edx: u32, ecx: u32) {
    asm!("wrmsr"
        :: "{ecx}"(ecx),"{eax}"(eax), "{edx}"(edx)
        ::"volatile", "intel");
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