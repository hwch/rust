use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;
pub struct TimerFuture {
    started: bool,
    duration: Duration,
    elem: Arc<Mutex<TimerElement>>,
}
struct TimerElement {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        if let Ok(mut v) = self.elem.lock() {
            if v.completed {
                return Poll::Ready(());
            }
            v.waker = Some(cx.waker().clone());
        }
        if !self.started {
            self.started = true;
            let elem_mutex = Arc::clone(&self.elem);
            let duration = self.duration;
            thread::spawn(move || {
                println!("Begin sleep {}s...", duration.as_secs());
                thread::sleep(duration);

                loop {
                    if let Ok(mut elem) = elem_mutex.try_lock() {
                        elem.completed = true;
                        elem.waker.take().unwrap().wake();
                        println!("End sleep {}s!", duration.as_secs());
                        break;
                    } else {
                        thread::yield_now();
                    }
                }
            });
        }
        Poll::Pending
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            started: false,
            elem: Arc::new(Mutex::new(TimerElement {
                completed: false,
                waker: None,
            })),
        }
    }
}
