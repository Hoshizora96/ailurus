use core::slice;
use core::mem::transmute;
use core::str;

use super::super::platform::instructions;
const IA32_APIC_BASE_MSR: u32 = 0x1B;

pub fn has_apic() -> bool {
    let (ebx, ecx, edx) = unsafe { instructions::cpuid(1) };
    // ebx[bit:9] will be 1 if CPU supports APIC
    edx & 0b_10_0000_0000 != 0
}

pub fn get_apic_base_addr()->(u32,u32) {
    unsafe {
        let (eax, edx) = instructions::rdmsr(IA32_APIC_BASE_MSR);
        (eax, edx)
    }
}

pub fn get_vendor_info() -> VendorInfo {
    unsafe {
        let (ebx, ecx, edx) = instructions::cpuid(0);
        VendorInfo { ebx, edx, ecx }
    }
}

pub struct VendorInfo {
    ebx: u32,
    edx: u32,
    ecx: u32,
}

impl VendorInfo {
    pub fn as_string<'a>(&'a self) -> &'a str {
        unsafe {
            let brand_string_start = self as *const VendorInfo as *const u8;
            let slice = slice::from_raw_parts(brand_string_start, 3 * 4);
            let byte_array: &'a [u8] = transmute(slice);
            str::from_utf8_unchecked(byte_array)
        }
    }
}

