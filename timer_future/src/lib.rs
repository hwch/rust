use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::runtime::Runtime;

mod timer_future;

pub use timer_future::TimerFuture;

pub struct MyFuture {
    counter: i32,
}

impl Future for MyFuture {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        if self.counter == 0 {
            Poll::Ready("Liftoff!")
        } else {
            self.counter -= 1;
            println!("Counter:{}", self.counter);
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl MyFuture {
    pub fn new(counter: i32) -> Self {
        Self { counter }
    }
}

pub fn exeute_async_fn<T>(fut: T) -> T::Output
where
    T: Future,
{
    // 创建一个运行时
    let rt = Runtime::new().expect("创建运行时失败");

    // block_on 会阻塞当前线程，直到 Future 完成
    rt.block_on(async {
        // 在这里你可以使用 .await
        fut.await
    })
}
