use std::sync::mpsc;

fn main() {
    let (s, r) = mpsc::channel();

    let th = std::thread::spawn(move || {
        println!("Sending...");
        s.send("Hello World").unwrap();
    });

    // println!("Recieve: {}", r.recv().unwrap());
    th.join().unwrap();
}
