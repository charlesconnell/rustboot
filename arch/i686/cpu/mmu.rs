use core::mem::size_of;
use core::ptr::{copy_nonoverlapping_memory, set_memory};
use core::prelude::*;
use core::fmt;
use core;

use kernel::mm::physical;
use kernel::mm::physical::Phys;
use util::rt;
use kernel::Kernel;
use super::exception::{PageFault, exception_handler};

pub type Frame = [u8, ..PAGE_SIZE];

define_flags!(Flags: uint {
    PRESENT  = 1 << 0,
    RW       = 1 << 1,
    USER     = 1 << 2,
    ACCESSED = 1 << 5,
    HUGE     = 1 << 7
})

#[packed]
pub struct Page(pub uint);

static LOG2_PAGE_SIZE: uint = 12;
pub static PAGE_SIZE: uint = 0x1000;

static ENTRIES:   uint = 1024;

#[packed]
struct VMemLayout {
    temp1: PageDirectory,                    // @ 0xFF7FF000
    temp_tables: [PageTable, ..ENTRIES - 1], // @ 0xFF800000
    temp: PageDirectory,                     // @ 0xFFBFF000
    tables: [PageTable, ..ENTRIES - 1],      // @ 0xFFC00000
    dir: PageDirectory                       // @ 0xFFFFF000
}

static VMEM: *mut VMemLayout = 0xFF7FF000 as *mut VMemLayout;
static DIR_VADDR: uint = 0xFFFFF000;

// U: underlying element type
#[packed]
struct Table<U> {
    entries: [Page, ..ENTRIES]
}

#[packed]
struct Directory<U = PageTable> {
    entries: [U, ..ENTRIES]
}

pub type PageTable = Table<Page>;
pub type PageDirectory = Table<Table<Page>>;

pub unsafe fn init(kernel: &mut Kernel) -> Phys<PageDirectory> {
    let dir: Phys<PageDirectory> = kernel.alloc_frames(1);
    let table: Phys<PageTable>   = kernel.alloc_frames(1);

    (*table.as_ptr()).identity_map(0, PRESENT | RW);
    (*dir.as_ptr()).set_addr(0 as *mut u8, table, PRESENT | RW);

    // Map the directory as its own last table.
    // When accessing its virtual address(...)
    (*dir.as_ptr()).map_self(dir);

    kernel.interrupts.set_isr(PageFault, true, exception_handler());

    switch_directory(dir);
    enable_paging();
    dir
}

pub fn switch_directory(dir: Phys<PageDirectory>) {
    use common::x86::reg::CR3;
    CR3::write(Page::new(dir, Flags::zero()));
}

fn enable_paging() {
    use common::x86::reg::{CR0, CR0_PG};
    CR0::write(CR0 | CR0_PG);
}

pub unsafe fn map_frame(page_ptr: *mut u8, flags: Flags) {
    (*VMEM).dir.map_frame(page_ptr, flags);
}

#[inline]
fn flush_tlb<T>(addr: T) {
    unsafe {
        asm!("invlpg ($0)" :: "r"(addr) : "memory" : "volatile")
    }
}

impl Page {
    fn new<T>(addr: Phys<T>, flags: Flags) -> Page {
        Page(addr.offset()) | flags
    }

    fn at_frame(i: uint, flags: Flags) -> Page {
        Page(i * PAGE_SIZE) | flags
    }

    fn physical<P>(&self) -> Phys<P> {
        let &Page(p) = self;
        Phys::at(p & 0xFFFFF000)
    }

    fn present(self) -> bool {
        self & PRESENT
    }
}

impl core::ops::BitOr<Flags, Page> for Page {
    #[inline(always)]
    fn bitor(&self, other: &Flags) -> Page {
        match (self, other) {
            (&Page(p), &Flags(f)) => Page(p | f)
        }
    }
}

impl core::ops::BitAnd<Flags, bool> for Page {
    #[inline(always)]
    fn bitand(&self, other: &Flags) -> bool {
        match (self, other) {
            (&Page(p), &Flags(f)) => p & f != 0
        }
    }
}

