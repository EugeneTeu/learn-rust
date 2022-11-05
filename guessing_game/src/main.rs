use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        println!("Guess a number");
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("you failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("you have guessed smaller"),
            Ordering::Greater => println!("you have guessed bigger"),
            Ordering::Equal => {
                println!("you have won!");
                break;
            }
        }
        println!("secret number is {}", secret_number);
    }
}
