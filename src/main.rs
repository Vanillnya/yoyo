use egui::{Context, Frame, RichText};
use egui_plot::{PlotPoint, Text};

mod gate;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("yoyo"),
        ..Default::default()
    };
    eframe::run_native("yoyo", options, Box::new(|_cc| Box::new(Yoyo::new()))).unwrap();
}

pub struct Yoyo {}

impl Yoyo {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for Yoyo {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                // ui.add(Gate { ty: GateType::And });
                egui_plot::Plot::new("yoyo_visualizer")
                    .show_axes(false)
                    .data_aspect(1.)
                    .allow_scroll(false)
                    .show_x(false)
                    .show_y(false)
                    .show(ui, |pui| {
                        pui.text(Text::new(PlotPoint::new(15, 13), RichText::new("clk")));
                    })
            });
    }
}
