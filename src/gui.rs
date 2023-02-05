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
            label: "Ballistic Kalman Demo".to_owned(),
            initialized: false,
            data: vec![],
            rx,
        }
    }
    /// Fetches data from simulation thread through rx channel
    fn fetch_data(&mut self) {
        if self.rx.is_some() {
            self.rx
                .as_ref()
                .expect("hmm, should have gotten something right here.")
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

/// Wrap a given observable with time vector
fn wrap_with_time(time: &Vec<f64>, vec: &Vec<f64>) -> Vec<[f64; 2]> {
    vec.iter()
        .zip(time)
        .map(|(v, t)| [t.clone(), v.clone()])
        .collect()
}

/// Finds x/y limits of a given points set
fn find_max(points: &PlotPoints) -> (f64, f64) {
    let mut max_x = 0.0;
    let mut max_y = 0.0;
    for p in points.points() {
        if p.x > max_x {
            max_x = p.x
        }
        if p.y > max_y {
            max_y = p.y
        }
    }
    (max_x, max_y)
}

impl eframe::App for GraphView {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // Get data from simulation
        self.fetch_data();

        // Get time back
        let time: Vec<f64> = self.data.iter().map(|x| x[0]).collect();

        // Extract y from measurements, Kalman filter and true position.
        let m_y: Vec<f64> = self.data.iter().map(|x| x[2]).collect();
        let kf_y: Vec<f64> = self.data.iter().map(|x| x[4]).collect();
        let true_y: Vec<f64> = self.data.iter().map(|x| x[6]).collect();

        // Extract x position from measurements, Kalman filter and true position.
        let m_x: Vec<f64> = self.data.iter().map(|x| x[1]).collect();
        let kf_x: Vec<f64> = self.data.iter().map(|x| x[3]).collect();
        let true_x: Vec<f64> = self.data.iter().map(|x| x[5]).collect();

        // Wrap observables with time.
        let m_time_y: Vec<[f64; 2]> = wrap_with_time(&time, &m_y);
        let m_time_x: Vec<[f64; 2]> = wrap_with_time(&time, &m_x);
        let kf_time_y: Vec<[f64; 2]> = wrap_with_time(&time, &kf_y);
        let kf_time_x: Vec<[f64; 2]> = wrap_with_time(&time, &kf_x);
        let true_time_y: Vec<[f64; 2]> = wrap_with_time(&time, &true_y);
        let true_time_x: Vec<[f64; 2]> = wrap_with_time(&time, &true_x);

        TopBottomPanel::top("x_panel")
            .exact_height(frame.info().window_info.size[1] / 2.0)
            .show(ctx, |ui| {
                // Declare plot
                let mut plot = Plot::new("x_plot").legend(Legend::default());

                // Convert x position data to PlotPoints
                let m_x_points: PlotPoints = PlotPoints::from(m_time_x.clone());
                let kf_x_points: PlotPoints = PlotPoints::from(kf_time_x.clone());
                let true_x_points: PlotPoints = PlotPoints::from(true_time_x.clone());

                // Get x/y extent
                let (max_x, max_y) = find_max(&kf_x_points);

                // Define plot elements
                let measure_line = Points::new(m_x_points)
                    .color(Color32::from_rgb(100, 100, 200))
                    .radius(2.0)
                    .name("Measurements");

                let kf_line = Line::new(kf_x_points)
                    .color(Color32::from_rgb(100, 200, 100))
                    .style(LineStyle::Solid)
                    .width(3.0)
                    .name("Kalman Filtered");

                let true_line = Line::new(true_x_points)
                    .color(Color32::from_rgb(200, 100, 100))
                    .style(LineStyle::Solid)
                    .width(3.0)
                    .name("True");

                // Set bounds
                plot = plot.include_y(max_y);
                plot = plot.include_y(0.0);
                plot = plot.include_x(max_x);
                plot = plot.include_x(0.0);

                // Set title
                ui.label("X position (m)");

                // Plot lines
                plot.show(ui, |plot_ui| {
                    plot_ui.points(measure_line);
                    plot_ui.line(kf_line);
                    plot_ui.line(true_line);
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            // Update frame
            ui.ctx().request_repaint();

            // Declare plot
            let mut plot = Plot::new("y_plot").legend(Legend::default());

            // Convert x position data to PlotPoints
            let m_y_points: PlotPoints = PlotPoints::from(m_time_y.clone());
            let kf_y_points: PlotPoints = PlotPoints::from(kf_time_y.clone());
            let true_y_points: PlotPoints = PlotPoints::from(true_time_y.clone());

            // Get x/y extent
            let (max_x, max_y) = find_max(&kf_y_points);

            // Define plot elements
            let measure_line = Points::new(m_y_points)
                .color(Color32::from_rgb(100, 100, 200))
                .radius(2.0)
                .name("Measurements");

            let kf_line = Line::new(kf_y_points)
                .color(Color32::from_rgb(100, 200, 100))
                .style(LineStyle::Solid)
                .width(3.0)
                .name("Kalman Filtered");

            let true_line = Line::new(true_y_points)
                .color(Color32::from_rgb(200, 100, 100))
                .style(LineStyle::Solid)
                .width(3.0)
                .name("True");

            // Set bounds
            plot = plot.include_y(max_y);
            plot = plot.include_y(0.0);
            plot = plot.include_x(max_x);
            plot = plot.include_x(0.0);

            // Set title
            ui.label("Y position (m)");

            // Plot lines
            plot.show(ui, |plot_ui| {
                plot_ui.points(measure_line);
                plot_ui.line(kf_line);
                plot_ui.line(true_line);
            });
        });
    }
}
