fn main() {
    const COUNTER: i64 = 100;
    let sum2 = (1..=COUNTER).fold(0_i64, |acc, n| acc + n).pow(2); // 和平方
    let sum1 = (1..=COUNTER)
        .map(|x| i64::pow(x, 2))
        .fold(0_i64, |acc, n| acc + n); // 平方和
    println!("{sum2}-{sum1}={}", sum2 - sum1);
}
