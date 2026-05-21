// 填空
fn main() {
    let mut s = String::new();
    s.push(104 as char);
    s.push(101 as char);
    s.push(108 as char);
    s.push(108 as char);
    s.push(111 as char);

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    let s1 = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("Success!")
}
