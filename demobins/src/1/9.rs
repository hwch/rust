struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
fn main() {
    let x = Point::<i32> { x: 1, y: 2 };
    println!("{},{}", x.x(), x.y());
}
