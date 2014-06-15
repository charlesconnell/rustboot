use kernel::mm::physical::Phys;
use kernel::mm::PageDirectory;
use kernel::Kernel;

pub mod interrupt;
pub mod mmu;

pub struct Context {
	r0: u32,
	r1: u32,
	r2: u32,
	r3: u32,
}

impl Context {
	pub fn syscall_args(&self) -> [u32, ..6] {
		[self.r0, self.r1, self.r2, self.r3, 0, 0]
	}

    pub fn get_arg(&mut self) -> u32 {
        self.r0
    }

    pub fn set_arg(&mut self, ret: uint) {
        self.r0 = ret as u32;
    }
}

pub fn init(kernel: &mut Kernel) -> Phys<PageDirectory> {
    unsafe {
        mmu::init()
    }
}

pub fn info() {
}
