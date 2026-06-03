// 注释这段声明宏的定义看看会发生什么
// macro_rules! use_local {
//     () => {
//         local = 42;
//     };
// }
fn main() {
    let mut local = 0;
    // 取消注释这段声明宏的定义看看会发生什么
    macro_rules! use_local {
        () => {
            local = 42;
        };
    }
    use_local!();
}
