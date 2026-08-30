pub fn set_panic_hook() {
    #[cfg(feature = "wasm")]
    {
        // Inawasha panic hook safi ya browser console
        console_error_panic_hook::set_once();
    }
}