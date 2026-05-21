use std::fmt::Debug;

// 填空
fn print_array<T: Debug, const U: usize>(arr: [T; U]) {
    println!("{:?}", arr);
}
fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
