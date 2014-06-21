use core::num::{Num, div_rem, Zero, zero, One, one};

#[packed]
struct GpioBitMap {
    pins0: u32,
    pins1: u32,
    _reserved: u32
}

impl GpioBitMap {
    fn read(&self, pin: uint) -> bool {
        let idx = pin & (32 - 1);
        if pin >= 32 {
            (self.pins1 >> idx) & 1 == 1
            // TODO: check overflow
        } else {
            (self.pins0 >> idx) & 1 == 1
        }
    }

    pub fn write(&mut self, pin: uint, bit: bool) {
        let idx = pin & (32 - 1);
        let bit = if bit { 1 << idx } else { 0 };

        if pin >= 32 {
            self.pins1 &= !bit;
            self.pins1 |= bit;
        } else {
            self.pins0 &= !bit;
            self.pins0 |= bit;
        }
    }
}

pub enum PinFunction {
    INPUT = 0,
    OUTPUT = 1,
    ALT0 = 4,
    ALT1 = 5,
    ALT2 = 6,
    ALT3 = 7,
    ALT4 = 3,
    ALT5 = 2
}

#[packed]
struct GpioRegs {
    fsel: [u32, ..6],
    _reserved: u32,
    pub set: GpioBitMap,
    pub clr: GpioBitMap,
    level: GpioBitMap,
    eds: GpioBitMap,
    ren: GpioBitMap,
    fen: GpioBitMap,
    hen: GpioBitMap,
    len: GpioBitMap,
    aren: GpioBitMap,
    afen: GpioBitMap,
    pud: u32,
    pudclk: GpioBitMap,
    // _r: [u32, ..4],
    // test: u8
}

impl GpioRegs {
    pub fn set_mode(&mut self, pin: uint, mode: PinFunction) {
        let (bank_idx, idx) = div_rem(pin, 10);
        let shift = idx * 3;
        let bank = &mut self.fsel[bank_idx];
        *bank = (*bank & !(7 << shift)) | (mode as u32 << shift);
    }
}

pub static GPIO: *mut GpioRegs = 0x2020_0000 as *mut GpioRegs;
