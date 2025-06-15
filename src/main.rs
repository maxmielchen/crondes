#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(1)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Beispiel: Schreibe "Hello, world!\n" auf stdout (fd 1)
    let msg = b"Hello, world!\n";
    let _ = write(1, msg.as_ptr(), msg.len());
    exit(0)
}

fn write(fd: usize, buf: *const u8, count: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // sys_write
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") ret,
        );
    }
    ret
}

fn exit(code: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60, // sys_exit
            in("rdi") code,
            options(noreturn)
        );
    }
    loop {}
}
