pub mod canvas;
pub mod native_gpu;
pub mod ui;
pub mod webgl;

// Re-export zana kuu ili zionekane nje
pub use canvas::CanvasDriver;
pub use crate::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum VectorShape {
    Rectangle { x: f32, y: f32, w: f32, h: f32, color: Color },
    Circle { cx: f32, cy: f32, radius: f32, color: Color },
    Line { x1: f32, y1: f32, x2: f32, y2: f32, stroke_width: f32, color: Color },
    Text { body: String, x: f32, y: f32, size: f32, color: Color },
}