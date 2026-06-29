// 修复所有的 panic，让代码工作
fn main() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v = vec![1, 2, 3];
    let _ele = v[2];
    let _ele = v.get(3);

    // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
    let _v = production_rate_per_hour(2);

    divide(15, 1);
    working_items_per_minute(4);
    let n1 = "8".parse::<i32>()?;
    println!("Success!");
    Ok(())
}

fn divide(x: u8, y: u8) {
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;
    match speed {
        vv @ 1..=4 => {
            println!("binding mode: {vv}");
            (speed.wrapping_mul(cph)) as f64
        }
        5..=8 => (speed.wrapping_mul(cph)) as f64 * 0.9,
        9..=10 => (speed.wrapping_mul(cph)) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}
