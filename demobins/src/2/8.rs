use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let m1 = Arc::new(Mutex::new(5));
    let m2 = Arc::new(Mutex::new(25));

    {
        let m1 = Arc::clone(&m1);
        let m2 = Arc::clone(&m2);
        let th = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            *m1.lock().unwrap() = 1;
            thread::sleep(Duration::from_secs(2));
            *m2.lock().unwrap() = 2;
        });
        th.join().unwrap();
    }
    {
        let m1 = Arc::clone(&m1);
        let m2 = Arc::clone(&m2);
        let th = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            *m2.lock().unwrap() = 11;
            thread::sleep(Duration::from_secs(2));
            *m1.lock().unwrap() = 22;
        });
        th.join().unwrap();
    }
    println!("{:?},{:?}", m1, m2);
}
