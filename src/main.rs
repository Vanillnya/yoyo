use circut::Circut;
use egui::{include_image, Context, Frame, RichText, Vec2, Visuals};
use egui_plot::{PlotPoint, Text};
use layout::CircutLayout;
use plot_symbol::PlotSymbol;
use skin::Skin;
use symbol::SymbolKind;
use tracing_subscriber::fmt::time;

mod circut;
mod layout;
mod plot_symbol;
mod skin;
mod symbol;

fn main() {
    tracing_subscriber::fmt()
        .with_timer(time::LocalTime::rfc_3339())
        .init();

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

pub struct Yoyo {
    skin: Skin,
    circut: Circut,
    layout: CircutLayout,
}

impl Yoyo {
    pub fn new() -> Self {
        let skin = Skin::new();

        let circut = layout::constructs::sr_latch();
        let layout = CircutLayout::layout(&circut);

        Self {
            skin,
            circut,
            layout,
        }
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
                        for node in &self.circut.nodes {
                            let position = self.layout.positions[node.0];
                            let position = PlotPoint::new(position.0, position.1);

                            match node.1.symbol {
                                SymbolKind::Input => {
                                    pui.text(Text::new(
                                        position,
                                        RichText::new("in").text_style(egui::TextStyle::Body),
                                    ));
                                }
                                SymbolKind::Output => {
                                    pui.text(Text::new(
                                        position,
                                        RichText::new("out").text_style(egui::TextStyle::Body),
                                    ));
                                }
                                SymbolKind::And => {
                                    pui.add(PlotSymbol {
                                        source: self.skin.and.clone(),
                                        position,
                                        size: Vec2::new(1.2, 0.5),
                                        highlight: false,
                                        name: format!("and{:?}", node.0),
                                        id: None,
                                    });
                                }
                                SymbolKind::Nor => {
                                    pui.add(PlotSymbol {
                                        source: self.skin.nor.clone(),
                                        position,
                                        size: Vec2::new(1.2, 0.5),
                                        highlight: false,
                                        name: format!("and{:?}", node.0),
                                        id: None,
                                    });
                                }
                            }
                        }
                    });
            });
    }
}
