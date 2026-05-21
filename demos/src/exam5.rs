use std::ops::Deref;
use std::ops::Drop;

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hello(s: &str) {
    println!("Name: {s}")
}

pub struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    pub fn new(s: String) -> CustomSmartPointer {
        CustomSmartPointer { data: s }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{}", self.data)
    }
}
