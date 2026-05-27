use std::{future::Future, time::Duration};
use trpl::Either;

async fn timeout<T, A>(f1: T, times: Duration) -> Result<A, Duration>
where
    T: Future<Output = A>,
{
    match trpl::select(f1, trpl::sleep(times)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(times),
    }
}

async fn timeout2<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
