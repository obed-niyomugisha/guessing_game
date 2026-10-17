use rand::prelude::*;
use std::cmp::Ordering;
use std::io::{self, Write};

const MAX_GUESSES: u8 = 5;

fn main() {
    println!("Guess the number between 1 and 100!");

    loop {
        let secret_number = rand::rng().random_range(1..=100);
        let mut guesses = 0;

        loop {
            if guesses >= MAX_GUESSES {
                println!("\nYou've run out of guesses!");
                println!("The secret number was {}.", secret_number);
                break;
            }

            println!(
                "\n[{}/{}] Please input your guess.",
                guesses + 1,
                MAX_GUESSES
            );
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read your guess");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) if num >= 1 && num <= 100 => num,
                _ => {
                    println!("Invalid input. Please enter a number between 1 and 100.");
                    continue;
                }
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too large!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            };

            guesses += 1;
        }

        print!("\nWould you like to play again [Y/n]?: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read your answer");

        if play_again.trim() == "n" {
            break;
        } else {
            continue;
        }
    }
}
