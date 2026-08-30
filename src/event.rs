#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    PointerDown { x: f32, y: f32 },
    PointerMove { x: f32, y: f32 },
    PointerUp { x: f32, y: f32 },
    Scroll { delta_x: f32, delta_y: f32 },
    Resize { width: f32, height: f32 },
}

pub struct InputState {
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub is_pressed: bool,
    pub events_queue: Vec<Event>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            cursor_x: 0.0,
            cursor_y: 0.0,
            is_pressed: false,
            events_queue: Vec::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        match &event {
            Event::PointerDown { x, y } => {
                self.cursor_x = *x;
                self.cursor_y = *y;
                self.is_pressed = true;
            }
            Event::PointerMove { x, y } => {
                self.cursor_x = *x;
                self.cursor_y = *y;
            }
            Event::PointerUp { x, y } => {
                self.cursor_x = *x;
                self.cursor_y = *y;
                self.is_pressed = false;
            }
            _ => {}
        }
        self.events_queue.push(event);
    }

    pub fn drain_events(&mut self) -> Vec<Event> {
        self.events_queue.drain(..).collect()
    }
}