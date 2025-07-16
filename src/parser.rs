// do this file first

/*
parser.rs
Responsibility: Handles user input and breaks it down into usable components.
What it does:

Reads input from standard input.
Strips whitespace and newline characters.
Splits the input into command and arguments (usually a Vec<String>).
Prepares the command in a format that executor.rs can use.
No execution logic happens here — only parsing and sanitization.
*/

pub fn user_input() -> Vec<String> {
    use std::io::{self, Write};
    print!("simpleshell> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}