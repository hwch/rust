use std::cell::Cell;

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", v);
    retain_even(&mut v);
    println!("{:?}", v);
}
