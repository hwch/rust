type MyNum = i64;
fn main() {
    const COUNTER: MyNum = 20;
    let mut res = 1;
    for n in 2..=COUNTER {
        res = gcm(res, n);
        println!("{n}: {}", res);
    }
}

/// 获取最小公倍数
fn gcm(acc: MyNum, num: MyNum) -> MyNum {
    acc * num / lcd(acc, num)
}

/// 获取最大公约数
fn lcd(acc: MyNum, num: MyNum) -> MyNum {
    if acc % num != 0 {
        lcd(num, acc % num)
    } else {
        num
    }
}
