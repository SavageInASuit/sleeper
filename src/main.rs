use eframe::egui;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 120.0)),
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
    killswitch: Option<Sender<bool>>,
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
            killswitch: None,
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

fn sleep_at(instant: Instant, kill_signal: Receiver<bool>) -> JoinHandle<()> {
    spawn(move || {
        let now = Instant::now();
        let mut until_sleep = instant.duration_since(now);
        while until_sleep.as_secs() > 0 {
            sleep(Duration::from_secs(1));
            let now = Instant::now();
            until_sleep = instant.duration_since(now);
            println!("Time until sleep: {:?}", until_sleep);
            if kill_signal.try_recv().is_ok() {
                println!("Kill signal received");
                return;
            }
        }
        println!("Sleeping now");
        let mut cmd = Command::new("osascript");
        cmd.arg("-e").arg("tell application \"Finder\" to sleep");
        println!(
            "TODO: Send 'has slept' signal - Command output: {:?}",
            cmd.output()
        );
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
                    //  - Notify when sleeping - adds ability to reset UI
                    let (tx, rx) = channel::<bool>();
                    self.killswitch = Some(tx);
                    self.sleep_started = SleepyInstant(Instant::now());
                    let time_to_sleep = self.sleep_started.0
                        + Duration::from_secs((self.sleep_minutes * 60) as u64);
                    let handle = sleep_at(time_to_sleep, rx);
                    self.sleep_pending = true;
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
                if ui.button("Cancel Sleep").clicked() {
                    match self.killswitch.as_mut().unwrap().send(true) {
                        Ok(_) => println!("Send kill signal"),
                        Err(e) => panic!("An error occurred while sending kill signal: {}", e),
                    }
                    self.sleep_pending = false;
                }
            }
        });
    }
}
