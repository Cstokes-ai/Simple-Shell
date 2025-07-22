/*
executor.rs
Responsibility: Executes parsed commands.

What it does:
- Receives the command and arguments as a Vec<String> from the shell orchestrator.
- Uses system tools to spawn and run processes.
- Waits for processes to complete (synchronously).
- Handles errors like invalid commands or failed processes.
- May expand later to handle things like background jobs or redirection.

Note:
- This module does not parse user input or know about parser.rs.
- It only executes commands passed to it in the correct format.
*/

pub fn execute_command(args: Vec<String>) -> String {
    if args.is_empty() {
        return "No command entered.".to_string();
    }
    let command = &args[0];
    let command_args = &args[1..];

    #[cfg(windows)]
    let mut cmd = {
        // Run Windows built-ins through cmd.exe
        let builtins = ["dir", "copy", "del", "type", "cls", "echo", "cd", "chdir", "pause", "help", "goto", "notepad", "mkdir", "move", "erase",
        "shift", "tree", "ver", "xcopy", "print", "attrib", "fc", "find", "findstr", "format", "label", "md", "rd", "ren", "set", "setlocal", "sort", "start", "tasklist", "taskkill", 
        "title", "timeout", "tree", "where", "whoami", "wmic", "xcopy", "assoc", "break", "call", "cd", "chcp", "cls", "color", "comp", "compact", "continue", "copy", "date", "del", "dir"];
        if builtins.contains(&command.as_str()) {
            let mut c = std::process::Command::new("cmd");
            c.args(["/C", command]);
            c.args(command_args);
            c
        } else {
            let mut c = std::process::Command::new(command);
            c.args(command_args);
            c
        }
    };

    #[cfg(not(windows))]
    let mut cmd = {
        let mut c = std::process::Command::new(command);
        c.args(command_args);
        c
    };

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if !stderr.is_empty() {
                format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
            } else {
                stdout
            }
        }
        Err(e) => format!("Failed to execute command '{}': {}", command, e),
    }
}