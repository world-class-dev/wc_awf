use wc_awf::{Color, WcEngine};
use wc_awf::security::{E2PassStream, SandboxIsolation};

fn main() {
    let sandbox = SandboxIsolation::new();
    assert!(sandbox.verify_integrity());

    let key = [7u8; 32];
    let stream = E2PassStream::new(&key);
    let payload = b"Atsonic Telemetry Stream Active";
    let encrypted = stream.decrypt_chunk(payload);

    let mut engine = WcEngine::new("Atsonic Enterprise Secure Dashboard", 1920, 1080);
    println!("{}", engine.boot());

    engine.render_frame(|ui| {
        ui.draw_rect(0.0, 0.0, 1920.0, 1080.0, Color::DARK_BG);
        ui.draw_rect(50.0, 50.0, 600.0, 400.0, Color::rgb(20, 20, 20));
        ui.draw_text("E2PASS ENCRYPTED TELEMETRY DASHBOARD", 80.0, 100.0, 22.0, Color::GREEN);
        ui.draw_line(80.0, 120.0, 600.0, 120.0, 2.0, Color::GREEN);
    });

    println!("[+] Secure Dashboard initialized. Encrypted bytes length: {}", encrypted.len());
}
