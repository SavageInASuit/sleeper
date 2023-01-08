use eframe::egui::{self};
use std::sync::mpsc::{channel, Sender};
use std::time::{Duration, Instant};

use crate::app::sleep::{sleep_at, SleepyInstant};

#[derive(Default)]
pub(crate) struct MyEguiApp {
    sleep_minutes: i32,
    sleep_minutes_s: String,
    sleep_pending: bool,
    sleep_started: SleepyInstant,
    killswitch: Option<Sender<bool>>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            sleep_minutes: 6,
            sleep_minutes_s: "6".to_owned(),
            sleep_pending: false,
            sleep_started: SleepyInstant(std::time::Instant::now()),
            killswitch: None,
        }
    }

    fn pre_sleep_ui(&mut self, ui: &mut egui::Ui) {
        self.sleep_counter(ui);
        ui.label(format!(
            "Should we sleep after {} minutes?",
            self.sleep_minutes
        ));
        if ui.button("Yes").clicked() {
            println!("We will sleep in {} minutes", self.sleep_minutes);
            let (tx, rx) = channel::<bool>();
            self.killswitch = Some(tx);
            self.sleep_started = SleepyInstant(Instant::now());
            let time_to_sleep =
                self.sleep_started.0 + Duration::from_secs((self.sleep_minutes * 60) as u64);
            sleep_at(time_to_sleep, rx);
            self.sleep_pending = true;
        }
    }

    fn sleep_pending_ui(&mut self, ui: &mut egui::Ui) {
        let elapsed_seconds = self.sleep_started.0.elapsed().as_secs() as i32;
        let seconds_until_sleep = (self.sleep_minutes * 60) - elapsed_seconds;
        let minutes_until_sleep: i32 = seconds_until_sleep / 60;
        ui.heading(format!(
            "Sleeping in {} minutes {} seconds",
            minutes_until_sleep,
            seconds_until_sleep % 60
        ));
        // The countdown isn't updated in UI unless there is an event triggered, so we need to manually repaint to see the countdown
        // This should probably only happen if the window is visible... Can we check for that?
        ui.ctx().request_repaint();

        ui.separator();

        if ui.button("Cancel Sleep").clicked() {
            match self.killswitch.as_mut().unwrap().send(true) {
                Ok(_) => println!("Send kill signal"),
                Err(e) => panic!("An error occurred while sending kill signal: {}", e),
            }
            self.sleep_pending = false;
        }
        let time_to_sleep =
            self.sleep_started.0 + Duration::from_secs((self.sleep_minutes * 60) as u64);
        if Instant::now() > time_to_sleep {
            self.sleep_pending = false;
        }
    }

    fn sleep_counter(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Delay Minutes");
            if ui.button("-").clicked() && self.sleep_minutes > 0 {
                self.sleep_minutes -= 5;
                self.sleep_minutes_s = self.sleep_minutes.to_string();
            }
            let minute_input = ui.text_edit_singleline(&mut self.sleep_minutes_s);
            let mut style = (*minute_input.ctx.style()).clone();
            style.spacing.text_edit_width = 20.;
            minute_input.ctx.set_style(style);
            if minute_input.changed() {
                self.sleep_minutes = match self.sleep_minutes_s.parse() {
                    Ok(num) => num,
                    _ => 0,
                }
            }
            if ui.button("+").clicked() {
                self.sleep_minutes += 5;
                self.sleep_minutes_s = self.sleep_minutes.to_string();
            }
        });
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sleepy");
            ui.separator();
            if !self.sleep_pending {
                self.pre_sleep_ui(ui);
            } else {
                self.sleep_pending_ui(ui);
            }
        });
    }
}
