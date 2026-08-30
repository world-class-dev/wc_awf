#![cfg(feature = "wasm")]

use crate::core::engine::Engine;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct WasmApp {
    engine: Rc<RefCell<Engine>>,
}

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WasmApp, JsValue> {
        let window = web_sys::window().expect("No global window found");
        let document = window.document().expect("No document found");
        let canvas = document
            .get_element_by_id(canvas_id)
            .expect("Canvas element not found")
            .dyn_into::<HtmlCanvasElement>()?;

        let width = canvas.client_width() as u32;
        let height = canvas.client_height() as u32;

        let mut engine = Engine::new("wc_awf", width, height);
        let _ = engine.boot();

        let engine_rc = Rc::new(RefCell::new(engine));

        Ok(WasmApp { engine: engine_rc })
    }

    pub fn start_loop(&self) {
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let engine = self.engine.clone();

        let window = web_sys::window().expect("No window");
        let _performance = window.performance().expect("No performance counter");

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            engine.borrow_mut().render_frame(|_ui| {});

            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));

        window
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }
}
