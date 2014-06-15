use kernel::mm::physical::Phys;
use kernel::mm::PageDirectory;
use kernel::Kernel;

pub mod interrupt;
pub mod mmu;

pub fn init(kernel: &mut Kernel) -> Phys<PageDirectory> {
    unsafe {
        mmu::init()
    }
}

pub fn info() {
}
