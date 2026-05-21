use std::thread;

pub fn exam3() {
    let mut data = vec![1, 2, 3];

    println!("{:?}", data);
    let mut add_value = || data.push(4);
    // println!("{data:?}");
    add_value();

    data.push(5);
    println!("{data:?}")
}

pub fn move_borrow() {
    let list = vec![1, 2, 3];

    thread::spawn(move || println!("{list:?}")).join().unwrap();
}
