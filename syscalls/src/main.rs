use std::{env, io};

use os::os_syscall;
use raw::raw_syscall;

pub mod os;
pub mod raw;

fn main() -> io::Result<()> {
    let syscall_type = env::args().nth(1).unwrap_or_else(|| String::from("raw"));

    let message = String::from("Hello low level world\n");

    match syscall_type.as_ref() {
        "raw" => {
            raw_syscall(message);
        }
        "os" => os_syscall(message)?,
        _ => eprintln!("argument supported: os, raw"),
    }

    Ok(())
}
