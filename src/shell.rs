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
use crate::utils;

pub fn shell() {
    loop {
        let args = parser::user_input();

        if args.is_empty() {
            continue;
        }

        match args[0].as_str() {
            "exit" => {
                println!("Exiting shell...");
                break;
            }
            "cd" => {
                if args.len() > 1 {
                    if let Err(e) = std::env::set_current_dir(&args[1]) {
                        eprintln!("cd: {}", e);
                    }
                } else {
                    eprintln!("cd: missing argument");
                }
            }
            "clear" => utils::clear_screen(),
            "echo" => {
                println!("{}", args[1..].join(" "));
            }
            _ => {
                executor::execute_command(args);
            }
        }
    }
}