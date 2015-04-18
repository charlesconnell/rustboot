use core::ptr::{copy_nonoverlapping, write_bytes};
use core::mem::transmute;
use core::prelude::*;
use core;

use kernel::process::Process;
use kernel::mm;
use platform::io;

#[cfg(target_pointer_width = "32")] pub use self::elf32::{Ehdr, Phdr, Auxv, AuxvValue, AuxvType};
#[cfg(target_pointer_width = "64")] pub use self::elf64::{Ehdr, Phdr, Auxv, AuxvValue, AuxvType};
#[cfg(target_pointer_width = "32")] mod elf32;
#[cfg(target_pointer_width = "64")] mod elf64;

#[repr(u32)]
enum HeaderType {
    PT_NULL = 0,
    PT_LOAD = 1,
    PT_DYNAMIC = 2,
    PT_INTERP = 3,
    PT_NOTE = 4,
    PT_SHLIB = 5,
    PT_PHDR = 6,
    PT_TLS = 7,
    PT_LOOS = 0x60000000,
    PT_GNU_EH_FRAME = 0x6474e550,
    PT_GNU_STACK    = 0x6474e551,
    PT_HIOS = 0x6fffffff,
    PT_LOPROC = 0x70000000,
    PT_HIPROC = 0x7fffffff
}

bitflags!(flags HeaderFlags: u32 {
    const PT_X = 1,
    const PT_R = 2,
    const PT_W = 4
});

#[repr(packed)]
struct ELFIdent {
    ei_mag: [u8; ..4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; ..7]
}

trait EhdrT {
    unsafe fn spawn_process(&self) -> Process;
}

trait PhdrT {
    unsafe fn load(&self, task: &Process, buffer: *const u8);
}

impl EhdrT for self::Ehdr {
    unsafe fn spawn_process(&self) -> Process {
        let mut task = Process::new();
        //TODO: Verify file integrity
        let buffer: *const u8 = transmute(self);
        let ph_size = self.e_phentsize as isize;
        let ph_base = buffer.offset(self.e_phoff as isize);

        let mut stack_flags = mm::RW;

        for i in 0..self.e_phnum {
            let pheader = ph_base.offset(ph_size * i as isize) as *const Phdr;

            match (*pheader).p_type {
                HeaderType::PT_NULL => {}
                HeaderType::PT_LOAD => (*pheader).load(&task, buffer),
                HeaderType::PT_DYNAMIC => (*pheader).load(&task, buffer),
                HeaderType::PT_GNU_STACK => {
                    if (*pheader).p_flags.contains(PT_X) {
                        // We don't need an executable stack
                        stack_flags = mm::Flags::empty();
                    }
                },
                _ => {}
            }
        }

        static stack_bottom: u32 = 0xC0000000;
        let stack_vaddr = (stack_bottom - 0x1000) as *mut u8;
        task.mmap(stack_vaddr, 0x1000, stack_flags);
        let stack_ptr = (stack_bottom as *mut u8).offset(-(((4 + 5 + 15) & !0xF) + 8 + 4 + 4 + 4));
        let argv_ptr = stack_ptr as *mut *mut u8;
        let envp_ptr = argv_ptr.offset(2);
        let auxv_ptr = argv_ptr.offset(1) as *mut Auxv;
        let str_ptr = (stack_bottom as *mut u8).offset(-(4 + 5));

        *argv_ptr.offset(1) = transmute(u0);
        *envp_ptr = transmute(u0);
        *auxv_ptr = Auxv { a_type: AuxvType::AT_NULL, a_un: AuxvValue { data: 0 } };

        let (strs, len): (*const u8, usize) = transmute("test\0");
        copy_nonoverlapping(str_ptr, strs, len);
        *argv_ptr = str_ptr;

        // return entry address
        task.esp = stack_ptr as u32;
        task.eip = transmute(self.e_entry);
        task
    }
}

impl PhdrT for self::Phdr {
    unsafe fn load(&self, task: &Process, buffer: *const u8) {
        let vaddr = self.p_vaddr as *mut u8;
        let mem_size = self.p_memsz as usize;
        let file_pos = self.p_offset as isize;
        let file_size = self.p_filesz as usize;

        let flags = if self.p_flags.contains(PT_W) {
            mm::RW
        } else {
            mm::Flags::empty()
        };

        task.mmap(vaddr, mem_size, flags);

        copy_nonoverlapping(vaddr, buffer.offset(file_pos), file_size);
        write_bytes(vaddr.offset(file_size as isize), 0, mem_size - file_size);
    }
}

impl ELFIdent {
    unsafe fn load(&self) -> Option<&Ehdr> {
        // TODO: check endianness
        static MAGIC_STRING : &'static str = "\x7fELF";
        if *(MAGIC_STRING.as_ptr() as *const u32) != transmute(self.ei_mag) {
            return None;
        }

        #[cfg(target_word_size = "32")] const CLASS: u8 = 1;
        #[cfg(target_word_size = "64")] const CLASS: u8 = 2;

        match self.ei_class {
            CLASS => return Some(transmute(self)),
            _ => return None
        }
    }
}

pub fn exec(buffer: *const u8) {
    unsafe {
        let ident: &ELFIdent = transmute(buffer);
        ident.load().map(|e| { e.spawn_process().enter() });
    }
}
