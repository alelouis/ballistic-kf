mod gui;
mod kalman;
mod physics;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // Physics thread
    let (tx, rx) = channel();
    thread::spawn(move || physics::run_simulation(tx));

    // GUI thread
    eframe::run_native(
        "Ballistic Kalman",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(gui::GraphView::new(cc, Some(rx)))),
    );
}
