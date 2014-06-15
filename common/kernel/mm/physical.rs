use core::mem::transmute;
use core::ptr::set_memory;

use kernel::heap;
use kernel::mm;
use kernel::mm::Allocator;
use platform::cpu::mmu::Frame;
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
    pub fn new(base: *mut u8, storage: *mut [u32, ..1024]) -> FrameAllocator {
        FrameAllocator { parent: mm::Alloc::new(
            mm::BuddyAlloc::new(13, bitv::Bitv { storage: storage as *mut u32 }),
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
        self.alloc(count)
    }

    #[inline]
    pub unsafe fn free<T>(&mut self, ptr: Phys<T>) {
        self.parent.free(ptr.offset() as *mut u8);
    }
}

pub fn init() -> FrameAllocator {
    unsafe {
        let btree = 0x100_000 as *mut [u32, ..1024];
        set_memory(btree, 0, 1);
        // keep it well aligned
        let a = FrameAllocator::new(0x110_000 as *mut u8, btree);
        frames = &a as *FrameAllocator as *mut FrameAllocator;
        a
    }
}
