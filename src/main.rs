use rand::prelude::*;
use std::cmp::Ordering;
use std::io::{self, Write};

const MIN_NUMBER: u8 = 1;

fn main() {
    println!("Choose level: ");
    println!("1. Easy");
    println!("2. Medium (Default)");
    println!("3. Hard\n");

    let level = match input("> ").parse() {
        Ok(1) => Difficulty::Easy,
        Ok(3) => Difficulty::Hard,
        _ => Difficulty::Medium,
    };

    let game = Game::new(&level);

    println!(
        "\nGuess the number between {MIN_NUMBER} and {}!",
        game.max_number
    );

    loop {
        game.play_round();

        let play_again = input("\nWould you like to play again [Y/n]?: ");
        if play_again.to_lowercase() == "n" {
            break;
        }
    }
}

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

struct Game {
    max_attempts: u8,
    max_number: u8,
}

impl Game {
    fn new(difficulty: &Difficulty) -> Game {
        match difficulty {
            Difficulty::Easy => Game {
                max_attempts: 10,
                max_number: 50,
            },
            Difficulty::Medium => Game {
                max_attempts: 5,
                max_number: 75,
            },
            Difficulty::Hard => Game {
                max_attempts: 3,
                max_number: 100,
            },
        }
    }

    fn play_round(&self) {
        let secret_number = rand::rng().random_range(MIN_NUMBER..=self.max_number);

        for attempt in 1..=self.max_attempts {
            match self.read_guess(attempt).cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too large!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }

            if attempt == self.max_attempts {
                println!("\nYou've run out of guesses!");
                println!("The secret number was {secret_number}.",);
            }
        }
    }

    fn read_guess(&self, attempt: u8) -> u8 {
        loop {
            println!(
                "\n[{attempt}/{}] Please input your guess.",
                self.max_attempts
            );

            match input("> ").parse() {
                Ok(num) if (MIN_NUMBER..=self.max_number).contains(&num) => return num,
                _ => println!(
                    "Invalid input. Please enter a number between {MIN_NUMBER} and {}.",
                    self.max_number
                ),
            }
        }
    }
}

fn input(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut line = String::new();

    let bytes_read = io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    if bytes_read == 0 {
        println!("\nInput closed. Goodbye!");
        std::process::exit(0);
    }

    line.trim().to_string()
}
