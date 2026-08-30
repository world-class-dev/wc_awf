use crate::render::VectorShape;

pub struct NativeGpuPipeline;

impl NativeGpuPipeline {
    pub fn new() -> Self {
        NativeGpuPipeline
    }

    pub fn draw_shape(&self, shape: &VectorShape) {
        match shape {
            VectorShape::Rectangle {
                x: _,
                y: _,
                w: _,
                h: _,
                color: _,
                border_radius: _,
            } => {
                // TODO: Tekeleza GPU draw call ya Rectangle
            }
            VectorShape::Circle {
                cx: _,
                cy: _,
                radius: _,
                color: _,
            } => {
                // TODO: Tekeleza GPU draw call ya Circle
            }
            VectorShape::Line {
                x1: _,
                y1: _,
                x2: _,
                y2: _,
                stroke_width: _,
                color: _,
            } => {
                // TODO: Tekeleza GPU draw call ya Line
            }
            VectorShape::Text {
                body: _,
                x: _,
                y: _,
                size: _,
                color: _,
            } => {
                // TODO: Tekeleza GPU draw call ya Text / Glyph Atlas
            }
            VectorShape::Image { id: _, x: _, y: _, w: _, h: _ } => {
                // TODO: Tekeleza GPU draw call ya Texture/Image
            }
            VectorShape::CandleStick {
                x: _,
                open: _,
                high: _,
                low: _,
                close: _,
                width: _,
            } => {
                // TODO: Tekeleza GPU draw call ya Candlestick
            }
        }
    }

    pub fn submit_frame(&self, shapes: &[VectorShape]) {
        for shape in shapes {
            self.draw_shape(shape);
        }
    }
}