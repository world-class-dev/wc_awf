pub mod android;
pub mod desktop;
pub mod ios;
pub mod web;

use crate::render::VectorShape;

pub trait NativePlatform {
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn poll_events(&mut self);
    fn present_frame(&mut self, shapes: &[VectorShape]);
}
