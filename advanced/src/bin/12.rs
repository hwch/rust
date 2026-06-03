#![feature(macro_metavar_expr)]

macro_rules! foo {
    () => {
        macro_rules! bar {
            ( $$( $$any:tt )* ) => { $$( $$any )* };
        }
    };
}

fn main() {
    foo!();
    println!(bar!("haha"));
}
