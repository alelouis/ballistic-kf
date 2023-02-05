use egui::plot::{Legend, Line, LineStyle, Plot, PlotPoints, Points};
use egui::*;

use std::sync::mpsc::Receiver;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GraphView {
    label: String,
    initialized: bool,
    data: Vec<[f64; 7]>,
    #[serde(skip)]
    rx: Option<Receiver<[f64; 7]>>,
}

impl GraphView {
    pub fn new(__cc: &eframe::CreationContext<'_>, rx: Option<Receiver<[f64; 7]>>) -> Self {
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
}

fn wrap_with_time(time: &Vec<f64>, vec: &Vec<f64>) -> Vec<[f64; 2]> {
    vec.iter()
        .zip(time)
        .map(|(v, t)| [t.clone(), v.clone()])
        .collect()
}

impl eframe::App for GraphView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.ctx().request_repaint();
            let mut plot = Plot::new("lines_demo").legend(Legend::default());

            self.push_data();
            let time: Vec<f64> = self.data.iter().map(|x| x[0]).collect();
            let m_altitude: Vec<f64> = self.data.iter().map(|x| x[2]).collect();
            let kf_altitude: Vec<f64> = self.data.iter().map(|x| x[4]).collect();
            let true_altitude: Vec<f64> = self.data.iter().map(|x| x[6]).collect();

            let m_time_altitude: Vec<[f64; 2]> = wrap_with_time(&time, &m_altitude);
            let kf_time_altitude: Vec<[f64; 2]> = wrap_with_time(&time, &kf_altitude);
            let true_time_altitude: Vec<[f64; 2]> = wrap_with_time(&time, &true_altitude);

            let m_altitude_points: PlotPoints = PlotPoints::from(m_time_altitude.clone());
            let kf_altitude_points: PlotPoints = PlotPoints::from(kf_time_altitude.clone());
            let true_altitude_points: PlotPoints = PlotPoints::from(true_time_altitude.clone());

            let mut max_x = 0.0;
            let mut max_y = 0.0;
            for point in kf_altitude_points.points() {
                if point.x > max_x {
                    max_x = point.x
                }
                if point.y > max_y {
                    max_y = point.y
                }
            }
            let measure_line = Points::new(m_altitude_points)
                .color(Color32::from_rgb(200, 100, 100))
                .radius(2.0)
                .name("Measurements");

            let kf_line = Line::new(kf_altitude_points)
                .color(Color32::from_rgb(100, 200, 100))
                .style(LineStyle::Solid)
                .width(3.0)
                .name("Kalman Filtered");

            let true_line = Line::new(true_altitude_points)
                .color(Color32::from_rgb(100, 100, 200))
                .style(LineStyle::Solid)
                .width(3.0)
                .name("True");

            plot = plot.include_y(max_y);
            plot = plot.include_y(0.0);
            plot = plot.include_x(max_x);
            plot = plot.include_x(0.0);

            plot.show(ui, |plot_ui| {
                plot_ui.points(measure_line);
                plot_ui.line(kf_line);
                plot_ui.line(true_line);
            });
        });
    }
}
