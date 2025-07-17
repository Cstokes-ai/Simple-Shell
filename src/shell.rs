use crate::parser;
use crate::executor;
use crate::utils;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use mime_guess;

pub fn shell() {
    let mut rl = DefaultEditor::new().unwrap();
    println!("Welcome to Simpleshell!");
    println!("Type 'exit' to quit.");
    loop {
        let readline = rl.readline("simpleshell> ");
        match readline {
            Ok(line) => {
                let args = parser::parse_input(&line);
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
                    "dir" => {
                        if let Ok(entries) = std::fs::read_dir(".") {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    println!("{}", entry.file_name().to_string_lossy());
                                }
                            }
                        } else {
                            eprintln!("Failed to list directory contents");
                        }
                    }
                    "copy" => {
                        if args.len() > 2 {
                            let src = &args[1];
                            let dest = &args[2];
                            if let Err(e) = std::fs::copy(src, dest) {
                                eprintln!("copy: {}", e);
                            }
                        } else {
                            eprintln!("copy: missing source or destination");
                        }
                    }
                    "del" => {
                        if args.len() > 1 {
                            let path = &args[1];
                            if let Err(e) = std::fs::remove_file(path) {
                                eprintln!("del: {}", e);
                            }
                        } else {
                            eprintln!("del: missing file path");
                        }
                    }
                    "type" => {
                        if args.len() > 1 {
                            let path = &args[1];
                            if let Ok(content) = std::fs::read_to_string(path) {
                                println!("{}", content);
                                let filetype = mime_guess::from_path(path)
                                    .first()
                                    .map(|m| m.essence_str().to_string()) // Convert to an owned String
                                    .unwrap_or_else(|| "application/octet-stream".to_string()); // Provide a default value as String
                                println!("File type: {}", filetype);
                            } else {
                                eprintln!("type: failed to read file '{}'", path);
                            }
                        } else {
                            eprintln!("type: missing file path");
                        }
                    }
                    "cls" => utils::clear_screen(),
                    "pause" => {
                        println!("Press Enter to continue...");
                        let _ = rl.readline("");
                    }
                    "help" => {
                        println!("Available commands:");
                        println!("cd <path> - Change directory");
                        println!("clear - Clear the screen");
                        println!("echo <text> - Print text to the console");
                        println!("dir - List directory contents");
                        println!("copy <source> <destination> - Copy a file");
                        println!("del <file> - Delete a file");
                        println!("type <file> - Display file content and type");
                        println!("cls - Clear the screen (Windows only)");
                        println!("pause - Pause execution until Enter is pressed");
                        println!("exit - Exit the shell");
                    }
                    _ => {
                        executor::execute_command(args);
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting shell...");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}