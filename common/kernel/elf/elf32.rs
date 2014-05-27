use core::mem::transmute;

use rust_core::c_types::{c_ushort, c_uint, c_int, c_ulong, c_long};

use kernel::process::Process;
use util::int;
use util::ptr::mut_offset;

// rust-bindgen generated bindings
pub type Half = c_ushort;
pub type Word = c_uint;
pub type Sword = c_int;
pub type Xword = c_ulong;
pub type Sxword = c_long;
pub type Addr = c_uint;
pub type Off = c_uint;
pub type Section = c_ushort;
pub type Symndx = c_uint;
type c_uchar = u8;
type c_void = uint;

#[packed]
pub struct Ehdr {
    pub e_ident: [c_uchar, ..16u],
    pub e_type: Half,
    pub e_machine: Half,
    pub e_version: Word,
    pub e_entry: Addr,
    pub e_phoff: Off,
    pub e_shoff: Off,
    pub e_flags: Word,
    pub e_ehsize: Half,
    pub e_phentsize: Half,
    pub e_phnum: Half,
    pub e_shentsize: Half,
    pub e_shnum: Half,
    pub e_shstrndx: Half,
}

#[packed]
pub struct Phdr {
    pub p_type: super::HeaderType,
    pub p_offset: Off,
    pub p_vaddr: Addr,
    pub p_paddr: Addr,
    pub p_filesz: Word,
    pub p_memsz: Word,
    pub p_flags: super::HeaderFlags,
    pub p_align: Word,
}

#[packed]
pub struct Shdr {
    sh_name: Word,
    sh_type: Word,
    sh_flags: Word,
    sh_addr: Addr,
    sh_offset: Off,
    sh_size: Word,
    sh_link: Word,
    sh_info: Word,
    sh_addralign: Word,
    sh_entsize: Word,
}

pub struct Sym {
    st_name: Word,
    st_value: Addr,
    st_size: Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Section,
}

pub struct Syminfo {
    si_boundto: Half,
    si_flags: Half,
}

pub struct Rel {
    r_offset: Addr,
    r_info: Word,
}

pub struct Rela {
    r_offset: Addr,
    r_info: Word,
    r_addend: Sword,
}

pub struct Union_Unnamed1 {
    data: [c_uchar, ..4u],
}
impl Union_Unnamed1 {
    pub fn d_val(&mut self) -> *mut Word {
        unsafe { transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Addr {
        unsafe { transmute(self) }
    }
}
pub struct Dyn {
    d_tag: Sword,
    d_un: Union_Unnamed1,
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
    pub data: c_int, // or 8u?
}

impl AuxvValue {
    pub fn a_val(&mut self) -> *mut c_int {
        unsafe { transmute(self) }
    }
    pub fn a_ptr(&mut self) -> *mut *mut c_void {
        // WARN: cannot use 32 bit pointers on x86_64!
        unsafe { transmute(self) }
    }
    pub fn a_fcn(&mut self) -> *mut extern fn() {
        unsafe { transmute(self) }
    }
}

pub struct Auxv {
    pub a_type: AuxvType,
    pub a_un: AuxvValue,
}

/* Legal values for a_type (entry type).  */
#[repr(u32)]
pub enum AuxvType {
    AT_NULL     = 0,       /* End of vector */
    AT_IGNORE   = 1,       /* Entry should be ignored */
    AT_EXECFD   = 2,       /* File descriptor of program */
    AT_PHDR     = 3,       /* Program headers for program */
    AT_PHENT    = 4,       /* Size of program header entry */
    AT_PHNUM    = 5,       /* Number of program headers */
    AT_PAGESZ   = 6,       /* System page size */
    AT_BASE     = 7,       /* Base address of interpreter */
    AT_FLAGS    = 8,       /* Flags */
    AT_ENTRY    = 9,       /* Entry point of program */
    AT_NOTELF   = 10,      /* Program is not ELF */
    AT_UID      = 11,      /* Real uid */
    AT_EUID     = 12,      /* Effective uid */
    AT_GID      = 13,      /* Real gid */
    AT_EGID     = 14,      /* Effective gid */
    AT_CLKTCK   = 17,      /* Frequency of times() */

/* Some more special a_type values describing the hardware.  */
    AT_PLATFORM = 15,      /* String identifying platform.  */
    AT_HWCAP    = 16,      /* Machine-dependent hints about
                       processor capabilities.  */

/* This entry gives some information about the FPU initialization
   performed by the kernel.  */
    AT_FPUCW    = 18,      /* Used FPU control word.  */

/* Cache block sizes.  */
    AT_DCACHEBSIZE = 19,      /* Data cache block size.  */
    AT_ICACHEBSIZE = 20,      /* Instruction cache block size.  */
    AT_UCACHEBSIZE = 21,      /* Unified cache block size.  */

/* A special ignored value for PPC, used by the kernel to control the
   interpretation of the AUXV. Must be > 16.  */
    AT_IGNOREPPC   = 22,      /* Entry should be ignored.  */

    AT_SECURE = 23,      /* Boolean, was exec setuid-like?  */

    AT_BASE_PLATFORM = 24,     /* String identifying real platforms.*/

    AT_RANDOM = 25,      /* Address of 16 random bytes.  */

    AT_HWCAP2 = 26,      /* More machine-dependent hints about
                       processor capabilities.  */

    AT_EXECFN = 31,      /* Filename of executable.  */

/* Pointer to the global system page used for system calls and other
   nice things.  */
    AT_SYSINFO = 32,
    AT_SYSINFO_EHDR = 33,

/* Shapes of the caches.  Bits 0-3 contains associativity; bits 4-7 contains
   log2 of line size; mask those to get cache size.  */
    AT_L1I_CACHESHAPE = 34,
    AT_L1D_CACHESHAPE = 35,
    AT_L2_CACHESHAPE  = 36,
    AT_L3_CACHESHAPE  = 37
}

pub struct Nhdr {
    n_namesz: Word,
    n_descsz: Word,
    n_type: Word,
}

pub struct Struct_Unnamed5 {
    gt_current_g_value: Word,
    gt_unused: Word,
}
pub struct Struct_Unnamed6 {
    gt_g_value: Word,
    gt_bytes: Word,
}
pub struct gptab {
    data: [c_uchar, ..8u],
}
impl gptab {
    pub fn gt_header(&mut self) -> *mut Struct_Unnamed5 {
        unsafe { transmute(self) }
    }
    pub fn gt_entry(&mut self) -> *mut Struct_Unnamed6 {
        unsafe { transmute(self) }
    }
}
pub struct RegInfo {
    ri_gprmask: Word,
    ri_cprmask: [Word, ..4u],
    ri_gp_value: Sword,
}
pub struct Elf_Options {
    kind: c_uchar,
    size: c_uchar,
    section: Section,
    info: Word,
}
pub struct Elf_Options_Hw {
    hwp_flags1: Word,
    hwp_flags2: Word,
}
pub struct Lib {
    l_name: Word,
    l_time_stamp: Word,
    l_checksum: Word,
    l_version: Word,
    l_flags: Word,
}

pub type Conflict = Addr;
