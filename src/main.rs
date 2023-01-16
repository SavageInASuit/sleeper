mod app;
use app::gui::MyEguiApp;
use eframe::egui::{self};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(220.0, 100.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Sleeper",
        options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}
