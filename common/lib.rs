#![crate_name = "main"]
#![crate_type = "staticlib"]
#![no_std]
#![feature(plugin, no_std, asm, macro_rules, default_type_params, phase, globs, lang_items, intrinsics)]

// The plugin phase imports compiler plugins, including regular macros.

#[plugin]
extern crate core;

#[cfg(target_arch = "x86")]
pub use platform::runtime::{memset, memcpy, memmove};
#[cfg(target_arch = "arm")]
pub use rust_core::support::{memcpy, memmove};

pub use platform::cpu;
pub use kernel::util;

mod macros;

mod rust_core;

pub mod kernel;

#[allow(dead_code)]
#[cfg(target_arch = "x86")]
#[path = "../arch/x86/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
    pub mod runtime;
}

#[allow(dead_code)]
#[cfg(target_arch = "arm")]
#[path = "../arch/arm/"]
mod platform {
    pub mod cpu;
    pub mod io;
    pub mod drivers;
}

mod std {
    // macros refer to absolute paths
    pub use core::{cmp, clone};
    pub use core::fmt;
    pub use core::option;
}
