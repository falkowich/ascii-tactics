use ascii_tactics::app::App;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ascii tactics",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
