mod app;
mod types;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Task Manager",
        options,
        Box::new(|_cc| Box::<app::TaskManagerApp>::default()),
    )
}