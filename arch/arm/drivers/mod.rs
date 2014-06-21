use core::option::{Option, None};

use super::cpu::interrupt;
use super::io;
use kernel::Kernel;
use kernel;

mod bcm2835;

pub static mut keydown: Option<fn(u32)> = None;

pub fn init(kernel: &mut Kernel) {
    use self::bcm2835::*;
    unsafe {
        (*GPIO).set_mode(16, OUTPUT);
        let mut i = 0;
        loop {
            if (i & (1 << 20)) != 0 {
                (*GPIO).set.write(16, true);
            }
            else {
                (*GPIO).clr.write(16, true);
            }
            i += 1;
        }
    }
    unsafe {
        kernel.interrupts.enable(interrupt::IRQ, keypress);
    }
}

#[no_mangle]
pub unsafe fn keypress() {
    keydown.map(|f| f(*io::UART0) );
    // Exception return instruction. [8]
    // TODO: better interrupt handler. r11 could change
    asm!("pop {r11, lr}
          subs pc, r14, #4") // pc = lr - 4
}
