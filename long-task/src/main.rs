use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Instant};

struct LongTask {
    length_in_ms: i32,
    created_time: Instant,
}

impl LongTask {
    fn new(length: i32) -> LongTask {
        LongTask {
            length_in_ms: length,
            created_time: Instant::now()
        }
    }
}

impl Future for LongTask {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn main() {
    let tasks: Vec<LongTask> = vec![LongTask::new(400)];
}
