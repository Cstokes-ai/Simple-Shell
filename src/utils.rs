/*
Responsibility: Contains helper functions that don’t belong in parser or executor.
What it might include:

- Error formatting or output styling.
- Screen clearing, help messages, or logging.
- Any small functionality that supports the main modules but doesn’t logically belong in them.
*/

// Clears the terminal screen (works on Windows and Unix)
pub fn clear_screen() {
    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
    }
    #[cfg(not(windows))]
    {
        std::process::Command::new("clear")
            .status()
            .unwrap();
    }
}