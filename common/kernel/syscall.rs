use core::slice;
use core::mem::transmute;

use platform = arch::i686;
use cpu::Context;

macro_rules! syscall(
    (fn $name:ident($($param:ident: $T:ty),*) -> $ret:ty $func:expr) => (
        fn $name(regs: &mut Context) {
            match regs.syscall_args() {
                [$($param),*, ..] => {
                    $(let $param = $param as $T;)*
                    let ret: $ret = { $func };
                    regs.eax = ret as u32;
                }
            }
        }
    );
)

pub fn handler(ctx: &mut Context) {
    match ctx.eax {
        1 => {
            exit(ctx)
        }
        3 => {
            read(ctx)
        }
        4 => {
            write(ctx)
        }
        5 => {
            open(ctx)
        }
        6 => {
            close(ctx)
        }
        // 12 => {
        //     chdir(ctx)
        // }
        23 => {
            setuid(ctx)
        }
        243 => {
            set_thread_area(ctx)
        }
        // 25 => {
        //     getuid(ctx)
        // }
        _ => {
            platform::io::puti(ctx.eax as int);
            platform::io::putc(' ' as u8);
        }
    }
}

pub fn init() {
}

syscall!(fn exit(error_code: int) -> uint {
    platform::io::puti(error_code as int);
    0
})

syscall!(fn read(fd: uint, buf: *u8, count: uint) -> uint {
    0
})

syscall!(fn write(fd: uint, buf: *u8, count: uint) -> uint {
    unsafe {
        ::core::slice::raw::buf_as_slice(
            buf, count, |slice| platform::io::puts(slice));
    };
    count
})

syscall!(fn open(filename: *u8, flags: int, mode: uint) -> uint {
    0
})

syscall!(fn close(fd: uint) -> uint {
    0
})

syscall!(fn chdir(filename: *u8) -> uint {
    0
})

syscall!(fn setuid(uid: uint) -> uint {
    0
})

// syscall!(fn getuid() -> uint {
//     0
// })

syscall!(fn set_thread_area(entry: *platform::cpu::IdtEntry) -> uint {
    0
})
