use rand::prelude::*;
use std::cmp::Ordering;
use std::io::{self, Write};

const MAX_GUESSES: u8 = 5;
const MIN_NUM: u32 = 1;
const MAX_NUM: u32 = 100;

fn main() {
    println!("Guess the number between {MIN_NUM} and {MAX_NUM}!");

    loop {
        let secret_number = rand::rng().random_range(MIN_NUM..=MAX_NUM);

        for i in 1..=MAX_GUESSES {
            println!("\n[{i}/{MAX_GUESSES}] Please input your guess.");
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read your guess");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) if (MIN_NUM..=MAX_NUM).contains(&num) => num,
                _ => {
                    println!(
                        "Invalid input. Please enter a number between {MIN_NUM} and {MAX_NUM}."
                    );
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

            if i == MAX_GUESSES {
                println!("\nYou've run out of guesses!");
                println!("The secret number was {}.", secret_number);
            }
        }

        print!("\nWould you like to play again [Y/n]?: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read your answer");

        if play_again.trim() == "n" {
            break;
        }
    }
}
