fn main() {
    let mut num = 600_851_475_143u64;
    let mut idx = 2;
    while idx * idx < num {
        if num % idx == 0 {
            num /= idx;
        } else {
            idx += if idx == 2 { 1 } else { 2 };
        }
    }
    println!("{num}");
}
