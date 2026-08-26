use crate::platform::NativePlatform;
use crate::render::VectorShape;

pub struct WebHost;

impl WebHost {
    pub fn new() -> Self { WebHost }
}

impl NativePlatform for WebHost {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn poll_events(&mut self) {
        // Browser Window Events
    }

    fn present_frame(&mut self, _shapes: &[VectorShape]) {
        // WebGL / Canvas Flush Call
    }
}
