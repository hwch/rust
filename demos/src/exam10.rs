extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn exam() {
    unsafe { println!("Absolute value of -1 is {}", abs(-1)) }
}
