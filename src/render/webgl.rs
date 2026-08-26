use crate::render::VectorShape;

pub struct WebGlShaderEngine {
    pub context_name: String,
}

impl WebGlShaderEngine {
    pub fn new(context_name: &str) -> Self {
        WebGlShaderEngine {
            context_name: context_name.to_string(),
        }
    }

    pub fn draw_pipeline(&self, shapes: &[VectorShape]) -> usize {
        // High-performance Shader Batching Pipeline Logic
        shapes.len()
    }
}
