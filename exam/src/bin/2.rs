fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    if let Some(&third) = v.get(4) {
        println!("The third element is {third}");
    };

    for i in &v {
        println!("{i:p}");
    }
    println!("The third element is {third}");
}
