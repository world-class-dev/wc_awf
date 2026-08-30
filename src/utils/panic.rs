/// Sets up panic reporting based on execution environment.
pub fn set_panic_hook() {
    #[cfg(target_arch = "wasm32")]
    {
        // Redirect Rust panics directly to Browser Console Error Stream
        console_error_panic_hook::set_once();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Standard system stderr capture for Native Targets
        std::panic::set_hook(Box::new(|info| {
            eprintln!("[WC_AWF NATIVE PANIC]: {:?}", info);
        }));
    }
}