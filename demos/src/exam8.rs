use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn exam() {
    let m = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for _ in 0..10 {
        let o = Arc::clone(&m);
        let th = thread::spawn(move || {
            let mut n = *o.lock().unwrap();
            for i in 0..100 {
                n += i;
            }
        });
        handles.push(th)
    }

    for th in handles {
        th.join();
    }

    println!("Final result: {}", *m.lock().unwrap());
}
