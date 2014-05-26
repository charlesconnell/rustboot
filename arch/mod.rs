pub mod common;

#[cfg(target_arch = "x86")]
pub mod i686 {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
    #[allow(dead_code)]
    pub mod runtime;
}

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
    pub mod runtime;
    pub mod efi;
}

#[cfg(target_arch = "arm")]
pub mod arm {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}
