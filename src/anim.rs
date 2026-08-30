#[derive(Clone, Debug)]
pub struct Animator {
    pub start: f32,
    pub target: f32,
    pub current: f32,
    pub duration: f32,
    pub elapsed: f32,
}

impl Animator {
    pub fn new(start: f32, target: f32, duration: f32) -> Self {
        Self {
            start,
            target,
            current: start,
            duration,
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.elapsed < self.duration {
            self.elapsed += dt;
            let progress = (self.elapsed / self.duration).clamp(0.0, 1.0);
            // Smooth Ease-Out formula
            let ease_out = 1.0 - (1.0 - progress).powi(3);
            self.current = self.start + (self.target - self.start) * ease_out;
        } else {
            self.current = self.target;
        }
    }
}