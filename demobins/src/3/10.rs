use std::time::Duration;

use timer_future::{self, TimerFuture};

fn main() {
    let r = timer_future::exeute_async_fn(TimerFuture::new(Duration::from_secs(5)));
    println!("Completed:{:?}", r);
}
