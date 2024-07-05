use std::fmt::Display;

use egui::{Color32, Ui};
use egui_plot::{PlotBounds, PlotGeometry, PlotItem, PlotPoint};

pub struct Symbol {
    pub kind: SymbolKind,
    pub position: PlotPoint,
    pub highlight: bool,
}

pub enum SymbolKind {
    And,
}

impl SymbolKind {
    pub fn name(&self) -> &str {
        match self {
            SymbolKind::And => "And",
        }
    }
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PlotItem for Symbol {
    fn shapes(&self, ui: &Ui, transform: &egui_plot::PlotTransform, shapes: &mut Vec<egui::Shape>) {
    }

    fn initialize(&mut self, x_range: std::ops::RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        self.kind.name()
    }

    fn color(&self) -> Color32 {
        Color32::TRANSPARENT
    }

    fn highlight(&mut self) {
        self.highlight = true;
    }

    fn highlighted(&self) -> bool {
        self.highlight
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        let left_top = PlotPoint::new(self.position.x as f32 - 10., self.position.y as f32 - 5.);
        let right_bottom =
            PlotPoint::new(self.position.x as f32 + 10., self.position.y as f32 + 5.);
        bounds.extend_with(&left_top);
        bounds.extend_with(&right_bottom);
        bounds
    }

    fn id(&self) -> Option<egui::Id> {
        None
    }
}
