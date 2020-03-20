extern crate futures;
extern crate rand;

#[allow(unused_imports)]
use std::future::Future;
use std::thread;
use std::time::{Duration, Instant};

use futures::{executor, future::join_all, task::SpawnExt};

use rand::random;

fn sleep_time() -> u32 {
    200 + random::<u32>() % 3000
}

async fn task(num: i32) {
    let t = sleep_time();
    let time = Duration::from_millis(u64::from(t));

    thread::sleep(time);

    println!("{} has awaken after {:?}", num, time);
}

fn main() {
    let mut futs = Vec::new();
    let pool = futures::executor::ThreadPoolBuilder::new().pool_size(2).create().expect("I cannot");
    
    println!("Party started");
    let now = Instant::now();
    
    for i in 1..5 {
        let join_handle = pool.spawn_with_handle(task(i)).unwrap();
        futs.push(join_handle);
    }

    let res = executor::block_on(join_all(futs));

    println!("{:?}", res);
    println!("{:?}", now.elapsed());
}
