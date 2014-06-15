use core::ptr::RawPtr;
use core::clone::Clone;
use core::intrinsics::transmute;

use kernel::mm::{Prot, PageDirectory, VirtRange};
use kernel::mm::physical;

use util::rt::breakpoint;

use platform::cpu::mmu;
use platform::cpu::mmu::{
    PAGE_SIZE
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

    pub fn mmap(&self, mut page_ptr: *mut u8, size: uint, prot: Prot) -> VirtRange {
        // TODO: optimize with uints?
        VirtRange::new(page_ptr, size).mmap(prot)
    }

    #[cfg(target_arch = "x86")]
    pub fn jump((ip, sp): (uint, uint)) {
        unsafe {
            asm!("xor eax, eax
                  xor edx, edx
                  jmp $0" :: "r"(ip), "{esp}"(sp) : "eax", "edx" : "volatile", "intel")
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn enter(&self) {
        unsafe {
            breakpoint();
            // TODO need to store physical address
            mmu::switch_directory(self.paging);
            asm!("xor eax, eax
                  xor edx, edx
                  jmp $0" :: "r"(self.eip), "{esp}"(self.esp) : "eax", "edx" : "volatile", "intel")
        }
    }

    #[cfg(target_arch = "arm")]
    pub fn jump((ip, sp): (uint, uint)) {
        unsafe {
            asm!("bx $0" :: "r"(ip), "{sp}"(sp) :: "volatile")
        }
    }

    #[cfg(target_arch = "arm")]
    pub fn enter(&self) {}
}
