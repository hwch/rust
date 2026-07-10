macro_rules! i3 {
    () => {
        let mut i0:i32;
    };
    ($($x:ident),*) => {$(
                let mut $x:i32;
            )*

    };
}

fn main() {
    i3![x, y, z, a, b, c];
}
