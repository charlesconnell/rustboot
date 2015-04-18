use core::intrinsics::write_bytes;

// TODO: make an actual bitmap.

/// A vector of 2-bit values.
pub struct Bitv {
    pub storage: *mut u32
}

impl Bitv {
    #[inline]
    pub fn get(&self, i: usize) -> u8 {
        let w = (i / 16) as isize;
        let b = (i % 16) * 2;
        unsafe {
            (*self.storage.offset(w) as usize >> b) as u8 & 3
        }
    }

    #[inline]
    pub fn set(&self, i: usize, x: u8) {
        let w = (i / 16) as isize;
        let b = (i % 16) * 2;
        unsafe {
            *self.storage.offset(w) = *self.storage.offset(w) & !(3 << b) | ((x as u32) << b)
        }
    }

    #[inline]
    fn as_mut_ptr(&self) -> *mut u8 {
        self.storage as *mut u8
    }

    pub fn clear(&self, capacity: usize) {
        unsafe {
            write_bytes(self.as_mut_ptr(), 0, capacity / 4);
        }
    }
}
