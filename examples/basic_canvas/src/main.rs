use wc_awf::{Color, WcEngine};

fn main() {
    let mut engine = WcEngine::new("Atsonic Basic Canvas", 1280, 720);
    println!("{}", engine.boot());

    let shapes_rendered = engine.render_frame(|ui| {
        ui.draw_rect(0.0, 0.0, 1280.0, 720.0, Color::DARK_BG);
        ui.draw_rect(100.0, 100.0, 400.0, 200.0, Color::GREEN);
        ui.draw_text("WC_AWF BASIC CANVAS ENGINE", 120.0, 150.0, 20.0, Color::WHITE);
    });

    println!("[+] Total Shapes Vectorized: {}", shapes_rendered);
}
