//! Interrupt descriptor table

use core;

use super::DtReg;

bitflags!(flags IdtFlags: u8 {
    const INTR_GATE = 0b1110,
    const TRAP_GATE = 0b1111,
    const PRESENT = 1 << 7
});

pub type IdtReg = DtReg<IdtEntry>;

#[repr(packed)]
pub struct IdtEntry {
    addr_lo: u16,
    sel: u16,
    zero: u8,
    flags: IdtFlags,
    addr_hi: u16
}

impl IdtEntry {
    pub fn new(func: unsafe extern "C" fn(), sel: u16, flags: IdtFlags) -> IdtEntry {
        let addr = func as usize;
        let (addr_hi, addr_lo) = (
            (addr & 0xFFFF0000) >> 16,
            (addr & 0x____FFFF)
        );
        IdtEntry {
            addr_lo: addr_lo as u16,
            addr_hi: addr_hi as u16,
            sel: sel,
            zero: 0,
            flags: flags
        }
    }
}

impl super::Load for IdtEntry {
    #[inline]
    unsafe fn load(reg: &super::DtReg<IdtEntry>) {
        asm!("lidt [$0]" :: "A"(reg as *const super::DtReg<IdtEntry>) :: "intel");
    }
}
