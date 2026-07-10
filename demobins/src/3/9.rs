use timer_future::{self, MyFuture};

fn main() {
    let r = timer_future::exeute_async_fn(MyFuture::new(10));
    println!("Completed:{:?}", r);
}
