use core::iter::Iterator;
use core::ptr::RawPtr;
use core::option::{Option, Some, None};
use core;

use platform::cpu::mmu::PAGE_SIZE;

pub use self::allocator::{
    Allocator,
    BuddyAlloc,
    Alloc,
};

pub use platform::cpu::mmu::{
    Frame,
    PageDirectory,
};

pub mod allocator;
pub mod physical;

define_flags!(Prot: uint {
    READ     = 1 << 0,
    WRITE    = 1 << 1,
    EXEC     = 1 << 2,
    NONE     = 0
})

// in bytes
pub struct VirtRange {
    pub vaddr: uint,
    pub len: uint
}

impl VirtRange {
    pub fn new(ptr: *mut u8, len: uint) -> VirtRange {
        VirtRange {
            vaddr: ptr as uint,
            len: len
        }
    }

    // fn protect(prot: Prot)

    pub fn iter(&self) -> Frames {
        Frames {
            ptr: (self.vaddr & !(PAGE_SIZE - 1)) as *mut _,
            end: ((self.vaddr + self.len + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) as *mut _
        }
    }
}

struct Frames {
    ptr: *mut Frame,
    end: *mut Frame
}

impl Iterator<*mut Frame> for Frames {
    fn next(&mut self) -> Option<*mut Frame> {
        if self.ptr == self.end {
            None
        } else {
            let old = self.ptr;
            self.ptr = unsafe { self.ptr.offset(1) };
            Some(old)
        }
    }
}
