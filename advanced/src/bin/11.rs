macro_rules! dead_rule {
    ($i:ident +) => {};
    ($e:expr) => {};
}

fn main() {
    dead_rule!(x+);
}
