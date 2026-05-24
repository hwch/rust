use std::cell::RefCell;

fn main() {
    let x: RefCell<i32> = RefCell::new(5);
    let mut y = x.borrow_mut();
    *y = 1;
    println!("{:?}", x);
}
