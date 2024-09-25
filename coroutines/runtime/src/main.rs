mod future;
mod http;
mod runtime;

use std::time::Instant;
use crate::http::Http;
use future::*;


fn main(){
    let mut executor = runtime::init();
    executor.block_on(async_main());
}

coroutine fn async_main(){
    println!("Program starting");
    let txt = Http::get("/600/HelloAsyncAwait").wait;
    println!("{txt}");

    let txt = Http::get("/400/HelloAsyncAwait").wait;
    println!("{txt}");
}


