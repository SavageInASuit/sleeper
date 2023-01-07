use eframe::egui;
use std::process::Command;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 100.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Catherine is Sleepy",
        options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct SleepyInstant(std::time::Instant);

impl Default for SleepyInstant {
    fn default() -> Self {
        Self(std::time::Instant::now())
    }
}

#[derive(Default)]
struct MyEguiApp {
    sleep_minutes: i32,
    sleep_pending: bool,
    sleep_started: SleepyInstant,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            sleep_minutes: 6,
            sleep_pending: false,
            sleep_started: SleepyInstant(std::time::Instant::now()),
        }
    }
}

fn sleep_counter(ui: &mut egui::Ui, counter: &mut i32) {
    ui.horizontal(|ui| {
        ui.label("Sleep Minutes");
        if ui.button("-").clicked() && *counter > 0 {
            *counter -= 5;
        }
        ui.label(format!("{}", counter));
        if ui.button("+").clicked() {
            *counter += 5;
        }
    });
}

fn sleep_after(minutes: &i32) -> JoinHandle<std::io::Result<std::process::Output>> {
    let minutes = *minutes;
    spawn(move || {
        sleep(Duration::from_secs((minutes * 60) as u64));
        println!("Sleeping now");
        let mut cmd = Command::new("osascript");
        cmd.arg("-e").arg("tell application \"Finder\" to sleep");
        cmd.output()
    })
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sleepy");
            ui.separator();
            if !self.sleep_pending {
                sleep_counter(ui, &mut self.sleep_minutes);

                ui.label(format!(
                    "Should we sleep after {} minutes?",
                    self.sleep_minutes
                ));

                if ui.button("Yes").clicked() {
                    println!("We will sleep in {} minutes", self.sleep_minutes);
                    // TODO: Use signals to
                    //  - Add ability to cancel
                    //  - Notify when sleeping - adds ability to reset UI
                    let handle = sleep_after(&self.sleep_minutes);
                    self.sleep_pending = true;
                    self.sleep_started = SleepyInstant(std::time::Instant::now());
                }
            } else {
                let elapsed_seconds = self.sleep_started.0.elapsed().as_secs() as i32;
                let seconds_until_sleep = (self.sleep_minutes * 60) - elapsed_seconds;
                let minutes_until_sleep: i32 = seconds_until_sleep / 60;
                ui.heading(format!(
                    "Sleeping in {} minutes {} seconds",
                    minutes_until_sleep,
                    seconds_until_sleep % 60
                ));
            }
        });
    }
}
