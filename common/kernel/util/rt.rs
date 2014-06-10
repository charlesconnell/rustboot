/* rt.rs
 */

use core::intrinsics::{ctlz32, cttz32};
use core::mem::{transmute, size_of};

extern "C" {
    pub fn memset(s: *mut u8, c: i32, n: u32);
    pub fn memcpy(dest: *mut u8, src: *u8, n: int);
}

mod detail {
    extern {
        #[link_name = "llvm.debugtrap"]
        pub fn breakpoint();
    }
}

#[no_mangle]
pub fn breakpoint() {
    unsafe { detail::breakpoint() }
}
