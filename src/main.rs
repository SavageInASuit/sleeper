mod app;
use app::gui::MyEguiApp;
use eframe::egui::{self};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 120.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "Catherine is Sleepy",
        options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}
