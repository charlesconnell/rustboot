use core::option::{Option, Some, None};
use core::mem::{uninit};

use platform::{cpu, io, drivers};
use cpu::interrupt;
use cpu::mmu::{Frame, PageDirectory, RW, PRESENT};
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
mod sh;

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
        let heap = heap::init();
        let frame_allocator = mm::physical::init();
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

#[lang="start"]
#[no_mangle]
pub fn main() {
    let mut kernel = Kernel::init();

    unsafe {
    }
    elf::exec(&_binary_initram_elf_start);
    extern { static _binary_initram_elf_start: u8; }
}
