use platform::io;
use core::mem::volatile_store;

static VIC_INT_ENABLE: *mut u32 = (0x10140000 + 0x010) as *mut u32;
static UART0_IRQ: u8 = 12;

fn set_word(handler: u8, instruction: u32) {
    unsafe {
        volatile_store((handler * 4) as *mut u32, instruction);
    }
}

fn branch(rel: u32) -> u32 {
    // b isr ; branch instruction [1]
    0xea000000 | (((rel - 8) >> 2) & 0xffffff)
}

pub struct table;

impl table {
    pub unsafe fn new() -> table {
        table
    }

    pub fn enable(&self, handler: u8, isr: u32) {
        // Installing exception handlers into the vectors directly [1]
        set_word(handler, branch(isr - (handler as u32 * 4)));
    }

    pub unsafe fn load(&self) {
        let mut i = 0;
        while i < 10 {
            // make every handler loop indefinitely
            set_word(i, branch(0));
            i += 1;
        }

        self.enable(0, start as u32);

        // Enable IRQs [5]
        asm!("mov r2, sp
          mrs r0, cpsr      // get Program Status Register
          bic r1, r0, #0x1F // go in IRQ mode
          orr r1, r1, #0x12
          msr cpsr, r1
          mov sp, 0x19000   // set IRQ stack
          bic r0, r0, #0x80 // Enable IRQs
          msr cpsr, r0      // go back in Supervisor mode
          mov sp, r2"
        ::: "r0", "r1", "r2", "cpsr");

        // enable UART0 IRQ [4]
        *VIC_INT_ENABLE = 1 << UART0_IRQ;
        // enable RXIM interrupt
        *io::UART0_IMSC = 1 << 4;
    }
}

extern "C" {
    fn start();
}

/*
#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe { zero::abort(); }
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe { zero::abort(); }
}
*/
