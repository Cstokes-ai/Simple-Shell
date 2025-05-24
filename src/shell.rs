//this one is third.

/*
shell.rs
Responsibility: Coordinates the core shell logic.
What it does:

Implements the main loop: read input → parse → execute.

Calls the appropriate functions from parser.rs and executor.rs.

Handles flow control like checking for the "exit" command.

This is where your shell "lives" — think of it as the orchestrator.
*/

use crate::parser;
use crate::executor;

pub fn shell() {
    // Load command descriptions once before the loop
    let command_descriptions = executor::load_command_descriptions("src/commands.txt");

    loop {
        // Prompt and parse user input once per loop
        let args = parser::user_input();

        // If no input, continue to next iteration
        if args.is_empty() {
            continue;
        }

        // Check if the command is "exit"
        if args[0] == "exit" {
            println!("Exiting shell...");
            break;
        }

        // Execute the command
        executor::execute_command(args, &command_descriptions);
    }
}