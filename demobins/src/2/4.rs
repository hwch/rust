use std::collections::HashMap;

fn main() {
    let v = vec![("1", 1), ("2", 2)];
    let scores: HashMap<_, _> = v.into_iter().collect();

    let score: i32 = scores.get("2").copied().unwrap_or(0);
    println!("{score}");
    for (k, v) in &scores {
        println!("{k},{v}");
    }
}
