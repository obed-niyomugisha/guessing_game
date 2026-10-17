use rand::prelude::*;
use std::cmp::Ordering;
use std::io::{self, Write};

const MAX_ATTEMPTS: u8 = 5;
const MIN_NUMBER: u8 = 1;
const MAX_NUMBER: u8 = 100;

fn main() {
    println!("Guess the number between {MIN_NUMBER} and {MAX_NUMBER}!");

    loop {
        play_round();

        let play_again = input("\nWould you like to play again [Y/n]?: ");
        if play_again.to_lowercase() == "n" {
            break;
        }
    }
}

fn input(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line.trim().to_string()
}

fn play_round() {
    let secret_number = rand::rng().random_range(MIN_NUMBER..=MAX_NUMBER);

    for attempt in 1..=MAX_ATTEMPTS {
        match read_guess(attempt).cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if attempt == MAX_ATTEMPTS {
            println!("\nYou've run out of guesses!");
            println!("The secret number was {secret_number}.",);
        }
    }
}

fn read_guess(attempt: u8) -> u8 {
    loop {
        println!("\n[{attempt}/{MAX_ATTEMPTS}] Please input your guess.");

        match input("> ").parse() {
            Ok(num) if (MIN_NUMBER..=MAX_NUMBER).contains(&num) => return num,
            _ => println!(
                "Invalid input. Please enter a number between {MIN_NUMBER} and {MAX_NUMBER}."
            ),
        }
    }
}
