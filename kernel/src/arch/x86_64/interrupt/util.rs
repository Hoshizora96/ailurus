#![allow(unused_macros)]
macro_rules! context_push {
    () => (asm!(
        "push r15
         push r14
         push r13
         push r12
         push r11
         push r10
         push r9
         push r8
         push rbp
         push rdi
         push rsi
         push rdx
         push rcx
         push rbx
         push rax"
        : : : : "intel", "volatile"
    ));
}

macro_rules! context_pop {
    () => (asm!(
        "pop rax
         pop rbx
         pop rcx
         pop rdx
         pop rsi
         pop rdi
         pop rbp
         pop r8
         pop r9
         pop r10
         pop r11
         pop r12
         pop r13
         pop r14
         pop r15"
        : : : : "intel", "volatile"
    ));
}

macro_rules! iret {
    () => (asm!(
        "iretq"
        : : : : "intel", "volatile"
    ));
}

macro_rules! error_code_pop {
    () => {asm!(
        "add rsp, 8"
        : : : : "intel", "volatile"
    )};
}


macro_rules! fs_push {
    () => (asm!(
        "push fs
        mov rax, 0x18
        mov fs, ax"
        : : : : "intel", "volatile"
    ));
}

macro_rules! fs_pop {
    () => (asm!(
        "pop fs"
        : : : : "intel", "volatile"
    ));
}

#[repr(packed)]
pub struct ContextRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

#[repr(packed)]
pub struct IretRegisters {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[repr(packed)]
pub struct InterruptFrame {
    pub context_registers: ContextRegisters,
    pub iret_registers: IretRegisters,
}

#[repr(packed)]
pub struct InterruptFrameWithErrorCode {
    pub context_registers: ContextRegisters,
    pub error_code: u64,
    pub iret_registers: IretRegisters,
}

macro_rules! dump_interrupt_info {
    ($name: expr, $frame: ident) => {
        println!("\n{:-^78}", "EXCEPTION OCCURRED!");
        println!("EXCEPTION: {}", $name);
        println!("INSTRUCTION POINTER: 0x{:0>16X}", $frame.iret_registers.rip);
        dump_registers!($frame);
        println!("{:-^78}", "END OF DUMP");
    }
}

macro_rules! dump_interrupt_info_with_error_code {
    ($name: expr, $frame: ident) => {
        println!("\n{:-^78}", "EXCEPTION OCCURRED!");
        println!("EXCEPTION: {}", $name);
        println!("INSTRUCTION POINTER: 0x{:0>16X}", $frame.iret_registers.rip);
        dump_registers!($frame);
        println!("ERROR CODE: 0x{:X}", $frame.error_code);
        println!("{:-^78}", "END OF DUMP");
    }
}

macro_rules! dump_registers {
    ($frame: ident) => {
        println!("REGISTERS:");
        println!("  RAX: 0x{:0>16X}  RBX: 0x{:0>16X}  RCX: 0x{:0>16X}",
        $frame.context_registers.rax, $frame.context_registers.rbx, $frame.context_registers.rcx);
        println!("  RDX: 0x{:0>16X}  RSI: 0x{:0>16X}  RDI: 0x{:0>16X}",
        $frame.context_registers.rdx, $frame.context_registers.rsi, $frame.context_registers.rdi);
        println!("  R8:  0x{:0>16X}  R9:  0x{:0>16X}  R10: 0x{:0>16X}",
        $frame.context_registers.r8, $frame.context_registers.r9, $frame.context_registers.r10);
        println!("  R11: 0x{:0>16X}  R12: 0x{:0>16X}  R13: 0x{:0>16X}",
        $frame.context_registers.r11, $frame.context_registers.r12, $frame.context_registers.r13);
        println!("  R14: 0x{:0>16X}  R15: 0x{:0>16X}  RIP: 0x{:0>16X}",
        $frame.context_registers.r14, $frame.context_registers.r15, $frame.iret_registers.rip);
        println!("  RBP: 0x{:0>16X}  RSP: 0x{:0>16X}",
        $frame.context_registers.rbp, $frame.iret_registers.rsp);
        println!("  RF:  0x{:0>16X}",
        $frame.iret_registers.rflags);
        println!("  CS:  0x{:0>16X}  SS:  0x{:0>16X}",
        $frame.iret_registers.cs, $frame.iret_registers.ss);
    }
}

macro_rules! impl_handler {
    ($name:ident, $stack: ident, $func:block) => {
        #[naked]
        pub unsafe extern fn $name () {
            #[inline(never)]
            unsafe fn inner($stack: &mut $crate::arch::x86_64::interrupt::util::InterruptFrame) {
                $func
            }

            context_push!();

            let rsp: u64;
            asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

            inner(&mut *(rsp as *mut $crate::arch::x86_64::interrupt::util::InterruptFrame));

            context_pop!();

            iret!();
        }
    };
}

macro_rules! impl_handler_with_error_code {
    ($name:ident, $stack:ident, $func:block) => {
        #[naked]
        pub unsafe extern fn $name () {
            #[inline(never)]
            unsafe fn inner($stack: &$crate::arch::x86_64::interrupt::util::InterruptFrameWithErrorCode) {
                $func
            }

            context_push!();

            let rsp: u64;
            asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

            inner(&*(rsp as *const $crate::arch::x86_64::interrupt::util::InterruptFrameWithErrorCode));

            context_pop!();

            error_code_pop!();
            iret!();
        }
    };
}