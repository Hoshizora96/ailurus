#[derive(Debug, Clone, Copy)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    pub const fn new(index: u16, rpl: PrivilegeLevel) -> Self {
        SegmentSelector(index << 3 | (rpl as u16))
    }

    pub fn index(&self) -> u16 {
        self.0 >> 3
    }

    pub fn rpl(&self) -> PrivilegeLevel {
        PrivilegeLevel::from_u16(self.0 & 0b11)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

impl PrivilegeLevel {
    pub fn from_u16(value: u16) -> PrivilegeLevel {
        match value {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            i => panic!("{} is not a valid privilege level", i),
        }
    }
}

pub fn get_cs() -> SegmentSelector {
    let selector: u16;
    unsafe { asm!("mov %cs, $0" : "=r" (selector) ) };
    SegmentSelector(selector)
}

pub unsafe fn load_ds(selector: SegmentSelector) {
    asm!(
        "mov ds, rax"
        : :"{rax}"(selector.0) :"rax" :"intel", "volatile"
    )
}

pub unsafe fn load_es(selector: SegmentSelector) {
    asm!(
        "mov es, rax"
        : :"{rax}"(selector.0) :"rax" :"intel", "volatile"
    )
}

pub unsafe fn load_fs(selector: SegmentSelector) {
    asm!(
        "mov fs, rax"
        : :"{rax}"(selector.0) :"rax" :"intel", "volatile"
    )
}

pub unsafe fn load_gs(selector: SegmentSelector) {
    asm!(
        "mov gs, rax"
        : :"{rax}"(selector.0) :"rax" :"intel", "volatile"
    )
}

pub unsafe fn load_ss(selector: SegmentSelector) {
    asm!(
        "mov ss, rax"
        : :"{rax}"(selector.0) :"rax" :"intel", "volatile"
    )
}