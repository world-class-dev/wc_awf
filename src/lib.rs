pub mod core;
pub mod platform;
pub mod render;
pub mod security;
pub mod utils;

pub use crate::core::engine::WcEngine;
pub use crate::render::VectorShape;

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
