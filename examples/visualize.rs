use eframe::{
    egui::{
        self,
        plot::{Line, Plot},
        ScrollArea,
    },
    epaint::Color32,
    run_native, CreationContext, NativeOptions,
};
use symbolica::symbols::OpArgument;

const RES: usize = 100;

#[derive(Default)]
struct PlotInfo {
    expr: String,
    op_tree: Option<OpArgument>,
}

impl PlotInfo {
    fn parse_plots(&mut self) {
        // TODO: Parse expr to set op_tree
        todo!();
    }

    fn parametrized(&self, x: f64) -> f64 {
        // TODO: Implement auto-parametrization
        // Perhaps require \(x\) and \(y\) as coordinates.
        todo!();
    }
}

#[derive(Default)]
struct App {
    plots: Vec<PlotInfo>,
    updated_plots: bool,
}

impl App {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            ..Default::default()
        }
    }

    fn parse_plots(&mut self) {
        self.plots.iter_mut().for_each(PlotInfo::parse_plots);
        self.updated_plots = false;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("Equation List").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for plot_id in 0..self.plots.len() {
                    let resp = ui.text_edit_singleline(&mut self.plots[plot_id].expr);
                    if resp.changed() {
                        self.updated_plots = true;
                    }

                    if resp.dragged() {
                        if resp.drag_delta().y >= 1.0 && plot_id + 1 < self.plots.len() {
                            self.plots.swap(plot_id, plot_id + 1);
                        }
                    }
                }

                if self.updated_plots {
                    self.parse_plots();
                }

                let resp = ui.button("+");
                if resp.clicked() {
                    self.plots.push(PlotInfo::default());
                } else if resp.double_clicked() {
                    self.plots.pop();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("plot").show(ui, |plot_ui| {
                let bounds = plot_ui.plot_bounds();
                let span = bounds.min()[0]..bounds.max()[0];
                for plot in self.plots.iter() {
                    let mut plot_points = vec![[0.; 2]; RES];
                    for i in 0..RES {
                        let x = (i as f64) / (RES as f64 - 1.0) * (span.end - span.start) + span.start;
                        plot_points[i] = [x, plot.parametrized(x)];
                    }
                    plot_ui.line(Line::new(plot_points));
                }
            });
        });

        if ctx.input(|i| i.key_released(egui::Key::Q)) {
            frame.close();
        }
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        Color32::from_black_alpha(58).to_normalized_gamma_f32()
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native("Visualizer", options, Box::new(|cc| Box::new(App::new(cc))))
        .expect("Oh no, it crashed...");
}
