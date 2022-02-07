use std::io::{self, Write};

fn main() {
    let mut num: i32;
    loop {
        let mut str = String::new();
        print!("Enter data:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut str).expect("Failed to read");

        num = match str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        if num < 0 {
            println!("Non negative number only");
            continue;
        } else {
            break;
        }
    }

    println!("Fibonacci number is: {}", fib(num));
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}
