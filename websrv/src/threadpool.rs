use crate::IError;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Job = Box<dyn FnOnce() + Send + 'static>;
type SyncJob = Arc<Mutex<mpsc::Receiver<Job>>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: u128,
    th: thread::JoinHandle<()>,
}

impl Worker {
    fn build(id: u128, receiver: SyncJob) -> Result<Worker, IError> {
        let builder = thread::Builder::new();
        let th = builder.spawn(move || {
            loop {
                let job = receiver
                    .lock()
                    .expect("获取锁失败")
                    .recv()
                    .expect("读取信道失败");
                println!("Worker {id} got a job; executing.");
                job();
            }
        })?;
        Ok(Worker { id, th })
    }
}

impl ThreadPool {
    /// 创建一个新的线程池。
    ///
    /// size 是池中线程的数量。
    ///
    /// # Panics
    ///
    /// 如果 size 为 0，`new` 方法会 panic。
    pub fn build(size: usize) -> Result<ThreadPool, IError> {
        if size == 0 {
            return Err(IError::PoolCreationError);
        }
        let mut threads = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let r = Arc::new(Mutex::new(receiver));
        for no in 0..size {
            // 创建一些线程并将它们存入 vector 中。

            threads.push(Worker::build(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() + no as u128,
                Arc::clone(&r),
            )?);
        }
        Ok(ThreadPool {
            workers: threads,
            sender,
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn scan(&self) {
        for v in &self.workers {
            println!("Thread[{}] is running", v.id);
        }
    }
}
