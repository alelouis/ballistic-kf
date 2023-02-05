use egui::plot::{Legend, Line, LineStyle, Plot, PlotPoints, Points};
use egui::*;

use std::sync::mpsc::Receiver;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GraphView {
    label: String,
    initialized: bool,
    data: Vec<[f64; 5]>,
    #[serde(skip)]
    rx: Option<Receiver<[f64; 5]>>,
}

impl GraphView {
    pub fn new(__cc: &eframe::CreationContext<'_>, rx: Option<Receiver<[f64; 5]>>) -> Self {
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

    fn get_data(&mut self) -> (Vec<f64>, Vec<Vec<[f64; 2]>>) {
        self.push_data();
        let time: Vec<f64> = self.data.iter().map(|x| x[0]).collect();
        let measure_data: Vec<[f64; 2]> = self.data.iter().map(|x| [x[1], x[2]]).collect();
        let kf_data: Vec<[f64; 2]> = self.data.iter().map(|x| [x[3], x[4]]).collect();
        (time, vec![measure_data, kf_data])
    }
}

impl eframe::App for GraphView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();
            let mut plot = Plot::new("lines_demo").legend(Legend::default());
            let (time, data) = self.get_data();

            let measure_points: PlotPoints = PlotPoints::from(data[0].clone());
            let kf_points: PlotPoints = PlotPoints::from(data[1].clone());

            let mut max_x = 0.0;
            let mut max_y = 0.0;
            for point in measure_points.points() {
                if point.x > max_x {
                    max_x = point.x
                }
                if point.y > max_y {
                    max_y = point.y
                }
            }
            let measure_line = Points::new(measure_points)
                .color(Color32::from_rgb(200, 100, 100))
                .radius(2.0)
                .name("Measurements");

            let kf_line = Line::new(kf_points)
                .color(Color32::from_rgb(100, 100, 200))
                .style(LineStyle::Solid)
                .width(3.0)
                .name("Kalman Filtered");

            plot = plot.include_y(max_y);
            plot = plot.include_y(0.0);
            plot = plot.include_x(max_x);
            plot = plot.include_x(0.0);

            plot.show(ui, |plot_ui| {
                plot_ui.points(measure_line);
                plot_ui.line(kf_line)
            });
        });
    }
}
