use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess a number: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Error reading input");

        println!("Your guess is: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Correct! You Win!!!");
                break;
            },
        };
    }



}
