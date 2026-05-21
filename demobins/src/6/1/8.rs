#![allow(unused_variables)]
// 使用两种方法来解决错误，不要新增代码行
fn main() {
    // let s = "hello, world".to_string();
    let s = "hello, world";
    let s1: &str = &s;
    // let s1: &str = s.as_str();
}
