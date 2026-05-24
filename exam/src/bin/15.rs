use std::any::{Any, TypeId};

#[derive(Debug, PartialEq)]
enum Cons {
    List(i32, Box<Cons>),
    Nil,
}

fn print_cons(cons: Cons) {
    if TypeId::of::<Cons>() == cons.type_id() {
        println!("List");
        return;
    }
    println!("Nil");
}
fn main() {
    let x = Cons::List(
        1,
        Box::new(Cons::List(2, Box::new(Cons::List(3, Box::new(Cons::Nil))))),
    );
    print_cons(x);
}
