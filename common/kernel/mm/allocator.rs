//! Mechanisms for the allocation of kernel objects.

use core::mem::transmute;
use core::intrinsics::{write_bytes, copy};
use core::intrinsics::offset;
use core::intrinsics::ctlz32;

use util::bitv::Bitv;

#[repr(u8)]
enum Node {
    UNUSED = 0,
    USED = 1,
    SPLIT = 2,
    FULL = 3
}

/// The allocator interface. Based on an unfinished RFC.
pub trait Allocator {
    fn alloc(&mut self, size: usize) -> (*mut u8, usize);

    fn zero_alloc(&mut self, s: usize) -> (*mut u8, usize) {
        let (ptr, size) = self.alloc(s);
        unsafe { write_bytes(ptr, 0, size); }
        (ptr, size)
    }

    fn realloc(&mut self, src: *mut u8, size: usize) -> (*mut u8, usize) {
        self.free(src);
        let (ptr, sz) = self.alloc(size);
        unsafe { copy(ptr, src as *const u8, sz); }
        (ptr, sz)
    }

    fn free(&mut self, ptr: *mut u8);
}

/// The [buddy memory allocation\[1\]][1] system is implemented with the use of a binary tree.
/// It can be augmented with the use of linked lists[[2]].
///
/// 1. [Buddy memory allocation - Wikipedia](http://en.wikipedia.org/wiki/Buddy_memory_allocation)
/// 2. [52.206 Operating Systems. Heap memory allocation](http://dysphoria.net/OperatingSystems1/4_allocation_buddy_system.html)
///
/// [1]: http://en.wikipedia.org/wiki/Buddy_memory_allocation
/// [2]: http://dysphoria.net/OperatingSystems1/4_allocation_buddy_system.html
pub struct BuddyAlloc {
    pub order: usize,
    pub tree: Bitv
}

pub struct Alloc {
    pub parent: BuddyAlloc,
    pub base: *mut u8,
    pub el_size: usize
}

impl BuddyAlloc {
    pub fn new(order: usize, storage: Bitv) -> BuddyAlloc {
        storage.clear(1 << (order + 1));
        BuddyAlloc { order: order, tree: storage }
    }

    #[inline]
    fn offset(&self, index: usize, level: usize) -> usize {
        (index + 1 - (1 << self.order >> level)) << level
    }

    fn alloc(&mut self, mut size: usize) -> (usize, usize) {
        if size == 0 {
            size = 1;
        }
        // smallest power of 2 >= size
        let lg2_size = 32 - unsafe { ctlz32(size as u32 - 1) } as usize;

        let mut index = 0; // points to current tree node
        let mut level = self.order; // current height

        loop {
            match (self.get(index), level == lg2_size) {
                (Node::UNUSED, true) => {
                    // Found appropriate unused node
                    self.set(index, Node::USED); // use

                    let mut parent = index;
                    loop {
                        let buddy = parent - 1 + (parent & 1) * 2;
                        match self.get(buddy) {
                            Node::USED | Node::FULL if parent > 0 => {
                                parent = (parent + 1) / 2 - 1;
                                self.set(parent, Node::FULL);
                            }
                            _ => break
                        }
                    }
                    return (
                        self.offset(index, level),
                        1 << lg2_size
                    );
                }
                (Node::UNUSED, false) => {
                    // This large node is unused, split it!
                    self.set(index, Node::SPLIT);
                    self.set(index*2 + 1, Node::UNUSED);
                    self.set(index*2 + 2, Node::UNUSED);
                    index = index * 2 + 1; // left child
                    level -= 1;
                }
                (Node::SPLIT, false) => {
                    // Traverse children
                    index = index * 2 + 1; // left child
                    level -= 1;
                }
                _ => loop {
                    // Go either right or back up
                    if index & 1 == 1 {
                        // right sibling
                        index += 1;
                        break;
                    }

                    // go up by one level
                    level += 1;

                    if index == 0 {
                        // out of memory -- back at tree's root after traversal
                        return (0, 0);
                    }

                    index = (index + 1) / 2 - 1; // parent
                }
            }
        }
    }

    fn free(&mut self, offset: usize) {
        let mut length = 1 << self.order;
        let mut left = 0;
        let mut index = 0;

        loop {
            match self.get(index) {
                Node::UNUSED => return,
                Node::USED => loop {
                    if index == 0 {
                        self.set(0, Node::UNUSED);
                        return;
                    }

                    let buddy = index - 1 + (index & 1) * 2;
                    match self.get(buddy) {
                        Node::UNUSED => {}
                        _ => {
                            self.set(index, Node::UNUSED);
                            loop {
                                let parent = (index + 1) / 2 - 1; // parent
                                match self.get(parent) {
                                    Node::FULL if index > 0 => {
                                        self.set(parent, Node::SPLIT);
                                    }
                                    _ => return
                                }
                                index = parent;
                            }
                        }
                    }
                    index = (index + 1) / 2 - 1; // parent
                },
                _ => {
                    length /= 2;
                    if offset < left + length {
                        index = index * 2 + 1; // left child
                    }
                    else {
                        left += length;
                        index = index * 2 + 2; // right child
                    }
                }
            }
        }
    }

    fn get(&self, i: usize) -> Node {
        unsafe {
            transmute(self.tree.get(i))
        }
    }

    fn set(&self, i: usize, x: Node) {
        self.tree.set(i, x as u8);
    }
}

impl Allocator for Alloc {
    fn alloc(&mut self, size: usize) -> (*mut u8, usize) {
        let (offset, size) = self.parent.alloc(size);
        unsafe {
            return (
                self.base.offset((offset << self.el_size) as isize),
                size << self.el_size
            )
        }
    }

    fn free(&mut self, ptr: *mut u8) {
        let length = 1 << self.parent.order << self.el_size;

        unsafe {
            if ptr < self.base || ptr >= self.base.offset(length) {
                return;
            }
        }

        let offset = (ptr as usize - self.base as usize) >> self.el_size;
        self.parent.free(offset);
    }
}

impl Alloc {
    pub fn new(parent: BuddyAlloc, base: *mut u8, el_size: usize) -> Alloc {
        Alloc { parent: parent, base: base, el_size: el_size }
    }
}
