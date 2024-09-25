mod future;
mod http;
mod runtime;

use runtime::Waker;
use crate::{http::Http};
use future::*;


fn main() {
    let mut executor = runtime::init();
    let mut handlers = vec![];

    for i in 1..16{
        let name = format!("exec-{i}"); 
        let h = std::thread::Builder().name(name).spawn(move ||{
            let mut executor = Executor::new();
            executor.block_on(async_main());
        }).unwrap();

        handlers.push(h);
    }
    executor.block_on(async_main());
    handles.into_iter().for_each(|h| h.join().unwrap());
}

coroutine fn request(i: usize) {
    let path = format!("/{}/HelloWorld{i}", i*1000);
    let txt = Http::get(&path).wait;
    let txt = txt.lines().last().unwrap_or_default();

    println!("{txt}");
}

coroutine fn async_main(){
    println!("Program starting");
    for i in 0..5{
        let future = request(i);
        runtime::spawn(future);
    }
}


