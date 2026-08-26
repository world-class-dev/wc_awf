use wc_awf::prelude::*;

fn main() {
    let mut app = WcApp::new("Atsonic Custom Canvas", 1280, 720);

    // Kila Frame Inachorwa kupitia Vector Engine bila DOM
    let shapes_rendered = app.render_frame(|ctx, events| {
        // Kusikiliza Events (Mouse/Touch/Keyboard)
        for event in events {
            match event {
                WcEvent::PointerDown { x, y } => {
                    println!("[+] Screen Clicked at: x={}, y={}", x, y);
                }
                _ => {}
            }
        }

        // Chora Background & Vector Shapes
        ctx.draw_rect(0.0, 0.0, 1280.0, 720.0, Color::DARK_BG);
        ctx.draw_rect(100.0, 100.0, 400.0, 200.0, Color::GREEN);
        ctx.draw_circle(700.0, 200.0, 80.0, Color::WHITE);
        ctx.draw_line(100.0, 400.0, 1180.0, 400.0, 2.0, Color::GREEN);
        ctx.draw_text("WC_AWF VECTOR ENGINE ACTIVE", 120.0, 150.0, 24.0, Color::DARK_BG);
    });

    println!("{}", app.run());
    println!("[+] Total Vector Shapes Processed: {}", shapes_rendered);
}