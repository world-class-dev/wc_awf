
pub use crate::render::ui::UiEngine;

use crate::core::memory::LinearMemoryBuffer;

pub struct WcEngine {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub ui: UiEngine,
    pub memory: LinearMemoryBuffer,
}

impl WcEngine {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        WcEngine {
            title: title.to_string(),
            width,
            height,
            ui: UiEngine::new(width, height),
            memory: LinearMemoryBuffer::new(1024 * 1024), // 1MB Arena
        }
    }

    pub fn boot(&mut self) -> String {
        crate::utils::panic::set_panic_hook();

        #[cfg(target_os = "android")]
        {
            let mut host = crate::platform::android::AndroidHost::new();
            let _ = crate::platform::NativePlatform::initialize(&mut host);
            format!("[wc_awf::core] Android NDK Surface Active: {}", self.title)
        }

        #[cfg(target_os = "ios")]
        {
            let mut host = crate::platform::ios::IosHost::new();
            let _ = crate::platform::NativePlatform::initialize(&mut host);
            format!("[wc_awf::core] iOS Metal Layer Active: {}", self.title)
        }

        #[cfg(target_arch = "wasm32")]
        {
            format!("[wc_awf::core] WASM Engine Active: {}", self.title)
        }

        #[cfg(all(
            not(target_arch = "wasm32"),
            not(target_os = "android"),
            not(target_os = "ios")
        ))]
        {
            format!("[wc_awf::core] Desktop Binary Active: {}", self.title)
        }
    }

    pub fn render_frame<F>(&mut self, mut build_fn: F) -> usize
    where
        F: FnMut(&mut UiEngine),
    {
        self.ui.clear();
        build_fn(&mut self.ui);
        self.ui.render_queue.len()
    }
}

// Alias of stop error E0432 on the `src/lib.rs` na `src/wasm.rs`
pub type Engine = WcEngine;