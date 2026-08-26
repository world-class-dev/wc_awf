use crate::render::VectorShape;

pub struct NativeGpuPipeline;

impl NativeGpuPipeline {
    pub fn new() -> Self {
        NativeGpuPipeline
    }

    pub fn submit_frame(&self, shapes: &[VectorShape]) {
        for shape in shapes {
     match shape {
                VectorShape::Rectangle { .. } => {
                // badala ya { ... }, weka logic au todo!()
                 todo!() 
                }
                VectorShape::Circle { .. } => {
                 todo!()
                }
                 VectorShape::Text { .. } => {
                 todo!()
                }
                VectorShape::Line { .. } => {
                 todo!()
               }

            }
        }
    }
}
