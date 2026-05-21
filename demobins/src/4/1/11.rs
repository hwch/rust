// 填空，并解决错误
fn main() {
    // 整数加法
    assert!(1u32 + 2 == 3_u32);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150i32);

    assert!((9.6f64 / 3.2 - 3.0).abs() < 0.00001); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); //0b0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); //0b0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); //0b0110
    println!("1 << 5 is {}", 1u32 << 5); //0b10_0000
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //0b10_0000
}
