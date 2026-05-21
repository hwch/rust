#![cfg_attr(
    debug_assertions,
    allow(unused_mut, unused_variables, unused_assignments)
)]
// 移除某个部分让代码工作
fn main() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;

    let z = 10u32; // 这里 z 的类型是?
}
