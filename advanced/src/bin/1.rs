fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // let address = 0x012345usize;
    let r = 0x012345usize as *const i32;
    unsafe {
        *r2 = 3;
    }
    println!("num={num},r1={:p},r2={:p},r={:p}", r1, r2, r);
}
