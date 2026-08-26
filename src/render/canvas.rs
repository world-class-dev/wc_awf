use crate::render::VectorShape;
use web_sys::wasm_bindgen::JsCast;
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

    pub fn render(&self, shapes: &[VectorShape]) {
        if let Some(win) = window() {
            if let Some(document) = win.document() {
                if let Some(element) = document.get_element_by_id(&self.canvas_id) {
                    if let Ok(canvas) = element.dyn_into::<HtmlCanvasElement>() {
                        if let Ok(Some(context_obj)) = canvas.get_context("2d") {
                            if let Ok(ctx) = context_obj.dyn_into::<CanvasRenderingContext2d>() {
                                for shape in shapes {
                                    match shape {
                                        VectorShape::Rectangle { x, y, w, h, color } => {
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
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}