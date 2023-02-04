mod gui;
mod physics;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // zmq messaging
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    requester
        .connect("tcp://localhost:5555")
        .expect("Couldn't connect socket.");

    // Physics thread
    let (tx, rx) = channel();
    thread::spawn(move || physics::simulation(tx, requester));

    // GUI thread
    eframe::run_native(
        "eframe template",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(gui::GraphView::new(cc, Some(rx)))),
    );
}
