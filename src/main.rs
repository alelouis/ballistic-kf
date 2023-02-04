mod gui;
mod physics;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // Physics thread
    let (tx, rx) = channel();
    thread::spawn(move || physics::simulation(tx));

    // GUI thread
    eframe::run_native(
        "eframe template",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(gui::GraphView::new(cc, Some(rx)))),
    );
}
