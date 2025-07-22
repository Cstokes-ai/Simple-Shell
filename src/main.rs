#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release

use eframe::egui;
mod executor;
use executor::execute_command;
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
            // Change the background color to black
            ctx.set_visuals(egui::Visuals::dark()); // Use a dark theme
            ui.heading("Simpleshell GUI");
            // add a background image to the app
            ui.add()

            // Single input line for command and output
            ui.horizontal(|ui| {
                ui.label("$:"); // PowerShell-style prompt
                let response = ui.text_edit_singleline(&mut self.command);
                if response.lost_focus() && response.ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.output = execute_command_gui(&self.command);
                    self.command.clear(); // Clear the command after execution
                }
            });

            ui.separator();
            ui.label("Output:");
            // have the output box be multiline and scrollable and cover the entire screen and not just a portion
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(f32::INFINITY)
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut self.output).desired_rows(10)); // Display output in a multiline box
                });
        });
    }
}

fn execute_command_gui(command: &str) -> String {
    if command.is_empty() {
        return "No command entered.".to_string();
    }

    // Use the executor module to handle command execution and return the output
    let args: Vec<String> = command.split_whitespace().map(String::from).collect();
    executor::execute_command(args) // Return the output from executor::execute_command
}