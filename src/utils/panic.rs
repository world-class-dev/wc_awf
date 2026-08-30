pub fn set_panic_hook() {
    // Standard std panic hook for WASM / native builds
    std::panic::set_hook(Box::new(|info| {
        println!("Panic occurred: {:?}", info);
    }));
}