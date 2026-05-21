use std::ops::Add;

pub fn add<T>(a1: T, a2: T) -> T
where
    T: Add<Output = T>,
{
    a1 + a2
}

pub trait Summary {
    fn hello(&self) -> String {
        "Hello World".to_string()
    }
}

pub struct Hello {
    str: String,
}

impl Hello {
    pub fn new(str: String) -> Hello {
        Hello { str }
    }
}

impl Summary for Hello {
    fn hello(&self) -> String {
        self.str.clone()
    }
}

pub fn world(s: &impl Summary) {
    println!("Call Summary trait: {}", s.hello())
}
