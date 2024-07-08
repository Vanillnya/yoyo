use std::ops::RangeInclusive;

use egui::{
    load::{TextureLoadResult, TexturePoll},
    Color32, Context, Id, ImageOptions, ImageSource, Rect, Shape, TextureOptions, Ui, Vec2,
};
use egui_plot::{PlotBounds, PlotGeometry, PlotItem, PlotPoint, PlotTransform};

pub struct PlotSymbol<'a> {
    pub source: ImageSource<'a>,
    pub position: PlotPoint,
    pub size: Vec2,
    pub highlight: bool,
    pub name: String,
    pub id: Option<Id>,
}

impl<'a> PlotSymbol<'a> {
    pub fn load_for_size(&self, ctx: &Context, available_size: Vec2) -> TextureLoadResult {
        self.source
            .clone()
            .load(ctx, TextureOptions::default(), available_size.into())
    }
}

impl<'a> PlotItem for PlotSymbol<'a> {
    fn shapes(&self, ui: &Ui, transform: &PlotTransform, _shapes: &mut Vec<Shape>) {
        let image_screen_rect = {
            let left_top = PlotPoint::new(
                self.position.x - 0.5 * self.size.x as f64,
                self.position.y - 0.5 * self.size.y as f64,
            );
            let right_bottom = PlotPoint::new(
                self.position.x + 0.5 * self.size.x as f64,
                self.position.y + 0.5 * self.size.y as f64,
            );
            let left_top_screen = transform.position_from_point(&left_top);
            let right_bottom_screen = transform.position_from_point(&right_bottom);
            Rect::from_two_pos(left_top_screen, right_bottom_screen)
        };

        let tlr = self.load_for_size(ui.ctx(), image_screen_rect.size());

        match tlr {
            Ok(TexturePoll::Ready { texture }) => {
                egui::paint_texture_at(
                    ui.painter(),
                    image_screen_rect,
                    &ImageOptions::default(),
                    &texture,
                );
            }
            _ => {}
        }
    }

    fn initialize(&mut self, _x_range: RangeInclusive<f64>) {}

    fn name(&self) -> &str {
        &self.name
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

    fn allow_hover(&self) -> bool {
        false
    }

    fn geometry(&self) -> PlotGeometry<'_> {
        PlotGeometry::None
    }

    fn bounds(&self) -> PlotBounds {
        let mut bounds = PlotBounds::NOTHING;
        let left_top = PlotPoint::new(
            self.position.x as f32 - self.size.x / 2.0,
            self.position.y as f32 - self.size.y / 2.0,
        );
        let right_bottom = PlotPoint::new(
            self.position.x as f32 + self.size.x / 2.0,
            self.position.y as f32 + self.size.y / 2.0,
        );
        bounds.extend_with(&left_top);
        bounds.extend_with(&right_bottom);
        bounds
    }

    fn id(&self) -> Option<Id> {
        self.id
    }
}
