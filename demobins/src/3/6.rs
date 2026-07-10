fn main() {
    let c = Box::new(5);
    let cc = Box::leak(c);
    println!("{cc}");
}
