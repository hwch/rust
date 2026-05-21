use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn exam7() {
    let th = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    th.join().unwrap();

    println!("===========>exam7!");
}

pub fn exam71() {
    let v = vec![1, 2, 3];
    let x = thread::spawn(move || println!("Value:{:?}", v));

    x.join().unwrap();
    println!("===========>exam71!");
}

pub fn exam72() {
    let (tx, rx) = mpsc::channel();

    let th = thread::spawn(move || {
        let x = String::from("Hello World");
        if let Err(err) = tx.send(x) {
            println!("Error: {:?}", err);
        }
    });

    match rx.recv() {
        Err(e) => println!("{e}"),
        Ok(v) => println!("Succ:{v}"),
    }
    if let Err(e) = th.join() {
        println!("Join error:{e:?}");
    }
    println!("===========>exam72!");
}

pub fn exam73() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for x in v {
            if let Err(e) = tx.send(x) {
                eprintln!("Send message failed: {e}");
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    for x in rx {
        println!("Recieved: {}", x);
    }
    println!("===========>exam73!");
}

pub fn exam74() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let v = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for x in v {
            if let Err(e) = tx.send(x) {
                eprintln!("Send message failed: {e}");
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let v = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];

        for x in v {
            if let Err(e) = tx1.send(x) {
                eprintln!("Send message failed: {e}");
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    for x in rx {
        println!("Recieved: {}", x);
    }
    println!("===========>exam74!");
}
