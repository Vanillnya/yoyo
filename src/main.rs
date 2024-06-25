use eframe::Frame;
use egui::Context;
use gate::{Gate, GateType};

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
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("yoyo");
            ui.add(Gate { ty: GateType::And });
        });
    }
}
