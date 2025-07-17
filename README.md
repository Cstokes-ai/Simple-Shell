# SimpleshellRust

## Overview
SimpleshellRust is a Rust-based shell application with a graphical user interface (GUI) built using the `egui` and `eframe` libraries. It allows users to execute shell commands and view their output in a user-friendly GUI.

## Requirements
To build and run this project, ensure you have the following installed:

### Rust Toolchain
- **Rust Nightly Toolchain** (required for `edition2024` features used by dependencies like `egui` and `eframe`):
  ```bash
  rustup install nightly
  rustup default nightly
  ```

### Additional Tools
- **Visual Studio Build Tools** (for the MSVC toolchain):
  - Install the "Desktop development with C++" workload using the [Visual Studio Installer](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

### Dependencies
The following dependencies are required and specified in `Cargo.toml`:

```toml
[dependencies]
rustyline = "16.0.0"       # For command-line input handling
mime_guess = "2.0.0"      # For guessing MIME types of files
winres = "0.1.12"         # For embedding resources like icons
egui = "0.32.0"           # GUI framework
eframe = "0.32.0"         # Framework for building egui apps
```

## Setup Instructions

1. **Clone the Repository**
   ```bash
   git clone <repository-url>
   cd SimpleshellRust
   ```

2. **Install Rust Nightly Toolchain**
   ```bash
   rustup install nightly
   rustup default nightly
   ```

3. **Ensure Visual Studio Build Tools Are Installed**
   - Open the Visual Studio Installer and install the "Desktop development with C++" workload.

4. **Build the Project**
   - For a debug build:
     ```bash
     cargo build
     ```
   - For a release build:
     ```bash
     cargo build --release
     ```

5. **Run the Application**
   - For debug mode:
     ```bash
     cargo run
     ```
   - For release mode, navigate to the `target/release` directory and run:
     ```bash
     ./SimpleshellRust.exe
     ```

## Features
- Execute shell commands and view their output in a GUI.
- User-friendly interface with input fields and buttons.
- Displays both `stdout` and `stderr` of executed commands.
- Customizable application icon (via `winres` crate).

## Notes
- Ensure the `icon.ico` file is placed in the project root for the `winres` crate to embed it into the executable.
- If you encounter issues with dependencies requiring `edition2024`, ensure you are using the nightly Rust toolchain.

## Troubleshooting
- **`edition2024` Error:**
  - Switch to the nightly toolchain:
    ```bash
    rustup install nightly
    rustup default nightly
    ```

- **`dlltool` Error:**
  - Ensure MinGW is not in your PATH. Use the MSVC toolchain instead.
  - Clean the project and rebuild:
    ```bash
    cargo clean
    cargo build
    ```

## License
This project is licensed under the MIT License.
