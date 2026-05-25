use std::thread;
use std::time::Duration;

fn main() {
    let th = thread::spawn(|| {
        for i in 1..10 {
            println!("Inner: {i}");
            // thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Outter: {i}");
        // thread::sleep(Duration::from_millis(1));
    }
    th.join().expect("Couldn't join on the associated thread");
}
