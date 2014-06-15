use core::ptr::set_memory;
use core::fmt;
use core::prelude::*;
use core;

use kernel::mm::physical;
use kernel::mm::physical::Phys;
use kernel::mm::{Prot, VirtRange};

pub type Frame = [u8, ..PAGE_SIZE];

pub static PAGE_SIZE: uint = 0x1000;
static PAGE_SIZE_LOG2: uint = 12;

// kinda clever
define_flags!(Flags: u32 {
    // SECTION = 0b10 ?
    SECTION = 0b10010,

    BUFFER = 1 << 2,
    CACHE,
    RW     = 1 << 10,
    CLIENT_ACCESS
})

#[packed]
pub struct Descriptor(u32);

#[packed]
struct PageTableCoarse {
    pages: [Descriptor, ..256]
}

#[allow(visible_private_types)]
#[packed]
pub struct PageDirectory {
    entries: [Descriptor, ..4096]
}

pub static mut directory: *mut PageDirectory = 0 as *mut PageDirectory;

macro_rules! impl_read_write (
    ($reg:expr, $name:ident : $T:ident) => (
        impl $name {
            #[inline] #[allow(dead_code)]
            pub fn read() -> $T {
                unsafe {
                    let flags;
                    asm!(concat!("mrc p15, 0, $0, ", $reg, ", c0, 0") : "=r"(flags));
                    $T(flags)
                }
            }

            #[inline] #[allow(dead_code)]
            pub fn write(f: $T) {
                match f {
                    $T(val) => unsafe {
                        asm!(concat!("mcr p15, 0, $0, ", $reg, ", c0, 0") :: "r"(val) :: "volatile");
                    }
                }
            }
        }
    );
)

define_reg!(CR, CRFlags: uint {
    CR_M  = 1 << 0,  // MMU enable
    CR_A  = 1 << 1,
    CR_C  = 1 << 2,  // Data cache enable

    // sb1
    CR_W  = 1 << 3,
    CR_P  = 1 << 4,  // 32-bit exception handler
    CR_D  = 1 << 5,  // 32-bit data address range
    CR_L  = 1 << 6,  // Implementation defined

    CR_B  = 1 << 7,  // Endianness
    CR_S  = 1 << 8,
    CR_R  = 1 << 9,

    // sb0
    CR_F  = 1 << 10, // Implementation defined
    CR_Z  = 1 << 11, // Implementation defined

    CR_I  = 1 << 12, // Instruction cache enable
    CR_V  = 1 << 13,
    CR_RR = 1 << 14,
    CR_L4 = 1 << 15
})

// Each of the 16 domains can be either allowed full access (manager)
// to a region of memory or restricted access to some pages in that region (client).
define_reg!(DomainType, DomainTypeMask: uint {
    KERNEL = 0b11 << 0,
    USER   = 0b11 << 2,
    NOACCESS = 0,
    CLIENT   = 0b01 * 0x55555555,
    MANAGER  = 0b11 * 0x55555555
})

struct DirBase;

impl_read_write!("c1", CR: CRFlags)
impl_read_write!("c3", DomainType: DomainTypeMask)
// impl_read_write!("c2", DirBase: Phys, Phys<PageDirectory>)

impl DirBase {
    #[inline] #[allow(dead_code)]
    pub fn read() -> Phys<PageDirectory> {
        unsafe {
            let addr;
            asm!(concat!("mrc p15, 0, $0, c2, c0, 0") : "=r"(addr));
            Phys::at(addr)
        }
    }

    #[inline] #[allow(dead_code)]
    pub unsafe fn write(val: Phys<PageDirectory>) {
        asm!(concat!("mcr p15, 0, $0, c2, c0, 0") :: "r"(val.offset()) :: "volatile");
    }
}

impl fmt::Show for CRFlags {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let &CRFlags(f) = self;
        write!(fmt, "{:x}", f)
    }
}

pub unsafe fn init() -> Phys<PageDirectory> {
    let dir: Phys<PageDirectory> = (*physical::frames).zero_alloc(4);
    assert_eq!(dir.offset() & (4 * PAGE_SIZE - 1), 0);

    for i in range(0u, 4096) {
        (*dir.as_ptr()).entries[i] = Descriptor::section(i as u32 << 20, RW);
    }

    directory = dir.as_ptr();
    switch_directory(dir);
    enable_paging();

    dir
}

pub fn switch_directory(dir: Phys<PageDirectory>) {
    // Memory protection is determined by control register c1 bits S and R,
    // domain access reg. c3 and per-page domain number and permission bits.
    let cpu_domain = KERNEL & MANAGER | USER & MANAGER;

    unsafe {
        DomainType::write(cpu_domain);
        DirBase::write(dir);
    }
}

fn enable_paging() {
    unsafe {
        asm!("mov ip, 0
              mcr p15, 0, ip, c7, c5, 0     // invalidate I & D cache
              mcr p15, 0, ip, c7, c10, 4    // drain write buffer
              mcr p15, 0, ip, c8, c7, 0     // invalidate I & D TLBs
            " ::: "ip" : "volatile");

        CR::write(CR - (CR_A | CR_W | CR_P | CR_D | CR_R | CR_F | CR_Z | CR_V | CR_RR)
                    | (CR_S | CR_I | CR_C | CR_M));
    }
}

pub unsafe fn map(_: *mut u8, _: uint, _: Flags) {
    // TODO
}

impl ::kernel::mm::VirtRange {
    pub fn mmap(&self, prot: Prot) -> VirtRange {
        // TODO
        *self
    }
}

impl Descriptor {
    fn section(base: u32, flags: Flags) -> Descriptor {
        // make a section descriptor
        Descriptor(base) | flags | SECTION
    }
}

impl_ops!(Descriptor, Flags)

impl PageDirectory {
    pub unsafe fn map(&self, _: *mut u8, _: uint, _: Flags) {
        // TODO
    }

    pub unsafe fn clone(&mut self) -> Phys<PageDirectory> {
        Phys::at(self as *mut PageDirectory as uint)
    }
}

pub fn clone_directory() -> Phys<PageDirectory> {
    unsafe {
        (*directory).clone()
    }
}
