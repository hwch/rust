#[allow(unused_variables)]
// 解决代码中的错误和 `panic`
fn main() {
    let v1: u8 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    let v3 = 3;
    println!("{},{}", v1, v2);
}
