use std::io::{self, Write};

pub fn run_cli() {
    loop {
        println!("\n=== Rust Beginner Toolkit ===");
        println!("1. Add two numbers");
        println!("2. Reverse a string");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Enter first number:");
                let a = read_number();
                println!("Enter second number:");
                let b = read_number();
                println!("Result: {}", a + b);
            }
            "2" => {
                println!("Enter text:");
                let mut text = String::new();
                io::stdin().read_line(&mut text).unwrap();
                println!("Reversed: {}", text.trim().chars().rev().collect::<String>());
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}

fn read_number() -> i32 {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        if let Ok(n) = buf.trim().parse::<i32>() {
            return n;
        }
        println!("Invalid number, try again:");
    }
}
