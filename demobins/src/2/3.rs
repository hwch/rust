// 修复错误并实现缺失的代码
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v[i])
    }
    let x = Some(1);
    for i in 0..5 {
        if i > 2 {
            v.push(i);
        }
        v[i] *= 2;
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!")
}
