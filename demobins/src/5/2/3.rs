#![cfg_attr(debug_assertions, allow(unused_variables, unused_mut))]
// 修复错误
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s)
}

fn borrow_object(s: &String) {}
