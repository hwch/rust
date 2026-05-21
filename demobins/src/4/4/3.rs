use std::thread;
use std::time;
// 用两种方法求解
fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    loop {
        println!("I will never return");
        thread::sleep(time::Duration::from_secs(1));
    }
    // panic!("I will never return");
}
