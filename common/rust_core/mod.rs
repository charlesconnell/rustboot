// bits of things mined from rust-core
#![macro_use]
#![allow(ctypes)]

pub mod c_types;
pub mod fail;

pub mod bitflags;

#[cfg(target_arch = "x86")]
#[macro_use]
pub mod macros;

#[cfg(target_arch = "arm")]
pub mod support;
