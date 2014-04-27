use core::option::{Option, None};
use core::mem::transmute;

use cpu::interrupt::{interrupt_handler, Isr, Int};
use kernel::Kernel;
use kernel;

pub mod pic;
pub mod vga;
pub mod keyboard;

pub static mut keydown: Option<fn(u8)> = None;

pub fn init(kernel: &mut Kernel) {
    vga::clear_screen(vga::LightRed);
    vga::cursor_at(0);

    unsafe {
        kernel.interrupts.enable_maskable(keyboard::IRQ, keyboard::isr_addr());
        kernel.interrupts.set_intr_gate(0x80, Isr::new(transmute(0x80u8), false));
    }
}
