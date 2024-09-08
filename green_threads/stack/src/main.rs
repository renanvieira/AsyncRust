use std::arch::asm;

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK");
    println!("This is a new stack indeed.");
    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm! {
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new,
    };
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];

    unsafe {
        // Gets the bottom of the stack by offsetting the pointer to the stack size (SSIZE)
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        // Gets the stack bottom 16-bytes aligned, since its not garanteed that we will always get
        //   the pointer aligned
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        // Write the function pointer (hello) to the bottom of the stack)
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        // Assign the bottom of the stack, that now points to the hello function into the thread
        // context
        ctx.rsp = sb_aligned.offset(-16) as u64;


        // Prints the pointer address (mem) and its value.
        //   in this case it will print the hello function pointer as u8
        for i in 0..SSIZE {
            println!(
                "mem: {:?}, val: {}",
                sb_aligned.offset(-i as isize),
                *sb_aligned.offset(-i as isize)
            )
        }

        // Instruct the CPU to jump to the ThreadContext stack.
        gt_switch(&mut ctx);
    }
}
