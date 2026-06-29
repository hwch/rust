use std::collections::HashMap;

fn main() {
    let mut v = HashMap::with_capacity(64);
    for i in (100..=999).rev() {
        for j in (100..=i).rev() {
            let temp_max = i * j;
            let palindromic = format!("{temp_max}");
            let new_pdc: String = palindromic.chars().rev().collect();
            if palindromic == new_pdc {
                v.insert(temp_max, format!("{i}*{j}"));
            }
        }
    }
    let i_max = v.iter().map(|(k, _)| k).max().unwrap();
    println!("{}=>{i_max}", v.get(i_max).unwrap());
}
