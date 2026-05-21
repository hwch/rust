use std::rc::Rc;

pub enum ConsList {
    Cons(i32, Rc<ConsList>),
    Nil,
}

use ConsList::{Cons, Nil};

pub fn exam6() {
    let a = Rc::new(Cons(12, Rc::new(Cons(8, Rc::new(Cons(10, Rc::new(Nil)))))));
    println!("the strong count of a is {}", Rc::strong_count(&a));
    let c = Cons(16, Rc::clone(&a));
    println!("the strong count of a is {}", Rc::strong_count(&a));
    {
        let b = Cons(14, Rc::clone(&a));
        println!("the strong count of a is {}", Rc::strong_count(&a));
    }
    println!("the strong count of a is {}", Rc::strong_count(&a))
}
