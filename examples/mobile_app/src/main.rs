use wc_awf::{Color, WcEngine};

fn main() {
    let mut engine = WcEngine::new("Atsonic Mobile App Engine", 1080, 2400);
    println!("{}", engine.boot());

    engine.render_frame(|ui| {
        ui.draw_rect(0.0, 0.0, 1080.0, 2400.0, Color::DARK_BG);
        ui.draw_circle(540.0, 600.0, 150.0, Color::GREEN);
        ui.draw_text("MOBILE NATIVE SURFACE (ANDROID NDK / IOS METAL)", 100.0, 1000.0, 24.0, Color::WHITE);
    });
}
