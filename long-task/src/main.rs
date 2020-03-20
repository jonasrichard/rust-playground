extern crate futures;

#[allow(unused_imports)]
use std::future::Future;
use std::thread;
use std::time::{Duration, Instant};

use futures::executor;
use futures::future::join_all;

fn task1(num: i32) -> impl Future<Output = i32> {
    async move {
        thread::sleep(Duration::from_millis(100));

        num
    }
}

fn main() {
    let mut tasks = Vec::new();

    for i in 1..10 {
        tasks.push(task1(i));
    }

    let now = Instant::now();

    for fut in tasks {
        let value: i32 = executor::block_on(fut);

        println!("{}", value);
    }

    println!("Took {:?} time", now.elapsed());

    let mut tasks2 = Vec::new();

    for i in 1..10 {
        tasks2.push(task1(i));
    }

    println!("Join all");
    let now2 = Instant::now();

    let results = executor::block_on(join_all(tasks2));

    println!("{:?}", results);
    println!("Took {:?} time", now2.elapsed());
}
