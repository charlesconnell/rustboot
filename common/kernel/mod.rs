use core::option::{Some};
use core::mem::uninit;
use core::ptr::RawPtr;

use platform::{cpu, io, drivers};
use platform::cpu::interrupt;
use platform::cpu::mmu::{Frame, PageDirectory};
use util::rt::breakpoint;
use self::mm::physical::{Phys, FrameAllocator};

pub mod util;
pub mod mm;
pub mod heap;
pub mod syscall;
mod process;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod elf;
//mod sh; // this doesn't exist yet...

// #[lang="fail_"]
// pub fn fail_(_: *u8, _: *u8, _: uint) -> ! {
//     loop {}
// }
pub struct Kernel {
    process: process::Process,
    heap: mm::Alloc,
    frames: mm::physical::FrameAllocator, // TODO: spinlock
    pub interrupts: interrupt::Table,
    // page_dir: mm::physical::Phys<PageDirectory>
}

impl Kernel {
    fn init() -> Kernel {
        let mut frame_allocator = mm::physical::init();
        let heap = heap::init(&mut frame_allocator);
        let int_table = interrupt::Table::new();
        int_table.load();
        unsafe {
            drivers::keydown = Some(io::putc);
        }
        let mut this = Kernel {
            heap: heap,
            frames: frame_allocator,
            interrupts: int_table,
            process: process::Process {
                eip: 0,
                esp: 0,
                paging: unsafe { uninit() }
            }
        };
        this.process.paging = cpu::init(&mut this);
        drivers::init(&mut this);

        this
    }

    pub fn alloc_frames<T = Frame>(&mut self, count: uint) -> Phys<T> {
        unsafe { self.frames.alloc(count) }
    }

    pub fn zero_alloc_frames<T = Frame>(&mut self, count: uint) -> Phys<T> {
        unsafe { self.frames.zero_alloc(count) }
    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang="start"]
#[no_mangle]
pub fn main() {
    let mut kernel = Kernel::init();

    unsafe {
        let stack: Phys<Frame> = kernel.alloc_frames(1);
        let mut init = process::Process::new();
        init.esp = stack.as_ptr().offset(1) as u32;
        run_init(init);
    }
}

#[cfg(target_arch = "x86")]
unsafe fn run_init(mut init: process::Process) -> ! {
    init.eip = initcode as u32;
    init.enter();

    asm!("initcode:
        mov eax, 11 // sys_execve
        int 0x80
        $$.loop: jmp $$.loop" :::: "intel")
    extern { fn initcode(); }
    fail!()
}

#[cfg(target_arch = "arm")]
unsafe fn run_init(mut init: process::Process) -> ! {
    init.eip = initcode as u32;
    init.enter();

    asm!("initcode:
        swi 0
        b .")
    extern { fn initcode(); }
    fail!()
}
