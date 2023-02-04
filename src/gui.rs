use egui::plot::{Legend, Line, LineStyle, Plot, PlotPoints};
use egui::*;

use std::sync::mpsc::Receiver;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GraphView {
    label: String,
    initialized: bool,
    data: Vec<[f64; 2]>,
    #[serde(skip)]
    rx: Option<Receiver<[f64; 2]>>,
}

impl GraphView {
    pub fn new(__cc: &eframe::CreationContext<'_>, rx: Option<Receiver<[f64; 2]>>) -> Self {
        Self {
            label: "Demo GUI".to_owned(),
            initialized: false,
            data: vec![],
            rx,
        }
    }

    fn push_data(&mut self) {
        if self.rx.is_some() {
            self.rx
                .as_ref()
                .expect("hmm")
                .try_recv()
                .and_then(|data| {
                    Ok({
                        self.data.push(data);
                    })
                })
                .ok();
        }
    }

    fn get_data(&mut self) -> PlotPoints {
        self.push_data();
        let points: PlotPoints = PlotPoints::from(self.data.clone());
        points
    }
}

impl eframe::App for GraphView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();
            let mut plot = Plot::new("lines_demo").legend(Legend::default());
            let data = self.get_data();

            let mut max_x = 0.0;
            let mut max_y = 0.0;
            for point in data.points() {
                if point.x > max_x {
                    max_x = point.x
                }
                if point.y > max_y {
                    max_y = point.y
                }
            }
            let line = Line::new(data)
                .color(Color32::from_rgb(200, 100, 100))
                .style(LineStyle::Solid)
                .name("wave");
            plot = plot.include_y(max_y);
            plot = plot.include_y(0.0);
            plot = plot.include_x(max_x);
            plot = plot.include_x(0.0);

            plot.show(ui, |plot_ui| {
                plot_ui.line(line);
            });
        });
    }
}
