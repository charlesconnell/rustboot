use core::mem::transmute;

use platform::cpu::Context;
use kernel::elf;
use kernel::process::Process;

macro_rules! syscall(
    (fn $name:ident($($param:ident: $T:ty),*) -> $ret:ty $func:expr) => (
        fn $name(regs: &mut Context) {
            match regs.syscall_args() {
                [$($param),*, ..] => {
                    $(let $param = $param as $T;)*
                    let ret: $ret = { $func };
                    regs.set_arg(ret);
                }
            }
        }
    );
)

pub fn handler(ctx: &mut Context) {
    match ctx.get_arg() {
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
        11 => {
            execve(ctx)
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
        other => {
            println!("{}", other);
        }
    }
}

pub fn init() {
}

syscall!(fn exit(error_code: int) -> uint {
    println!("{}", error_code);
    0
})

syscall!(fn read(fd: uint, buf: *u8, count: uint) -> uint {
    0
})

syscall!(fn write(fd: uint, buf: *u8, count: uint) -> uint {
    unsafe {
        ::core::slice::raw::buf_as_slice(
            buf, count, |slice| println!("{}", slice));
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

syscall!(fn execve(x: uint) -> uint {
    // TODO: accept arguments
    elf::load(&_binary_initram_elf_start).map(|addr| {
        Process::jump(addr)
    });
    extern { static _binary_initram_elf_start: u8; }
    0
})

// syscall!(fn getuid() -> uint {
//     0
// })

syscall!(fn set_thread_area(entry: */*platform::cpu::IdtEntry*/uint) -> uint {
    0
})