impl fmt::Show for Page {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let &Page(p) = self;
        let page = p & 0xFFFFF000;
        let (p, r, u, a) = (
            if self & PRESENT  { 'P' } else { ' ' },
            if self & RW       { 'R' } else { ' ' },
            if self & USER     { 'U' } else { ' ' },
            if self & ACCESSED { 'A' } else { ' ' }
        );
        write!(fmt, "0x{:x}({}{}{}{})", page, p, r, u, a)
    }
}

impl fmt::Show for Flags {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (p, r, u, a) = (
            if (self & PRESENT).is_zero()  { ' ' } else { 'P' },
            if (self & RW      ).is_zero() { ' ' } else { 'R' },
            if (self & USER    ).is_zero() { ' ' } else { 'U' },
            if (self & ACCESSED).is_zero() { ' ' } else { 'A' }
        );
        write!(fmt, "{}{}{}{}", p, r, u, a)
    }
}

impl<U> Table<U> {
    fn set_addr<S, T>(&mut self, vaddr: *mut S, phys: Phys<T>, flags: Flags) {
        self.set(vaddr as uint, Page::new(phys, flags));
        flush_tlb(vaddr);
    }

    fn set(&mut self, addr: uint, page: Page) { // TODO addr: Phys<T>
        // update entry, based on the underlying type (page, table)
        let size = size_of::<U>() / size_of::<Page>() * PAGE_SIZE;
        let index = (addr / size) % ENTRIES;
        self.entries[index] = page;
    }

    fn get(&self, addr: uint) -> Page {
        let size = size_of::<U>() / size_of::<Page>() * PAGE_SIZE;
        let index = (addr / size) % ENTRIES;
        self.entries[index]
    }
}

impl Table<Page> {
    fn identity_map(&mut self, start: uint, flags: Flags) {
        for i in range(0, ENTRIES) {
            self.entries[i] = Page::at_frame(start + i, flags);
        }
    }
}

// FIXME(Rust #9767): Can't impl on typedefs.
impl Table<Table<Page>> {
    fn fetch_table<T>(&mut self, vptr: *mut T, flags: Flags) -> *mut PageTable {
        match self.get(vptr as uint) {
            table @ Page(_) if table.present() => {
                table.physical().as_ptr()
            }
            _ => unsafe { // allocate table
                let table: Phys<PageTable> = (*physical::frames).alloc(1);

                let size = ENTRIES * PAGE_SIZE;
                let index = (vptr as uint / size) & (ENTRIES - 1);
                self.entries[index] = Page::new(table, flags);

                let t = &mut (*VMEM).tables[index];
                flush_tlb(t);
                set_memory(t, 0, 1);

                table.as_ptr()
            }
        }
    }

    pub unsafe fn set_page<T>(&mut self, vptr: *mut T, phys: Phys<T>, flags: Flags) {
        let table = self.fetch_table(vptr, flags);
        (*table).set_addr(vptr, phys, flags);
    }

    pub unsafe fn map_frame(&mut self, vptr: *mut u8, flags: Flags) -> Phys<u8> {
        let phys = (*physical::frames).alloc(1);
        self.set_page(vptr, phys, flags | PRESENT);
        phys
    }

    fn map_self(&mut self, this: Phys<PageDirectory>) {
        self.set(DIR_VADDR as uint, Page::new(this, PRESENT | RW));
    }

    pub fn clone(&self) -> Phys<PageDirectory> {
        unsafe {
            // Allocate a single page
            let dir_phys: Phys<PageDirectory> = (*physical::frames).alloc(1);
            // Create a blank directory
            let &VMemLayout { ref mut temp1, ref mut dir, .. } = &mut *VMEM;
            dir.set_page(temp1, dir_phys, PRESENT | RW);
            set_memory(temp1, 0, 1);
            temp1.map_self(dir_phys);
            // Copy entries
            let cnt = 0xC0000000 / (ENTRIES * PAGE_SIZE);
            copy_nonoverlapping_memory(&mut temp1.entries as *mut Page, &self.entries as *Page, cnt);

            dir_phys
        }
    }
}

pub fn clone_directory() -> Phys<PageDirectory> {
    unsafe {
        (*VMEM).dir.clone()
    }
}

// FIXME make uses of this function obsolete
pub fn get_dir() -> &'static mut PageDirectory {
    unsafe { &'static mut (*VMEM).dir }
}
