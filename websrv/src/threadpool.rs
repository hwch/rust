use crate::IError;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Job = Box<dyn FnOnce() + Send + 'static>;
type SyncJob = Arc<Mutex<mpsc::Receiver<Job>>>;
type SomeSender = Option<mpsc::Sender<Job>>;
type Handle = thread::JoinHandle<()>;
#[cfg(feature = "some")]
type SomeHandle = Option<Handle>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: SomeSender,
}

struct Worker {
    id: u128,
    #[cfg(feature = "some")]
    th: SomeHandle,
    #[cfg(feature = "drain")]
    th: Handle,
}

impl Worker {
    fn build(id: u128, receiver: SyncJob) -> Result<Worker, IError> {
        let builder = thread::Builder::new();
        #[cfg(feature = "some")]
        let th = Some(builder.spawn(move || {
            loop {
                let job = receiver.lock().expect("获取锁失败").recv();

                match job {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        })?);

        #[cfg(feature = "drain")]
        let th = builder.spawn(move || {
            loop {
                let job = receiver.lock().expect("获取锁失败").recv();
                match job {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
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
    /// 如果 size 为 0，`build` 方法会 返回PoolCreationError。
    pub fn build(size: usize) -> Result<ThreadPool, IError> {
        if size == 0 {
            return Err(IError::PoolCreationError);
        }
        let mut threads = Vec::with_capacity(size);

        let (isender, receiver) = mpsc::channel();
        let r = Arc::new(Mutex::new(receiver));
        for no in 0..size {
            // 创建一些线程并将它们存入 vector 中。

            threads.push(Worker::build(
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() + no as u128,
                Arc::clone(&r),
            )?);
        }
        let sender = Some(isender);
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
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        #[cfg(feature = "some")]
        for v in &mut self.workers {
            println!("Shutting down worker {}", v.id);
            if let Err(e) = v.th.take().unwrap().join() {
                //不在调用unwrap让其再次panic
                eprintln!("线程[{}]停止失败:{:?}", v.id, e);
            }
        }

        #[cfg(feature = "drain")]
        for v in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", v.id);
            if let Err(e) = v.th.join() {
                //不在调用unwrap让其再次panic
                eprintln!("线程[{}]停止失败:{:?}", v.id, e);
            }
        }
    }
}
