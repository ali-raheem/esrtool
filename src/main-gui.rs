use eframe::{egui};
use std::path::PathBuf;

struct ESRToolApp {
    game_path: Option<PathBuf>,
    message: String,
}

impl Default for ESRToolApp {
    fn default() -> Self {
        Self {
            game_path: None,
            message: String::new(),
        }
    }
}

impl eframe::App for ESRToolApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ESRtool GUI");

            if ui.button("Choose File").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.game_path = Some(path);
                }
            }

            if let Some(path) = &self.game_path {
                ui.label(format!("Selected file: {}", path.display()));
            } else {
                ui.label("No file selected.");
            }

            if ui.button("Apply Patch").clicked() {
                if let Some(path) = &self.game_path.take() {
                    self.apply_patch(path.to_str().unwrap());
                } else {
                    self.message = "No file selected.".to_string();
                }
            }

            if ui.button("Remove Patch").clicked() {
                if let Some(path) = &self.game_path.take() {
                    self.remove_patch(path.to_str().unwrap());
                } else {
                    self.message = "No file selected.".to_string();
                }
            }

            if !self.message.is_empty() {
                ui.label(&self.message);
            }
        });
    }
}

impl ESRToolApp {
    fn apply_patch(&mut self, path: &str) {
        match esrtool::Iso::new(path) {
                Ok(mut game) => {
                    match game.patch() {
                        Ok(_) => {
                            game.write().expect("Couldn't write out file");
                            self.message = "Patch applied successfully.".to_string();
                        }
                        Err(e) => {
                            self.message = format!("Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    self.message = format!("Failed to open file: {}", e);
                }
            }
    }



    fn remove_patch(&mut self, path: &str) {
        match esrtool::Iso::new(path) {
            Ok(mut game) => {
                match game.unpatch() {
                    Ok(_) => {
                        game.write().expect("Couldn't write out file");
                        self.message = "Patch removed successfully.".to_string();
                    }
                    Err(e) => {
                        self.message = format!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                self.message = format!("Failed to open file: {}", e);
            }
        }
    }
}


fn main() {
    let _app = ESRToolApp::default();
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "ESRTool GUI",
        native_options,
        Box::new(|_cc| Ok(Box::new(ESRToolApp::default())))
    );
}