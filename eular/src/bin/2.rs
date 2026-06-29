fn main() {
    let mut v = Vec::with_capacity(64);
    let mut prev = 1;
    let mut next = 2;
    v.extend(vec![prev, next]);

    for num in 3..=4_000_000 {
        if num == next + prev {
            (prev, next) = (next, num);
            print!("{num}, ");
            v.push(num);
        }
    }
    println!();
    println!("{:?}", v.iter().filter(|&x| x % 2 == 0).sum::<i32>());

    let mut prev = 1;
    let mut next = 2;

    let v = (3..=4_000_000)
        .filter(|&x| {
            if x == next + prev {
                (prev, next) = (next, x);
                print!("{x}, ");
                return true;
            }
            return false;
        })
        .filter(|&x| x % 2 == 0)
        .sum::<i32>()
        + 2;
    println!();
    println!("{:?}", v);

    let (mut f1, mut f2) = (2, 8);
    let mut total = f1 + f2;
    loop {
        let next_val = 4 * f2 + f1;
        if next_val > 4_000_000 {
            break;
        }
        (f1, f2) = (f2, next_val);
        total += next_val;
    }
    println!("sum:{total}");
}
