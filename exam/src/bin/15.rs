#[derive(Debug)]
enum Cons {
    List(i32, Box<Cons>),
    Nil,
}

fn main() {
    let x = Cons::List(
        1,
        Box::new(Cons::List(2, Box::new(Cons::List(3, Box::new(Cons::Nil))))),
    );

    println!("{:?}", x);
}
