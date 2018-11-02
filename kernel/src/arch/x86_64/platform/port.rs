use super::instructions;
use core::marker::PhantomData;

pub trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> Self {
        instructions::inb(port)
    }

    unsafe fn port_out(port: u16, value: Self) {
        instructions::outb(port, value)
    }
}

impl InOut for u16 {
    unsafe fn port_in(port: u16) -> Self {
        instructions::inw(port)
    }

    unsafe fn port_out(port: u16, value: Self) {
        instructions::outw(port, value)
    }
}

impl InOut for u32 {
    unsafe fn port_in(port: u16) -> Self {
        instructions::inl(port)
    }

    unsafe fn port_out(port: u16, value: Self) {
        instructions::outl(port, value)
    }
}

#[derive(Debug)]
pub struct UnsafePort<T: InOut>{
    port: u16,
    phantom: PhantomData<T>
}

impl<T: InOut> UnsafePort<T> {
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort {port, phantom: PhantomData}
    }

    pub unsafe fn read(&mut self) -> T {
        T::port_in(self.port)
    }

    pub unsafe fn write(&mut self, value: T) {
        T::port_out(self.port, value)
    }
}