use std::task::{Context, Poll, Waker};
use std::{sync::LazyLock, thread};

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = &LOGGER;
        logger.log("thread message");
    });

    // 主线程调用
    let logger = &LOGGER;
    logger.log("some message");

    let logger2 = &LOGGER;
    logger2.log("other message");

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

static LOGGER: LazyLock<Logger> = LazyLock::new(Logger::new);
impl Logger {
    fn new() -> Self {
        println!("Logger is being created...");
        Logger
    }
    fn log(&self, message: &str) {
        println!("{message}");
    }
}
