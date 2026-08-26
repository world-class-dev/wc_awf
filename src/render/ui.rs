use crate::render::VectorShape;
use crate::Color;

pub struct UiEngine {
    pub width: u32,
    pub height: u32,
    pub render_queue: Vec<VectorShape>,
}

impl UiEngine {
    pub fn new(width: u32, height: u32) -> Self {
        UiEngine {
            width,
            height,
            render_queue: Vec::with_capacity(512),
        }
    }

    pub fn clear(&mut self) {
        self.render_queue.clear();
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.render_queue.push(VectorShape::Rectangle { x, y, w, h, color });
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, radius: f32, color: Color) {
        self.render_queue.push(VectorShape::Circle { cx, cy, radius, color });
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, stroke_width: f32, color: Color) {
        self.render_queue.push(VectorShape::Line { x1, y1, x2, y2, stroke_width, color });
    }

    pub fn draw_text(&mut self, body: &str, x: f32, y: f32, size: f32, color: Color) {
        self.render_queue.push(VectorShape::Text {
            body: body.to_string(),
            x,
            y,
            size,
            color,
        });
    }
}
