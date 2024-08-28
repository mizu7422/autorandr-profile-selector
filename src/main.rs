use eframe::egui;
use std::usize;
use egui::Color32;
use egui::RichText;
use std::process::{Command, Stdio, exit};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions{
        ..Default::default()
    };
    eframe::run_native("Monitor profile selector", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp {
    profiles: Vec<String>,
    selected_profile: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            profiles: get_autorandr_detected_profiles(true),
            selected_profile: get_selected_profile_sorted(),
        }
    }
}

//autorandr --current is glitched, but the --detected always puts the current profile first
fn get_selected_profile_sorted() -> usize{
    let sorted_profiles = get_autorandr_detected_profiles(true);
    let current_profile = {
        let profiles = get_autorandr_detected_profiles(false);
        profiles[0].clone()
    };

    for (index, profile) in sorted_profiles.into_iter().enumerate(){
        if current_profile == profile {
            return index;
        }
    }
    0
}

fn get_autorandr_detected_profiles(ordered: bool) -> Vec<String> {
    let output = Command::new("autorandr")
        .args(["--detected"])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8(output.stdout).expect("Failed to convert output to string");
    let mut output_vec: Vec<String> = output_str.split_whitespace().map(|s| s.to_string()).collect();

    if ordered{
        output_vec.sort();
    }
    output_vec
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select your profile");
            ui.horizontal(|ui| {
                ui.label("Profiles: ");
                for (index, profile) in self.profiles.clone().iter().enumerate() {
                    if index == self.selected_profile{
                        ui.label(RichText::new(profile).color(Color32::RED));
                    }else{
                        ui.label(profile);
                    }
                }
                if self.profiles.is_empty(){
                    ui.label("this shit is empty");
                }
                if ui.input(|k| k.key_pressed(egui::Key::Tab)) {
                    if self.profiles.len() > self.selected_profile+1{
                        self.selected_profile += 1;
                    }else{
                        self.selected_profile = 0;
                    }
                }
                if ui.input(|k| k.key_pressed(egui::Key::Enter)) {
                    let _ = Command::new("autorandr").args(["--change", self.profiles[self.selected_profile].as_str()]).status();
                }
                if ui.input(|k| k.key_pressed(egui::Key::Escape)) {
                    exit(69)
                }
            });
        });
    }
}

