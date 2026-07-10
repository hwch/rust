use std::rc::Rc;
fn main() {
    let a = Rc::new(Box::new(3));
    println!("{}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("{}", Rc::strong_count(&b));
}
