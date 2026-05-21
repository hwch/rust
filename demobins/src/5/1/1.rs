fn main() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = &x;
    println!("{},{}", x, y);
    let y = x.clone();
    println!("{},{}", x, y);
    let y = x.as_str();
    println!("{},{}", x, y);
}
