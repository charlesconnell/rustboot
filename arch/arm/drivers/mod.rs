/* driver::mod.rs */

use super::cpu::interrupt;
use super::io;
use core::option::{Option, None};
use kernel;

pub mod arm1176jzf_s;
pub mod arm926ej_s;

pub fn init() {

    /* // not for RPi
    unsafe {
        kernel::int_table.map(|t| {
	// See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dai0235c/index.html
            t.enable(interrupt::IRQ, keypress);
        });
    }*/
    arm1176jzf_s::init();
}

pub static mut keydown: Option<extern unsafe fn(char)> = None;
pub static mut read_char: Option<extern fn()->char> = None;

#[no_mangle]
pub unsafe fn keypress() {
	keydown.map(|f| {
		let x = (*self::arm926ej_s::serial::UART0.base) as u8 as char;
		f(x)
	}
	);
    // Exception return instruction. [8]
    // TODO: better interrupt handler. r11 could change
    asm!("pop {r11, lr}
          subs pc, r14, #4") // pc = lr - 4
}
