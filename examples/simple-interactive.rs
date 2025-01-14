//! Simple plot example derived from
//! [eframe](https://docs.rs/eframe/0.22.0/eframe/index.html#usage-native) and
//! [plotters](https://docs.rs/plotters/0.3.4/plotters/index.html#quick-start)

use eframe::egui::{self, CentralPanel, Visuals};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simple Interactive Example",
        native_options,
        Box::new(|cc| Ok(Box::new(SimpleInteractive::new(cc)))),
    )
    .unwrap();
}

struct SimpleInteractive {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl SimpleInteractive {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());

        Self {
            x_min: -1.0,
            x_max: 1.0,
            y_min: -0.1,
            y_max: 1.0,
        }
    }
}

impl eframe::App for SimpleInteractive {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let (zoom_delta, delta, is_lmb_down) = ui.input(|input| {
                let delta = input.pointer.delta();
                let is_lmb_down = input.pointer.primary_down();
                let zoom_delta = input.zoom_delta();

                (zoom_delta, delta, is_lmb_down)
            });

            if is_lmb_down {
                let (dx, dy) = (delta.x, delta.y);

                let x_range = self.x_max - self.x_min;
                let y_range = self.y_max - self.y_min;

                self.x_min -= x_range * dx / 1000.0;
                self.x_max -= x_range * dx / 1000.0;
                self.y_min += y_range * dy / 1000.0;
                self.y_max += y_range * dy / 1000.0;
            }

            if zoom_delta != 1.0 {
                let x_center = (self.x_min + self.x_max) / 2.0;
                let y_center = (self.y_min + self.y_max) / 2.0;

                let x_range = (self.x_max - self.x_min) / 2.0;
                let y_range = (self.y_max - self.y_min) / 2.0;

                let x_range = x_range / zoom_delta;
                let y_range = y_range / zoom_delta;

                self.x_min = x_center - x_range;
                self.x_max = x_center + x_range;
                self.y_min = y_center - y_range;
                self.y_max = y_center + y_range;
            }

            let root = EguiBackend::new(ui).into_drawing_area();
            root.fill(&WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .caption("y=x^2", ("sans-serif", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(self.x_min..self.x_max, self.y_min..self.y_max)
                .unwrap();

            chart.configure_mesh().draw().unwrap();

            chart
                .draw_series(LineSeries::new(
                    (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                    &RED,
                ))
                .unwrap()
                .label("y = x^2")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

            chart
                .configure_series_labels()
                .background_style(WHITE.mix(0.8))
                .border_style(BLACK)
                .draw()
                .unwrap();

            root.present().unwrap();
        });
    }
}
