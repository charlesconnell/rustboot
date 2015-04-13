use core::mem::transmute;

use core::fmt;

use platform::io;
use platform::cpu::mmu::Page;
use cpu::Context;

#[repr(u8)]
pub enum Fault {
    DivideError = 0,
    NMI = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundExceeded = 5,
    InvalidOpcode = 6,
    NoMathCoprocessor = 7,
    DoubleFault = 8,
    CoprocessorSegmentOverun = 9,
    InvalidTss = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtection = 13,
    PageFault = 14,
    FloatingPointError = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SimdFpException = 19,
}

static Exceptions: &'static [&'static str] = &[
    "Divide-by-zero Error",
    "Debug",
    "Non-maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device Not Available",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Invalid TSS",
    "Segment Not Present",
    "Stack-Segment Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",
    "x87 Floating-Point Exception",
    "Alignment Check",
    "Machine Check",
    "SIMD Floating-Point Exception",
    "Virtualization Exception",
];

// TODO respect destructors
#[lang="begin_unwind"]
unsafe extern "C" fn begin_unwind(fmt: &fmt::Arguments, file: &str, line: uint) -> ! {
    asm!("hlt");
    loop { }; // for divergence check
}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(msg: &fmt::Arguments,
                                file: &'static str, line: uint) -> ! {
    unsafe { asm!("hlt"); }
    loop { }; // for divergence check
}

#[no_stack_check]
#[inline(never)]
unsafe fn blue_screen(stack: &Context) {
    io::puts("Exception ");
    io::puts(Exceptions[stack.int_no as uint]);
    asm!("hlt");
}

#[no_stack_check]
#[inline(never)]
pub unsafe fn exception_handler() -> unsafe extern "C" fn() {
    asm!("jmp skip_exception_handler
      exception_handler_asm:"
        :::: "volatile", "intel");

    // Points to the data on the stack
    let stack_ptr = Context::save();

    if stack_ptr.int_no as u8 == Fault::PageFault as u8 {
        let cr2: uint;
        asm!("mov %cr2, %eax" : "={eax}"(cr2));
        println!("Accessed {0:x} from {1:x}", cr2, stack_ptr.call_stack.eip);
    }

    if stack_ptr.int_no as u8 == Fault::Breakpoint as u8 {
        asm!("debug:" :::: "volatile")
    }
    else {
        blue_screen(stack_ptr);
    }

    Context::restore();

    asm!("skip_exception_handler:"
        :::: "volatile", "intel");

    extern { fn exception_handler_asm(); }
    exception_handler_asm
}
