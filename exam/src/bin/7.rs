fn largest<T: std::cmp::PartialOrd>(v: &[T]) -> &T {
    let mut largest = &v[0];
    for item in v {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let result = largest(&v);
    println!("The largest number is {}", result);
}
