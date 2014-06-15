use core::option::{Option, None};

use super::cpu::interrupt;
use super::io;
use kernel::Kernel;
use kernel;

pub static mut keydown: Option<fn(u32)> = None;

pub fn init(kernel: &mut Kernel) {
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
