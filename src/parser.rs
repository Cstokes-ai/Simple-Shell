// do this file first

/*
parser.rs
Responsibility: Handles parsing of user input into command and arguments.
No input reading here—just parsing and sanitization.
*/

pub fn parse_input(input: &str) -> Vec<String> {
    input.trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
    
}