//! Higher-level input/output interface.

use core::fmt;
use core::prelude::*;

use super::drivers::vga;

/// A format writer that writes out to the VGA screen.
struct Stdout;

impl Stdout {
    fn write_fmt(&mut self, fmt: &fmt::Arguments) {
        fmt::write(self, fmt);
    }
}

impl fmt::FormatWriter for Stdout {
    fn write(&mut self, bytes: &[u8]) -> fmt::Result {
        for &c in bytes.iter() {
            putc(c);
        }
        Ok(())
    }
}

pub fn print_args(fmt: &fmt::Arguments) {
    write!(&mut Stdout, "{}", fmt);
}

pub fn println_args(fmt: &fmt::Arguments) {
    writeln!(&mut Stdout, "{}", fmt);
}

static mut pos: isize = 0;

unsafe fn seek(offset: isize) {
    pos += offset;
}

unsafe fn write_char(c: char) {
    if c == '\x08' {
        if pos > 0 {
            if pos % 80 == 0 {
                while (*vga::SCREEN)[(pos-1) as usize].char == 0 {
                    pos -= 1;
                }
            }
            else if pos > 0 {
                pos -= 1;
                (*vga::SCREEN)[pos as usize].char = 0;
            }
        }
    }
    else if c == '\n' {
        seek(80 - pos % 80);
    }
    else if c == '\t' {
        seek(4 - pos % 4);
    }
    else {
        (*vga::SCREEN)[pos as usize].char = c as u8;
        pos += 1;
    }

    pos %= vga::SCREEN_SIZE as isize;
    vga::cursor_at(pos as usize);
}

pub fn putc(c: u8) {
    unsafe {
        write_char(c as char);
    }
}

pub fn puti(num: isize) {
}

pub fn puts(s: &str) {
    for c in s.as_bytes().iter() {
        putc(*c);
    }
}
