use crate::render::VectorShape;

#[cfg(feature = "wasm")]
use wasm_bindgen::JsCast;
#[cfg(feature = "wasm")]
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

pub struct CanvasDriver {
    pub canvas_id: String,
}

impl CanvasDriver {
    pub fn new(canvas_id: &str) -> Self {
        CanvasDriver {
            canvas_id: canvas_id.to_string(),
        }
    }

    #[cfg(feature = "wasm")]
    pub fn render(&self, shapes: &[VectorShape]) {
        let win = match window() {
            Some(w) => w,
            None => return,
        };

        let document = match win.document() {
            Some(d) => d,
            None => return,
        };

        let element = match document.get_element_by_id(&self.canvas_id) {
            Some(e) => e,
            None => return,
        };

        let canvas = match element.dyn_into::<HtmlCanvasElement>() {
            Ok(c) => c,
            Err(_) => return,
        };

        let context_obj = match canvas.get_context("2d") {
            Ok(Some(obj)) => obj,
            _ => return,
        };

        let ctx = match context_obj.dyn_into::<CanvasRenderingContext2d>() {
            Ok(c) => c,
            Err(_) => return,
        };

        for shape in shapes {
            match shape {
                VectorShape::Rectangle { x, y, w, h, color, .. } => {
                    let fill_style = format!(
                        "rgba({},{},{},{})",
                        color.r, color.g, color.b, color.a
                    );
                    ctx.set_fill_style_str(&fill_style);
                    ctx.fill_rect(*x as f64, *y as f64, *w as f64, *h as f64);
                }
                VectorShape::Circle { cx, cy, radius, color } => {
                    let fill_style = format!(
                        "rgba({},{},{},{})",
                        color.r, color.g, color.b, color.a
                    );
                    ctx.set_fill_style_str(&fill_style);
                    ctx.begin_path();
                    let _ = ctx.arc(
                        *cx as f64,
                        *cy as f64,
                        *radius as f64,
                        0.0,
                        std::f64::consts::TAU,
                    );
                    ctx.fill();
                }
                VectorShape::Text { body, x, y, size, color } => {
                    let fill_style = format!(
                        "rgba({},{},{},{})",
                        color.r, color.g, color.b, color.a
                    );
                    ctx.set_fill_style_str(&fill_style);
                    ctx.set_font(&format!("{}px sans-serif", size));
                    let _ = ctx.fill_text(body, *x as f64, *y as f64);
                }
            }
        }
    }

    #[cfg(not(feature = "wasm"))]
    pub fn render(&self, shapes: &[VectorShape]) {
        // Native / Headless Engine Fallback
        println!(
            "[WC_AWF ENGINE]: Rendering {} vector shape(s) on target '{}'",
            shapes.len(),
            self.canvas_id
        );
    }
}