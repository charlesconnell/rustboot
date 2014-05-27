use core::mem::transmute;
use core::intrinsics::offset;

use rust_core::c_types::{c_ushort, c_uint, c_int, c_ulong, c_long};

use util::int::range;

// rust-bindgen generated bindings
pub type Half = c_ushort;
pub type Word = c_uint;
pub type Sword = c_int;
pub type Xword = c_ulong;
pub type Sxword = c_long;
pub type Addr = c_ulong;
pub type Off = c_ulong;
pub type Section = c_ushort;
pub type Symndx = c_ulong;
type c_uchar = u8;
type c_void = uint;

#[packed]
pub struct Ehdr {
    e_ident: [c_uchar, ..16u],
    e_type: Half,
    e_machine: Half,
    e_version: Word,
    e_entry: Addr,
    e_phoff: Off,
    e_shoff: Off,
    e_flags: Word,
    e_ehsize: Half,
    e_phentsize: Half,
    e_phnum: Half,
    e_shentsize: Half,
    e_shnum: Half,
    e_shstrndx: Half,
}

#[packed]
pub struct Phdr {
    p_type: super::HeaderType,
    p_flags: Word,
    p_offset: Off,
    p_vaddr: Addr,
    p_paddr: Addr,
    p_filesz: Xword,
    p_memsz: Xword,
    p_align: Xword,
}

#[packed]
pub struct Shdr {
    sh_name: Word,
    sh_type: Word,
    sh_flags: Xword,
    sh_addr: Addr,
    sh_offset: Off,
    sh_size: Xword,
    sh_link: Word,
    sh_info: Word,
    sh_addralign: Xword,
    sh_entsize: Xword,
}

pub struct Sym {
    st_name: Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Section,
    st_value: Addr,
    st_size: Xword,
}
pub struct Syminfo {
    si_boundto: Half,
    si_flags: Half,
}
pub struct Rel {
    r_offset: Addr,
    r_info: Xword,
}
pub struct Rela {
    r_offset: Addr,
    r_info: Xword,
    r_addend: Sxword,
}

pub struct Union_Unnamed2 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed2 {
    pub fn d_val(&mut self) -> *mut Xword {
        unsafe { transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Addr {
        unsafe { transmute(self) }
    }
}
pub struct Dyn {
    d_tag: Sxword,
    d_un: Union_Unnamed2,
}
pub struct Verdef {
    vd_version: Half,
    vd_flags: Half,
    vd_ndx: Half,
    vd_cnt: Half,
    vd_hash: Word,
    vd_aux: Word,
    vd_next: Word,
}
pub struct Verdaux {
    vda_name: Word,
    vda_next: Word,
}
pub struct Verneed {
    vn_version: Half,
    vn_cnt: Half,
    vn_file: Word,
    vn_aux: Word,
    vn_next: Word,
}
pub struct Vernaux {
    vna_hash: Word,
    vna_flags: Half,
    vna_other: Half,
    vna_name: Word,
    vna_next: Word,
}

pub struct AuxvValue {
    data: [c_uchar, ..8u],
}

impl AuxvValue {
    pub fn a_val(&mut self) -> *mut c_long {
        unsafe { transmute(self) }
    }
    pub fn a_ptr(&mut self) -> *mut *mut c_void {
        unsafe { transmute(self) }
    }
    pub fn a_fcn(&mut self) -> *mut extern fn() {
        unsafe { transmute(self) }
    }
}

pub struct Auxv {
    a_type: c_long,
    a_un: AuxvValue,
}

pub struct Nhdr {
    n_namesz: Word,
    n_descsz: Word,
    n_type: Word,
}
pub struct Lib {
    l_name: Word,
    l_time_stamp: Word,
    l_checksum: Word,
    l_version: Word,
    l_flags: Word,
}
