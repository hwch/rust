/* 使用两种方法填空 */
fn main() {
    // let v: &str = "hello";
    const v: &str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}
