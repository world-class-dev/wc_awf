/// Sets up panic reporting based on execution environment.
pub fn set_panic_hook() {
    #[cfg(all(target_arch = "wasm32", feature = "wasm"))]
    {
        console_error_panic_hook::set_once();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::panic::set_hook(Box::new(|info| {
            eprintln!("[WC_AWF NATIVE PANIC]: {:?}", info);
        }));
    }
}