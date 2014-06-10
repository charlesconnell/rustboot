use core::ptr::RawPtr;
use core::clone::Clone;
use core::intrinsics::transmute;

use kernel::mm::{Flags, PageDirectory};
use kernel::mm::physical;

use util::rt::breakpoint;

use platform::cpu::mmu;
use platform::cpu::mmu::{
    PAGE_SIZE,
    PRESENT
};

pub struct Process {
    pub eip: u32,
    pub esp: u32,
    pub paging: physical::Phys<PageDirectory>
}

impl Process {
    pub fn new() -> Process {
        // TODO: set stack
        Process {
            eip: 0,
            esp: 0,
            // paging: unsafe { physical::zero_alloc_frames(1) as *mut PageDirectory }
            paging: unsafe { mmu::clone_directory() }
        }
    }

    pub fn mmap(&self, mut page_ptr: *mut u8, size: uint, flags: Flags) {
        // TODO: optimize with uints?
        unsafe {
            let end = page_ptr.offset(size as int);
            while page_ptr < end {
                let frame = (*physical::frames).alloc(1);
                (*self.paging.as_ptr()).set_page(page_ptr, frame, flags | PRESENT);
                // FIXME do not set globally!
                mmu::get_dir().set_page(page_ptr, frame, flags | PRESENT);
                page_ptr = page_ptr.offset(PAGE_SIZE as int);
            }
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn enter(&self) {
        unsafe {
            breakpoint();
            // TODO need to store physical address
            mmu::switch_directory(self.paging);
            asm!("xor %eax, %eax
                  xor %edx, %edx
                  jmp *$0" :: "m"(self.eip), "{esp}"(self.esp) :: "volatile")
        }
    }

    #[cfg(target_arch = "arm")]
    pub fn enter(&self) {}
}
