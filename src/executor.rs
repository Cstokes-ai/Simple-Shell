//do this fiel second.

/*
executor.rs
Responsibility: Executes parsed commands.

What it does:
- Receives the command and arguments as a Vec<String> from the shell orchestrator (not from parser.rs directly).
- Reads commands.txt to look up command descriptions and validate commands.
- Uses system tools to spawn and run processes based on the provided command and arguments.
- Waits for processes to complete (synchronously).
- Handles errors like invalid commands or failed processes.
- Can be expanded later to handle things like background jobs or redirection.

Note:
- This module does not parse user input or know about parser.rs.
- It only executes commands passed to it in the correct format.
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads the commands.txt file and returns a HashMap of command -> description.
/// This allows the shell to look up what a command is supposed to do.
pub fn load_command_descriptions(path: &str) -> HashMap<String, String> {
    let mut commands = HashMap::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some((cmd, desc)) = line.split_once('|') {
                commands.insert(cmd.trim().to_string(), desc.trim().to_string());
            }
        }
    }
    commands
}

/// Checks if the command exists in the loaded command descriptions and executes it if valid.
/// Prints a helpful message if the command is not recognized.
pub fn execute_command(args: Vec<String>, command_descriptions: &HashMap<String, String>) {
    if args.is_empty() {
        return;
    }
    let command = &args[0];

    // Check if the command exists in the command descriptions
    if let Some(description) = command_descriptions.get(command) {
        // Optionally print what the command is supposed to do
        println!("{}: {}", command, description);

        // Call a separate function to actually run the process
        run_process(&args);
    } else {
        eprintln!("Unknown command: '{}'. Type 'help' for a list of commands.", command);
    }
}

/// Actually runs the process using std::process::Command.
/// This function assumes the command is valid.
fn run_process(args: &[String]) {
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    let output = std::process::Command::new(args_ref[0])
        .args(&args_ref[1..])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}