mod app;
use app::gui::MyEguiApp;
use eframe::egui::ViewportBuilder;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([220.0, 100.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Sleeper",
        options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}
