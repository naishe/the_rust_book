use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let to_be_guessed = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&to_be_guessed) {
            std::cmp::Ordering::Equal => {
                println!("Yay!");
                break;
            }
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
