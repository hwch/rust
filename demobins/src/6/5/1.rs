// fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String,
}
fn main() {
    let age = 30;
    let hobby = "Hello".to_string();
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby,
    };
}
