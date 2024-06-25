use egui::{Color32, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Widget};

pub struct Gate {
    pub ty: GateType,
}

pub enum GateType {
    And,
}

impl Widget for Gate {
    fn ui(self, ui: &mut Ui) -> Response {
        let rect = Rect::from_points(&[Pos2::ZERO, Pos2::new(100., 80.)]);

        let response = ui.allocate_rect(rect, Sense::click());

        match self.ty {
            GateType::And => {
                ui.painter()
                    .rect_stroke(rect, Rounding::same(15.), Stroke::new(2., Color32::GRAY));
            }
        }

        response
    }
}
