use core::mem::transmute;

use kernel::heap;
use kernel::mm;
use kernel::mm::Allocator;
use cpu::mmu::Frame;
use util::bitv;

use rust_core::fail::abort;

pub static mut frames: *mut FrameAllocator = 0 as *mut FrameAllocator;

pub struct Phys<T> {
    ptr: *mut T
}

impl<T> Phys<T> {
    pub fn at(offset: uint) -> Phys<T> {
        Phys { ptr: offset as *mut T }
    }

    pub fn as_ptr(&self) -> *mut T {
        match *self {
            Phys { ptr: p } => p
        }
    }

    pub fn offset(&self) -> uint {
        unsafe {
            transmute(*self)
        }
    }
}

pub struct FrameAllocator {
    parent: mm::Alloc
}

impl FrameAllocator {
    pub fn new(base: *mut u8) -> FrameAllocator {
        FrameAllocator { parent: mm::Alloc::new(
            mm::BuddyAlloc::new(13, bitv::Bitv { storage: unsafe { heap::zero_alloc::<u32>(1024) } }),
            base,
            12
        ) }
    }

    pub unsafe fn alloc<T = Frame>(&mut self, count: uint) -> Phys<T> {
        match self.parent.alloc(count) {
            (_, 0) => abort(),
            (ptr, _) => Phys { ptr: ptr as *mut T }
        }
    }

    pub unsafe fn zero_alloc<T = Frame>(&mut self, count: uint) -> Phys<T> {
        match self.parent.zero_alloc(count) {
            (_, 0) => abort(),
            (ptr, _) => Phys { ptr: ptr as *mut T }
        }
        // self.alloc(count) ...
    }

    #[inline]
    pub unsafe fn free<T>(&mut self, ptr: Phys<T>) {
        self.parent.free(ptr.offset() as *mut u8);
    }
}

pub fn init() -> FrameAllocator {
    unsafe {
        let a = FrameAllocator::new(0x200_000 as *mut u8);
        frames = &a as *FrameAllocator as *mut FrameAllocator;
        a
    }
}
