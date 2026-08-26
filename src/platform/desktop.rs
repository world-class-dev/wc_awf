use crate::platform::NativePlatform;
use crate::render::VectorShape;

pub struct DesktopHost;

impl DesktopHost {
    pub fn new() -> Self { DesktopHost }
}

impl NativePlatform for DesktopHost {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn poll_events(&mut self) {
        // X11 / Wayland / Win32 Message Loop
    }

    fn present_frame(&mut self, _shapes: &[VectorShape]) {
        // Desktop Native Window Refresh
    }
}
