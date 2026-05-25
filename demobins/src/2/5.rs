use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (s, r) = mpsc::channel();
    let s1 = s.clone();
    thread::spawn(move || {
        let v = vec![1, 2, 3];
        for x in v {
            s.send(x).unwrap();
            thread::sleep(Duration::from_micros(500));
        }
    });

    thread::spawn(move || {
        let v = vec![4, 5, 6];
        for x in v {
            s1.send(x).unwrap();
            thread::sleep(Duration::from_millis(1_401));
        }
    });

    for s in r {
        println!("Recieved: {s}");
    }
    println!("Done!");
}
