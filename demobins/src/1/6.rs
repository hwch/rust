fn main() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );

    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
    let quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;
    if let Ok(s) = str::from_utf8(quotes) {
        println!("{s}");
    }
}
