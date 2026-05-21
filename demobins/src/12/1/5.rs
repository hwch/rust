fn main() {
    let mut arr: [u64; 13] = [0; 13];
    arr[0] = 0x1234567890abcdef;
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *mut [u8];
    unsafe {
        println!("{:?}", &*b);

        assert_eq!(std::mem::size_of_val(&*b), 13)
    }
}
