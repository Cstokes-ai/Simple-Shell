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


pub fn user_input()-> Vec<String> {
    let mut input = String:: new();
    std:: io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim(); // Remove whitespace and newline characters
    let input = input.to_string();
    //now we need to split input into command line arguments using Vec<String>
    let args: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    //this gets the commands ready in a format for my executor..rs
    
    let mut command = String::new();
    for args in args.iter() {
        command.push_str(args);
        command.push(' ');
    }
    command.pop();

    args
}