fn main() {
    let secret_number = 42;

    loop {
        // --snip--
        let mut guess = String::new();
        match std::io::stdin().read_line(&mut guess) {
            Ok(0) => {
                println!("结束读取");
                return;
            }
            Ok(_) => println!("读取: [{}]", guess.trim()),
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        }
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            _ => {
                println!("You win!");
                return;
            }
        }
    }
}
