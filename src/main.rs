#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simpleshell GUI",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)),
    )
}

#[derive(Default)]
struct MyApp {
    command: String,
    output: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simpleshell GUI");

            ui.horizontal(|ui| {
                ui.label("Command:");
                ui.text_edit_singleline(&mut self.command);
            });

            if ui.button("Run").clicked() {
                self.output = execute_command_gui(&self.command);
            }

            ui.separator();
            ui.label("Output:");
            ui.text_edit_multiline(&mut self.output);
        });
    }
}

fn execute_command_gui(command: &str) -> String {
    use std::process::Command;

    if command.is_empty() {
        return "No command entered.".to_string();
    }

    let parts: Vec<&str> = command.split_whitespace().collect();
    let cmd = parts[0];
    let args = &parts[1..];

    let output = Command::new(cmd).args(args).output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
        }
        Err(e) => format!("Failed to execute command: {}", e),
    }
}