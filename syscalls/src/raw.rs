use std::arch::asm;

#[inline(never)]
pub fn raw_syscall(message: String) {
    let ptr = message.as_ptr();
    let len = message.len();


    unsafe {
        asm! {
            "mov rax, 0x2000004", // 0x2000004 means write on macos
            "mov rdi, 1", // 1 is stdout
            "syscall",   // call kernel

            in("rsi") ptr, // address of the string
            in("rdx") len, // number of bytes

            out("rax") _,
            out("rdi") _,

            lateout("rsi") _,
            lateout("rdx") _,
        }

    }
}
