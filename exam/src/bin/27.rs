fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(ref _s) = s {
        println!("found a string");
    }

    println!("{s:?}");
}
