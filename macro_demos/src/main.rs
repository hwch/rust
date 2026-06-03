use macro_demos::*;
fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1, 1, 2, 3 ; ... ; a[n-1] + a[n-2]];
    //                                        ^~~~~^ changed

    for e in fib.take(10) {
        println!("{}", e)
    }

    for e in recurrence!(f[i]: f64 = 1.0; ...; f[i-1] * i as f64).take(10) {
        println!("{}", e)
    }
}
