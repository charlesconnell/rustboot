use core::clone::Clone;

use kernel::mm::{Flags, PageDirectory};
use kernel::mm::physical;

use platform::cpu::mmu;

pub struct Process {
    pub eip: u32,
    pub esp: u32,
    pub paging: physical::Phys<PageDirectory>
}

impl Process {
    pub fn new() -> Process {
        Process {
            eip: 0,
            esp: 0,
            // paging: unsafe { physical::zero_alloc_frames(1) as *mut PageDirectory }
            paging: unsafe { mmu::clone_directory() }
        }
    }

    pub fn mmap(&self, page_ptr: *mut u8, size: usize, flags: Flags) {
        unsafe {
            (*self.paging.as_ptr()).map(page_ptr, size, flags);
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn enter(&self) {
        unsafe {
            //breakpoint();
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
