use crate::platform::NativePlatform;
use crate::render::VectorShape;

pub struct IosHost;

impl IosHost {
    pub fn new() -> Self { IosHost }
}

impl NativePlatform for IosHost {
    fn initialize(&mut self) -> Result<(), &'static str> {
        // Bind CAMetalLayer with UIKit
        Ok(())
    }

    fn poll_events(&mut self) {
        // iOS Touch Events
    }

    fn present_frame(&mut self, _shapes: &[VectorShape]) {
        // Metal Pipeline submission
    }
}
