macro_rules! multi_five {
    ($e:expr) => {
        $e * 5
    };
}

macro_rules! gibberish {
    (4 fn ['spang "whammo"] @_@) => {
        "haha"
    };
}

macro_rules! istruct {
    ($key:ident, $tp1: ty, $tp2: ty) => {
        #[derive(Debug)]
        struct $key {
            name: $tp1,
            age: $tp2,
        }
    };
}
istruct! {person, String, i32}

fn main() {
    let c = vec!["1", "2"];
    let a = multi_five!(2 + 3);

    let p = person {
        name: String::from("lili"),
        age: 32,
    };
    println!("{a}");
    println!("{:?}", p);

    println!(gibberish!(4 fn ['spang "whammo"] @_@));
}
