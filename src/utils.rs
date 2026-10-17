use std::io::{self, Write};

pub fn input() -> String {
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    line.trim().to_string()
}

pub fn input_with_message(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().expect("Failed to flush stdout");

    input()
}
