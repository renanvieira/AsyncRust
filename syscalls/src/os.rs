use std::io;

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

pub fn os_syscall(message: String) -> io::Result<()> {
    let ptr = message.as_ptr();
    let len = message.len(); 

    let res = unsafe { write(1, ptr, len) };

    if res == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
