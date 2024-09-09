mod future;
mod http;
mod runtime;

use std::time::Instant;
use crate::http::Http;
use future::*;


fn main(){
    let future = async_main();
    let mut runtime=Runtime::new();
    runtime.block_on(future);
}

coroutine fn async_main(){
    println!("Program starting");
    let txt = Http::get("/600/HelloAsyncAwait").wait;
    println!("{txt}");

    let txt = Http::get("/400/HelloAsyncAwait").wait;
    println!("{txt}");
}


