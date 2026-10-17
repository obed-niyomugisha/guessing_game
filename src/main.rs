mod utils;
use utils::*;

use rand::prelude::*;
use std::cmp::Ordering;

const MAX_GUESSES: u8 = 5;
const MIN_NUM: u8 = 1;
const MAX_NUM: u8 = 100;

fn main() {
    println!("Guess the number between {MIN_NUM} and {MAX_NUM}!");

    loop {
        play_round();

        if input_with_message("\nWould you like to play again [Y/n]?: ") == "n" {
            break;
        }
    }
}

pub fn play_round() {
    let secret_number = rand::rng().random_range(MIN_NUM..=MAX_NUM);

    for attempt in 1..=MAX_GUESSES {
        match read_guess(attempt).cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        if attempt == MAX_GUESSES {
            println!("\nYou've run out of guesses!");
            println!("The secret number was {}.", secret_number);
        }
    }
}

fn read_guess(attempt: u8) -> u8 {
    loop {
        println!("\n[{attempt}/{MAX_GUESSES}] Please input your guess.");

        let guess: u8 = match input().parse() {
            Ok(num) if (MIN_NUM..=MAX_NUM).contains(&num) => num,
            _ => {
                println!("Invalid input. Please enter a number between {MIN_NUM} and {MAX_NUM}.");
                continue;
            }
        };

        break guess;
    }
}
