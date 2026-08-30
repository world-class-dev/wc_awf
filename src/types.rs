#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub const GREEN: Color = Color { r: 0, g: 255, b: 102, a: 1.0 };
    pub const DARK_BG: Color = Color { r: 12, g: 12, b: 12, a: 1.0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 1.0 };

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum VectorShape {
    Rectangle {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
        border_radius: f32,
    },
    Circle {
        cx: f32,
        cy: f32,
        radius: f32,
        color: Color,
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke_width: f32,
        color: Color,
    },
    Text {
        body: String,
        x: f32,
        y: f32,
        size: f32,
        color: Color,
    },
    Image {
        id: String,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },
    CandleStick {
        x: f32,
        open: f32,
        high: f32,
        low: f32,
        close: f32,
        width: f32,
    },
}