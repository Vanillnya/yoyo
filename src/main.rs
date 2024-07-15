use egui::{include_image, Context, Frame, RichText, Vec2, Visuals};
use egui_plot::{PlotPoint, Text};
use plot_symbol::PlotSymbol;

mod plot_symbol;
mod skin;
mod symbol;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("yoyo"),
        ..Default::default()
    };
    eframe::run_native(
        "yoyo",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Yoyo::new()))
        }),
    )
    .unwrap();
}

pub struct Yoyo {}

impl Yoyo {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for Yoyo {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                ui.style_mut().visuals = Visuals::light();
                egui_plot::Plot::new("yoyo_visualizer")
                    .show_axes(false)
                    .data_aspect(1.)
                    .allow_scroll(false)
                    .show_x(false)
                    .show_y(false)
                    .show(ui, |pui| {
                        pui.text(Text::new(PlotPoint::new(1.5, 1.5), RichText::new("clk").text_style(egui::TextStyle::Body)));
                        pui.add(PlotSymbol {
                            source: include_image!("../assets/skins/digital/wikipedia_inductiveload/AND_ANSI_Labelled.svg"),
                            position: PlotPoint::new(0.5, 0.5),
                            size: Vec2::new(1.2, 0.5),
                            highlight: false,
                            name: "Andgate".to_owned(),
                            id: None,
                        });
                    });
            });
    }
}
